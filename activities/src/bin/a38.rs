// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

use std::thread;

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let hello_handle = thread::spawn(move || msg_hello());
    let thread_handle = thread::spawn(move || msg_thread());
    let excited_handle = thread::spawn(move || msg_excited());

    println!("Waiting for threads...");
    match hello_handle.join() {
        Ok(msg) => println!("{:?}", msg),
        Err(e) => println!("hello error: {:?}", e),
    };
    match thread_handle.join() {
        Ok(msg) => println!("{:?}", msg),
        Err(e) => println!("thread error: {:?}", e),
    };
    match excited_handle.join() {
        Ok(msg) => println!("{:?}", msg),
        Err(e) => println!("excited error: {:?}", e),
    };
}
