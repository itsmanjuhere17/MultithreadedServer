use std::thread;
use std::sync::{mpsc,Arc,Mutex};

struct WorkerThread{
    thread: Option<thread::JoinHandle<()>>,
    id:usize
}

impl WorkerThread{
    fn new(id:usize , receiver:Arc<Mutex<mpsc::Receiver<Message>>>)->WorkerThread{
        WorkerThread{
            thread: Some(thread::spawn(move ||loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message{
                    Message::NewJob(job)=>{
                        println!("Worker {} got a job;Executing",id);
                        job();
                    },
                    Message::Terminate=>{
                        println!("Worker {} asked to terminate",id);
                        break;
                    }
                }
            })),
            id
        }
    }
}

//Job here holds the closure function.
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message{
    NewJob(Job),
    Terminate
}

pub struct ThreadPool{
    worker_threads: Vec<WorkerThread>,
    sender: mpsc::Sender<Message> //Here, sender holds the sending side of the channel.
}

impl ThreadPool{
    pub fn new(num:usize)->ThreadPool{
        assert!(num!=0);
        let (sender,receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver)); //We cannot use, RC here as receiver is shared between threads.Mutex is needed here as receiver will be mutated between threads.
        let mut worker_threads = Vec::with_capacity(num);
        for id in 0..num{
            worker_threads.push(WorkerThread::new(id,Arc::clone(&receiver)));
        }
        ThreadPool{
            worker_threads,
            sender
        }
    }

    pub fn execute<F>(&self,execute:F)
     where F:FnOnce()+Send+'static{
        let job = Box::new(execute);
        self.sender.send(Message::NewJob(job)).unwrap(); //Sending the job which is a closure here and is interpreted as trait.
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        for _ in &self.worker_threads{
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all worker threads");
        for worker in &mut self.worker_threads{
            println!("Shutting down worker {}",worker.id);
            if let Some(thread) = worker.thread.take(){ //If not this, way the join() will take ownership of worker thread which cannot be possible. So, Option is encoded.
                thread.join().unwrap();
            }
        }
    }
}