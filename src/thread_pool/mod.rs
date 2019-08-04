//! This module provides various implementations of `ThreadPool` trait.  
pub use naive::NaiveThreadPool;
pub use rayon::RayonThreadPool;
pub use shared_queue::SharedQueueThreadPool;
use crate::Result;

mod naive; 
mod rayon; 
mod shared_queue; 

/// An interface for thread pool creation and execution of tasks.
pub trait ThreadPool { 
    /// Create a new thread pool.
    ///
    /// # Error 
    ///
    /// Return an error if the initialization fails. 
    fn new(threads: u32) -> Result<Self> where Self: Sized;
    
    /// Given a function, execute that function when there are 
    /// idle threads. 
    /// If the function passed panics in the middle, new thread will 
    /// be spawned to keep up with the total number of threads.
    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static;
}
