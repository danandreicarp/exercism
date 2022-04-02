use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    println!("input length is: {}", input.len());
    println!("worker count is: {}", worker_count);

    if worker_count < 2 {
        single_threaded(input)
    } else {
        multi_threaded(input, worker_count)
    }
}

fn single_threaded(input: &[&str]) -> HashMap<char, usize> {
    parse(input.join(""))
}

fn multi_threaded(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input = input.join("");
    println!("entire input length is: {}", input.len());

    let chunk_size = input.len() / worker_count;
    println!("chuck size should be: {}", chunk_size);

    let (sender, receiver) = mpsc::channel();
    let mut handlers = Vec::with_capacity(worker_count);

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
        handlers.push(parse_input_in_thread(text.to_owned(), &sender));
    }

    let result = combine_results(worker_count, receiver);

    for handler in handlers {
        handler.join().expect("error joining threads")
    }

    result
}

fn parse_input_in_thread(input: String, sender: &Sender<HashMap<char, usize>>) -> JoinHandle<()> {
    let thread_sender = sender.clone();

    thread::spawn(move || {
        parse_input_to_sender(input, thread_sender);
    })
}

fn parse_input_to_sender(input: String, sender: Sender<HashMap<char, usize>>) {
    sender.send(parse(input)).expect("send parse result failed");
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

fn combine_results(
    mut expected_results: usize,
    receiver: Receiver<HashMap<char, usize>>,
) -> HashMap<char, usize> {
    let mut char_counts = HashMap::new();

    while expected_results > 0 {
        if let Ok(res) = receiver.try_recv() {
            merge_maps(res, &mut char_counts);

            expected_results -= 1;
        }
    }

    char_counts
}

fn merge_maps(source: HashMap<char, usize>, dest: &mut HashMap<char, usize>) {
    for (&ch, i) in source.iter() {
        *dest.entry(ch).or_insert(0) += i;
    }
}
