use std::thread;
use std::sync::mpsc;

struct WorkerThread{
    thread: thread::JoinHandle<()>,
    id:usize
}

impl WorkerThread{
    fn new(id:usize)->WorkerThread{
        WorkerThread{
            thread:thread::spawn(||{}),
            id
        }
    }
}

struct Job; //Job here holds the closure function.

pub struct ThreadPool{
    worker_threads: Vec<WorkerThread>,
    sender: mpsc::Sender<Job> //Here, sender holds the sending side of the channel.
}

impl ThreadPool{
    pub fn new(num:usize)->ThreadPool{
        assert!(num==0);
        let (sender,receiver) = mpsc::channel();
        let mut worker_threads = Vec::with_capacity(num);
        for id in 0..num{
            worker_threads.push(WorkerThread::new(id));
        }
        ThreadPool{
            worker_threads,
            sender
        }
    }

    pub fn execute<F>(&self,execute:F)
     where F:FnOnce()+Send+'static{

    }
}