
use crossbeam::channel;

use super::ThreadPool;
use crate::{Result};

/// shared 
pub struct SharedQueueThreadPool;

impl ThreadPool for SharedQueueThreadPool { 
    fn new(threads: u32) -> Result<Self> { 
        unimplemented!()
    }

    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static { 
        unimplemented!()
    }
}
