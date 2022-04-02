use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("poisoned mutex: another thread panicked")
                .recv()
                .expect("thread holding sending side of channel (ThreadPool) was shut down");

            match message {
                Message::NewJob(job) => {
                    // worker got a job; executing...
                    job();
                }
                Message::Terminate => {
                    // terminating worker
                    break;
                }
            }
        });

        Worker {
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(worker_count: usize) -> ThreadPool {
        assert!(worker_count > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(worker_count);
        for _ in 0..worker_count {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .send(Message::NewJob(job))
            .expect("send Job message failed in ThreadPool");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // sending terminate message to all workers
        for _ in &self.workers {
            self.sender
                .send(Message::Terminate)
                .expect("send Terminate message failed in ThreadPool");
        }

        // shutting down all workers
        for worker in &mut self.workers {
            // shutting down worker
            if let Some(thread) = worker.thread.take() {
                thread.join().expect("join failed on worker thread");
            }
        }
    }
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result = HashMap::new();

    let pool = ThreadPool::new(worker_count);

    let (sender, receiver) = mpsc::channel();

    let mut expected_results = input.len();
    for line in input.iter() {
        let thread_sender = sender.clone();
        let thread_line = line.to_owned().to_owned();
        pool.execute(move || {
            parse_line(thread_line, thread_sender);
        });

        if read_from_channel(&mut result, &receiver).is_ok() {
            expected_results -= 1;
        }
    }

    while expected_results > 0 {
        if read_from_channel(&mut result, &receiver).is_ok() {
            expected_results -= 1;
        }
    }

    result
}

fn parse_line(line: String, sender: Sender<HashMap<char, usize>>) {
    let mut thread_result = HashMap::new();
    let line = line.to_lowercase();
    for ch in line.chars() {
        if ch.is_alphabetic() {
            *thread_result.entry(ch).or_insert(0) += 1;
        }
    }
    sender
        .send(thread_result)
        .expect("send parse_line message failed");
}

fn read_from_channel(
    result: &mut HashMap<char, usize>,
    receiver: &Receiver<HashMap<char, usize>>,
) -> Result<bool, TryRecvError> {
    return match receiver.try_recv() {
        Ok(res) => {
            for (&ch, i) in res.iter() {
                let count = result.entry(ch).or_insert(0);
                *count += i;
            }
            Ok(true)
        }
        Err(error) => Err(error),
    };
}
