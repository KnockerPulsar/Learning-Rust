use std::thread;
use std::sync::{Arc, Mutex, mpsc};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Option<mpsc::Sender<Job>>
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);


        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let tx = Some(tx);

        ThreadPool {
            tx,
            workers: (0..size).map(|id| Worker::new(id, Arc::clone(&rx))).collect()
        }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static        
    {
        let job = Box::new(f);
        self.tx.as_ref().unwrap().send(job).unwrap();
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
    id: usize
}

impl Worker {
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            match rx.lock().unwrap().recv() {
                Ok(job) => {  
                    println!("Worker {id}, got a job; executing.");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down");
                    break;
                }
            };
        });
        
        let thread = Some(thread);
        Worker { id, thread }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        std::mem::drop(self.tx.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

