use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

enum Massage {
    NewJob(Job),
    Terminate,
}
pub struct ThreadPool {
    worlers: Vec<Worker>,
    sender: mpsc::Sender<Massage>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut worlers = Vec::with_capacity(size);

        for id in 0..size {
            worlers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { worlers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Massage::NewJob(job)).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.worlers {
            self.sender.send(Massage::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.worlers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Massage>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let massage = receiver.lock().unwrap().recv().unwrap();

            match massage {
                Massage::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Massage::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
