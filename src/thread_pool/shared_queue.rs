use crossbeam::crossbeam_channel::unbounded; 
use crossbeam::channel::{Sender, Receiver};

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

use super::ThreadPool;
use crate::{Result};

// question: use this to signal shutdown or just drop the channel?
enum ThreadMessage { 
    Job(Box<dyn FnOnce() + Send + 'static>),
    Shutdown,
}

// Spawns a new thread if
struct ThreadGuard {
    shutdown: bool,
    threads_count: Arc<AtomicUsize>,
    spawn_count: Arc<AtomicUsize>,
}

impl ThreadGuard { 
    fn new(threads_count: Arc<AtomicUsize>, spawn_count: Arc<AtomicUsize>) -> Self { 
        threads_count.fetch_add(1, Ordering::SeqCst);
        spawn_count.fetch_sub(1, Ordering::SeqCst);
        ThreadGuard { 
            shutdown: false, 
            threads_count,
            spawn_count,
        }
    }

    fn shutdown(&mut self) { 
        self.shutdown = true;
    }
}

impl Drop for ThreadGuard { 
    fn drop(&mut self) {
        if !self.shutdown {
            self.spawn_count.fetch_add(1, Ordering::SeqCst);
            self.threads_count.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

/// shared 
pub struct SharedQueueThreadPool { 
    sender: Arc<Sender<ThreadMessage>>,
    receiver: Receiver<ThreadMessage>,
    threads_count: Arc<AtomicUsize>, // number of active threads
    spawn_count: Arc<AtomicUsize>, // number of threads to spawn 
}

impl ThreadPool for SharedQueueThreadPool { 
    fn new(n_threads: u32) -> Result<Self> { 
        assert!(n_threads > 0);

        let (s, r) = unbounded();
        let s = Arc::new(s);
        let threads_count = Arc::new(AtomicUsize::new(0));
        let spawn_count = Arc::new(AtomicUsize::new(n_threads as usize));

        let pool = SharedQueueThreadPool { 
            sender: s,
            receiver: r,
            threads_count,
            spawn_count,
        };

        for _ in 0..n_threads {
            pool.spawn_new_thread();
        }

        Ok(pool)
    }

    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static { 
        if self.spawn_count.load(Ordering::SeqCst) != 0 { 
            let n = self.spawn_count.load(Ordering::SeqCst);
            for _ in 0..n { 
                self.spawn_new_thread();
            }
        }

        self.sender.send(ThreadMessage::Job(Box::new(job))).unwrap();
    }

}

impl SharedQueueThreadPool { 
    fn spawn_new_thread(&self) {
        let receiver = self.receiver.clone();
        let t_count = self.threads_count.clone();
        let s_count = self.spawn_count.clone();

        thread::Builder::new().spawn(move || {
            let mut guard = ThreadGuard::new(t_count, s_count);
            loop { 
                // handle panics 
                
                match receiver.recv() {
                    Ok(ThreadMessage::Job(f)) => f(),
                    Ok(ThreadMessage::Shutdown) => break,
                    Err(e) => eprintln!("{}", e),
                }
            }

            guard.shutdown();
        }).unwrap();
    }
}

impl Drop for SharedQueueThreadPool { 
    fn drop(&mut self) {
        let n = self.threads_count.load(Ordering::SeqCst);
        for _ in 0..n {
            self.sender.send(ThreadMessage::Shutdown).unwrap();
        }
    }
}
