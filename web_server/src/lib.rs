use std::thread;

pub struct ThreadPool{
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    //we’ll use this 4 threads as the number of elements in a collection of threads, which is what the usize type is for
    //because function panics its a good idea to use some documentaion 
    ///Creates a new Thread pool of usize == size.
    ///The size is the number of threads in the pool
    /// 
    /// #Panics 
    /// 
    /// The new function panics is the size is zero 
    pub fn new(size: usize) -> ThreadPool{
        //validate that usize > 0 
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size{
            //create threads
        }

        ThreadPool {threads}
    }

    pub fn execute<F>(&self,f:F)
    where
        F: FnOnce() + Send + 'static
    {
         
    }
}