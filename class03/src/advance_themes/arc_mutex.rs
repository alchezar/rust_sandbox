// IKinder

#![allow(clippy::collapsible_if, clippy::useless_conversion)]

// -----------------------------------------------------------------------------
// Arc/Mutex: Intro
// -----------------------------------------------------------------------------

pub fn intro() {
    use parking_lot::Mutex;
    use std::sync::Arc;
    use std::thread;

    struct Counter(usize);

    let counter = Counter(0);
    let shared_counter = Arc::new(Mutex::new(counter));

    let thread1_counter = Arc::clone(&shared_counter);
    let thread2_counter = shared_counter.clone();

    let thread1 = thread::spawn(move || {
        let mut counter = thread1_counter.lock();
        counter.0 += 1;
    });
    let thread2 = thread::spawn(move || {
        let mut counter = thread2_counter.lock();
        counter.0 += 1;
    });

    let result = thread1.join().and_then(|_| thread2.join());
    println!("{}", shared_counter.lock().0)
}

// -----------------------------------------------------------------------------
// Threading: Deadlocks: Intro
// -----------------------------------------------------------------------------

use parking_lot::ReentrantMutex;
use std::rc::Rc;

fn recurse(data: Rc<ReentrantMutex<u32>>, remaining: usize) -> usize {
    let mut locked = data.lock();
    match remaining {
        rem @ 0 => 0,
        rem => recurse(Rc::clone(&data), rem - 1),
    }
}

struct Account {
    balance: i64,
}

type ArcAccount = Arc<Mutex<Account>>;

