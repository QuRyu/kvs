use rayon;

use super::ThreadPool;
use crate::Result;

/// rayon
pub struct RayonThreadPool(rayon::ThreadPool);

impl ThreadPool for RayonThreadPool {
    fn new(n_threads: u32) -> Result<Self> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(n_threads as usize)
            .build()?;

        Ok(RayonThreadPool(pool))
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.0.spawn(job);
    }
}
