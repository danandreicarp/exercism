use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    println!("input length is: {}", input.len());
    println!("worker count is: {}", worker_count);

    let input = input.join("");

    if worker_count < 2 || input.len() < 100 {
        single_threaded(input)
    } else {
        multi_threaded_shared(input, worker_count)
    }
}

fn multi_threaded_shared(input: String, worker_count: usize) -> HashMap<char, usize> {
    let chunk_size = input.len() / worker_count;

    let mut handlers = Vec::with_capacity(worker_count);

    let char_count: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut chunk = 1;
    for i in 0..worker_count {
        let start = i * chunk_size;
        let end = if chunk != worker_count {
            chunk_size * chunk
        } else {
            // on the last chunk, go to the end of the input
            input.len()
        };

        let text = &input[start..end];
        chunk += 1;

        println!(
            "selected a text of length {}, from {} up to {}",
            text.len(),
            start,
            end
        );

        println!("spawning thread number {}", i);
        handlers.push(parse_and_merge(text.to_owned(), char_count.clone()));
    }

    for handler in handlers {
        handler.join().expect("error joining threads")
    }

    let result = char_count.lock().unwrap();
    result.deref().to_owned()
}

fn parse_and_merge(input: String, result: Arc<Mutex<HashMap<char, usize>>>) -> JoinHandle<()> {
    thread::spawn(move || {
        let char_count = parse(input);

        for (&ch, i) in char_count.iter() {
            *result.lock().unwrap().entry(ch).or_insert(0) += i;
        }
    })
}

fn single_threaded(input: String) -> HashMap<char, usize> {
    parse(input)
}

fn parse(input: String) -> HashMap<char, usize> {
    let mut char_count = HashMap::new();

    for ch in input.chars() {
        if ch.is_alphabetic() {
            if ch.is_lowercase() {
                *char_count.entry(ch).or_insert(0) += 1;
            } else {
                *char_count
                    .entry(ch.to_lowercase().next().unwrap())
                    .or_insert(0) += 1;
            }
        }
    }

    char_count
}