fn transfer1(from: ArcAccount, to: ArcAccount, amount: i64) {
    let mut from = from.lock();
    let mut to = to.lock();
    from.balance -= amount;
    to.balance += amount;
}
fn transfer2(from: ArcAccount, to: ArcAccount, amount: i64) {
    loop {
        if let Some(mut from) = from.try_lock() {
            if let Some(mut to) = to.try_lock() {
                from.balance -= amount;
                to.balance += amount;
                return;
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}
fn transfer3(from: ArcAccount, to: ArcAccount, amount: i64) {
    let test = from.try_lock();
    let op = || {
        if let Some(mut from) = from.try_lock() {
            if let Some(mut to) = to.try_lock() {
                from.balance -= amount;
                to.balance += amount;
                return Ok(());
            }
        }
        Err(0)?
    };
    let backoff = backoff::ExponentialBackoff::default();
    backoff::retry(backoff, op).unwrap();
}

pub fn intro1() {
    use parking_lot::Mutex;
    use std::sync::Arc;
    use std::thread;

    let a = Arc::new(Mutex::new(Account { balance: 1000 }));
    let b = Arc::new(Mutex::new(Account { balance: 500 }));
    let a1 = a.clone();
    let b1 = b.clone();

    let t1 = thread::spawn(move || {
        transfer1(a1, b1, 500);
    });
    let t2 = thread::spawn(move || transfer1(b, a, 800));
}

// -----------------------------------------------------------------------------
// Arc/Mutex: Demo
// -----------------------------------------------------------------------------

use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

type SharedSignData = Arc<Mutex<String>>;

struct DigitalSignBoard {
    display: SharedSignData,
}

impl DigitalSignBoard {
    fn update(&self) {
        let data = self.display.lock();
        println!("sign data: {}", data)
    }
}

fn spawn_display_thread(display_data: SharedSignData) {
    thread::spawn(move || {
        let board = DigitalSignBoard {
            display: display_data,
        };
        loop {
            board.update();
            thread::sleep(Duration::from_millis(200));
        }
    });
}

fn change_data(display_data: SharedSignData, new_data: &str) {
    let mut data = display_data.lock();
    *data = new_data.to_owned();
    println!("-----updated: {}", new_data);
}

pub fn demo() {
    let display_data = Arc::new(Mutex::new(String::from("init".to_owned())));
    spawn_display_thread(Arc::clone(&display_data));

    thread::sleep(Duration::from_millis(100));
    change_data(Arc::clone(&display_data), "message1");
    thread::sleep(Duration::from_millis(600));
    change_data(Arc::clone(&display_data), "another message");
    thread::sleep(Duration::from_millis(600));
    change_data(Arc::clone(&display_data), "goodbye");
    thread::sleep(Duration::from_millis(600));
}

// -----------------------------------------------------------------------------
// Arc/Mutex: Activity
// -----------------------------------------------------------------------------

// Topic: Arc, Mutex, and Threads
//
// Summary:
// Modify the existing multithreaded program to include a global
// counter shared among the threads. The counter should increase
// by 1 whenever a worker completes a job.
//
// Requirements:
// * 1. The total number of jobs completed must be displayed
//      at the end of the program.
// * 2. Use Arc & Mutex to share the total count among threads.
//      * Arc is in the standard library
//      * Mutex is in the parking_lot crate
//
// Notes:
// * Ensure following crates are added to your Cargo.toml file:
//   - crossbeam-channel
//   - parking_lot

use crossbeam_channel::{Receiver, Sender, unbounded};
use std::collections::VecDeque;
use std::thread::JoinHandle;
// use std::time::Duration;

/// Job given to workers.
#[derive(Clone)]
enum Job {
    Print(String),
    Sum(isize, isize),
}

/// Message sent to workers.
#[derive(Clone)]
enum Message {
    AddJob(Job),
    Quit,
}

struct Worker<M> {
    transmitter: Sender<M>,
    receiver: Receiver<M>,
    handle: JoinHandle<()>,
}

impl Worker<Message> {
    fn add_job(&self, job: Job) {
        self.transmitter
            .send(Message::AddJob(job))
            .expect("failed to add job")
    }
    fn join(self) {
        self.handle.join().expect("failed to send message")
    }
    fn send_msg(&self, msg: Message) {
        self.transmitter.send(msg).expect("failed to send message");
    }
}

fn consume_available_jobs(counter: Arc<Mutex<usize>>, jobs: &mut VecDeque<Job>) {
    while let Some(job) = jobs.pop_front() {
        match job {
            Job::Print(msg) => println!("{}", msg),
            Job::Sum(lhs, rhs) => println!("{}+{}={}", lhs, rhs, lhs + rhs),
        }
        let mut counter = counter.lock();
        *counter += 1;
    }
}

fn job_process(counter: Arc<Mutex<usize>>, receiver_thread: Receiver<Message>) {
    // VecDeque allows us to get jobs in the order they arrive.
    let mut jobs = VecDeque::new();

    // Outer loop is so we can have a brief delay when no jobs are available.
    loop {
        // Inner loop continuously processes jobs until no more are available.
        loop {
            // Get the next job.
            consume_available_jobs(Arc::clone(&counter), &mut jobs);

            // Check for messages on the channel.
            if let Ok(msg) = receiver_thread.try_recv() {
                match msg {
                    Message::AddJob(job) => {
                        // When we receive a new job, add it to the jobs list.
                        jobs.push_back(job);
                        continue;
                    }
                    Message::Quit => return,
                }
            } else {
                // No messages on the channel, break from inner loop
                // and thread will wait momentarily for more messages.
                break;
            }
        }
        // Pause to wait for more messages to arrive at channel.
        thread::sleep(Duration::from_millis(100));
    }
}

/// Create a new worker to receive jobs.
fn spawn_worker(counter: Arc<Mutex<usize>>) -> Worker<Message> {
    let (transmitter, receiver) = unbounded();
    // We clone the receiving end there so we have a copy to give to the thread.
    // THis allows us to save the `transmitter` and `receiver` into the Worker struct.
    let receiver_thread = receiver.clone();
    // Spawn a new thread.
    let handle = thread::spawn(move || {
        job_process(counter, receiver_thread);
    });

    Worker {
        transmitter,
        receiver,
        handle,
    }
}

pub fn activity() {
    let jobs = vec![
        Job::Print("hello".to_owned()),
        Job::Sum(2, 2),
        Job::Print("world".to_owned()),
        Job::Sum(4, 4),
        Job::Print("two words".to_owned()),
        Job::Sum(1, 1),
        Job::Print("a print job".to_owned()),
        Job::Sum(10, 10),
        Job::Print("message".to_owned()),
        Job::Sum(3, 4),
        Job::Print("thread".to_owned()),
        Job::Sum(9, 8),
        Job::Print("rust".to_owned()),
        Job::Sum(1, 2),
        Job::Print("compiler".to_owned()),
        Job::Sum(9, 1),
    ];

    let job_counter = Arc::new(Mutex::new(0));
    let jobs_sent = jobs.len();
    let mut workers = Vec::new();
    // Spawn 4 workers to process jobs.
    for _ in 0..4 {
        let worker = spawn_worker(Arc::clone(&job_counter));
        workers.push(worker);
    }

    // Create an iterator that cycles through each worker endlessly.
    let mut worker_ring = workers.iter().cycle();
    for job in jobs {
        // Get next worker.
        let worker = worker_ring.next().expect("failed to get worker");
        // Add the job.
        worker.add_job(job);
    }

    // Ask all workers to quit.
    for worker in &workers {
        worker.send_msg(Message::Quit);
    }

    // Wait for workers to terminate.
    for worker in workers {
        worker.join();
    }
    println!("Jobs sent: {}", jobs_sent);

    // Print out the number of jobs completed here.
    println!("Jobs received: {}", job_counter.lock());
}
