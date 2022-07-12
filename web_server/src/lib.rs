pub struct ThreadPool; 

impl ThreadPool {
    //we’ll use this 4 threads as the number of elements in a collection of threads, which is what the usize type is for
    pub fn new(size: usize) -> ThreadPool{
        ThreadPool 
    }

    pub fn execute<F>(&self,f:F)
    where
        F: FnOnce() + Send + 'static
    {
         
    }
}