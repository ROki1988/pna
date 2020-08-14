use crate::Result;
use crossbeam::{Receiver, Sender};

/// kvs ThreadPool
pub trait ThreadPool {
    /// create new thread pool
    fn new(size: u32) -> Result<Self>
    where
        Self: Sized;

    /// spawn task to thread pool
    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static;

    /// shutdown threads
    fn shutdown(&self);
}

/// pool native thead
pub struct NaiveThreadPool;

impl ThreadPool for NaiveThreadPool {
    fn new(_size: u32) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self)
    }

    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        std::thread::spawn(|| f());
    }

    fn shutdown(&self) {}
}

enum Message {
    Job(Box<dyn FnOnce() + Send + 'static>),
    Terminate,
}

/// pool shared queue
pub struct SharedQueueThreadPool {
    size: u32,
    sender: Sender<Message>,
}

#[derive(Clone)]
struct SharedQueueWorker {
    receiver: Receiver<Message>,
}

impl Drop for SharedQueueWorker {
    fn drop(&mut self) {
        if std::thread::panicking() {
            let rx = self.clone();
            if let Err(_) = std::thread::Builder::new().spawn(move || run_tasks(rx)) {}
        }
    }
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(size: u32) -> Result<Self>
    where
        Self: Sized,
    {
        let (tx, rx) = crossbeam::channel::unbounded::<Message>();

        for _ in 0..size {
            let worker = SharedQueueWorker {
                receiver: rx.clone(),
            };
            std::thread::Builder::new().spawn(move || run_tasks(worker))?;
        }
        Ok(Self { size, sender: tx })
    }

    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Message::Job(Box::new(f))).unwrap();
    }

    fn shutdown(&self) {
        for _ in 0..self.size {
            self.sender.send(Message::Terminate).unwrap();
        }
    }
}

fn run_tasks(rx: SharedQueueWorker) {
    loop {
        match rx.receiver.recv() {
            Ok(Message::Job(f)) => {
                f();
            }
            Ok(Message::Terminate) => {
                break;
            }
            Err(_) => {}
        }
    }
}

/// use rayon
pub struct RayonThreadPool {
    pool: rayon::ThreadPool,
}

impl ThreadPool for RayonThreadPool {
    fn new(size: u32) -> Result<Self>
    where
        Self: Sized,
    {
        rayon::ThreadPoolBuilder::new()
            .num_threads(size as usize)
            .build()
            .map_err(Into::into)
            .map(|x| Self { pool: x })
    }

    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.pool.spawn(f);
    }

    fn shutdown(&self) {}
}
