use std::thread;

pub struct ThreadPool{
    //closures here don't return anything and just handle the connection
    workers: Vec<Worker>,
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
        //
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            //create workers
            workers.push(Worker::new(id)); 
        }

        ThreadPool {workers}
    }

    pub fn execute<F>(&self,f:F)
    where
        F: FnOnce() + Send + 'static
    {
         
    }
}
///
/// Thread::spawn expects to get some code and run immediately. 
/// Here we want to create the threads and give the code later. 
struct Worker {
    id: usize, 
    handle:thread::JoinHandle<()>, 
}

impl Worker {
    fn new(id: usize) -> Worker{
        let handle = thread::spawn(||{}); 
        Worker{id,handle} 
    }
}