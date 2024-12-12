use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool<T> {
    workers: Vec<Worker<T>>,
    sender: Option<mpsc::Sender<Job<T>>>,
    my_receiver: Option<mpsc::Receiver<(usize,T)>>,
}

type Job<T> = Box<dyn FnOnce() -> T + Send + 'static>;

impl<T: std::marker::Send + 'static> ThreadPool<T> {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool<T> {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let (returns, my_receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let worker_sender = returns.clone();
            workers.push(Worker::new(id, Arc::clone(&receiver), worker_sender));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
            my_receiver: Some(my_receiver),
        }
    }

    pub fn execute<F>(&self, function: F)
    where
        F: FnOnce() -> T + Send + 'static,
    {
        let job = Box::new(function);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    pub fn await_returns(&self, amount: usize) -> Vec<(usize,T)> {
        let mut returns = Vec::new();
        for _i in 0..amount {
            let message = self.my_receiver.as_ref()
                .unwrap()
                .recv()
                .unwrap();

            returns.push(message);
        }
        returns
    }
}

impl<T> Drop for ThreadPool<T> {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker<T> {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: std::marker::Send + 'static> Worker<T> {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job<T>>>>, sender: mpsc::Sender<(usize, T)>) -> Worker<T> {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("Poisoned mutex. Killing worker! :D")
                .recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    let job_res = job();
                    let _ = sender.send((id, job_res));
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
            _marker: std::marker::PhantomData
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_job() {
        fn manipulate_input(val: i32) -> i32 {
            val*2
        }

        let tp = ThreadPool::new(6);
        let input = [1,2,3,4,5];
        
        for val in input {
            tp.execute(move || {
                manipulate_input(val)
            });
        }

        let returns = tp.await_returns(input.len());

        println!("returns = {returns:?}");
    }
}
