use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool{
    //closures here don't return anything and just handle the connection
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
///
/// This will hold the closures we'll send to the channel 
/// We use a trait object here so all acceptable jobs can be passed 
type Job = Box<dyn FnOnce() + Send + 'static>;

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

        let (sender,receiver) = mpsc::channel(); 
        let receiver = Arc::new(Mutex::new(receiver)); 
        
        
        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            //create workers
            //we want shared ownership and mutability for the receiver 
            //clone doesn't actually copy the data but increases the reference count
            workers.push(Worker::new(id,Arc::clone(&receiver))); 
        }

        ThreadPool {workers,sender}
    }

    pub fn execute<F>(&self,f:F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f); 
        self.sender.send(job).unwrap(); 
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let handle = thread::spawn(move || loop{
            let job = receiver.lock().unwrap().recv().unwrap(); 

            println!("Worker {} got a job; executing",id); 

            job(); 
        }); 
        Worker{id,handle} 
    }
}