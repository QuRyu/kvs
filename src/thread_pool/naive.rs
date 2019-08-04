use crate::Result; 

use super::ThreadPool;

/// Naive thread pool that spawns new threads when then 
/// `spawn` method is called. 
pub struct NaiveThreadPool; 

impl ThreadPool for NaiveThreadPool { 
    fn new(_threads: u32) -> Result<Self> { 
        Ok(NaiveThreadPool)
    }

    fn spawn<F>(&self, job: F) 
        where F: FnOnce() + Send + 'static 
    {
        std::thread::spawn(job);
    }

}
