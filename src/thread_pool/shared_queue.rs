use crossbeam::channel::{Receiver, Sender};
use crossbeam::crossbeam_channel::unbounded;

use std::sync::Arc;
use std::thread;

use super::ThreadPool;
use crate::Result;

// question: use this to signal shutdown or just drop the channel?
enum ThreadMessage {
    Job(Box<dyn FnOnce() + Send + 'static>),
    Shutdown,
}

// Spawns a new thread if
#[derive(Clone)]
struct ThreadGuard {
    receiver: Receiver<ThreadMessage>,
}

impl ThreadGuard {
    fn new(receiver: Receiver<ThreadMessage>) -> Self {
        ThreadGuard { receiver }
    }
}

impl Drop for ThreadGuard {
    fn drop(&mut self) {
        if thread::panicking() {
            let r = self.clone();
            if let Err(e) = thread::Builder::new().spawn(move || run_task(r)) {
                eprintln!("Fail to spawn new thread {}", e);
            }
        }
    }
}

/// shared
pub struct SharedQueueThreadPool {
    n_threads: u32,
    sender: Arc<Sender<ThreadMessage>>,
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(n_threads: u32) -> Result<Self> {
        assert!(n_threads > 0);

        let (s, r) = unbounded();
        let s = Arc::new(s);

        for _ in 0..n_threads {
            let guard = ThreadGuard::new(r.clone());
            thread::Builder::new()
                .spawn(move || {
                    run_task(guard);
                })
                .unwrap();
        }

        Ok(SharedQueueThreadPool {
            n_threads,
            sender: s,
        })
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(ThreadMessage::Job(Box::new(job))).unwrap();
    }
}

impl Drop for SharedQueueThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.n_threads {
            self.sender.send(ThreadMessage::Shutdown).unwrap();
        }
    }
}

fn run_task(t: ThreadGuard) {
    loop {
        match t.receiver.recv() {
            Ok(ThreadMessage::Job(task)) => task(),
            Ok(ThreadMessage::Shutdown) => break,
            Err(e) => eprintln!("Err on receiver: {}", e),
        }
    }
}
