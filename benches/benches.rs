use std::rc::Rc;

use kvs::{KvStore, KvsEngine, SledKvsEngine};

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::{thread_rng, Rng};
use std::process::Command;
use tempfile::TempDir;

fn rand_string<R: Rng>(rng: &mut R) -> String {
    let len: usize = rng.gen::<usize>() % 100000;

    let mut string = String::with_capacity(len);
    for _ in 0..len {
        string.push(rng.gen());
    }

    string
}

fn write<S: AsRef<str>, E: KvsEngine + 'static>(name: S, c: &mut Criterion, mut engine: E) {
    let mut rng = thread_rng();

    // populate test data
    let mut inputs = Vec::new();
    for _ in 0..100 {
        let key = rand_string(&mut rng);
        let value = rand_string(&mut rng);
        inputs.push((key, value));
    }

    inputs.shrink_to_fit();

    c.bench_function(name.as_ref(), move |b| {
        b.iter_batched(
            || inputs.clone(),
            |data| {
                for (k, v) in data {
                    engine.set(k, v).unwrap();
                }
            },
            BatchSize::SmallInput,
        )
    });
}

fn kvs_write(_: &mut Criterion) {
    let mut c: Criterion = Default::default();
    c = c.sample_size(20);

    let temp_dir = TempDir::new().unwrap();
    let engine = KvStore::open(temp_dir.path()).unwrap();

    write("kvs_write", &mut c, engine);
}

fn sled_write(_: &mut Criterion) {
    let mut c: Criterion = Default::default();
    c = c.sample_size(20);
    let temp_dir = TempDir::new().unwrap();
    let engine = SledKvsEngine::new(temp_dir.path()).unwrap();

    write("sled_write", &mut c, engine);
}

criterion_group!(benches, sled_write, kvs_write);
criterion_main!(benches);
