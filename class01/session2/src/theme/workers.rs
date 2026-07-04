// IKinder

use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Command {
    Run(Job),
    Quit,
}

pub fn main() {
    crate::show_name(file!());

    let (tx, rx) = mpsc::channel::<Command>();
    let handle = std::thread::spawn(move || {
        'l: while let Ok(command) = rx.recv() {
            match command {
                Command::Run(job) => job(),
                Command::Quit => break 'l,
            }
        }
    });

    let job1 = || println!("Hello from a closure");
    let job2 = || {
        for i in 0..10 {
            print!("{i} ")
        }
        println!();
    };

    tx.send(Command::Run(Box::new(job1))).unwrap();
    tx.send(Command::Run(Box::new(job2))).unwrap();
    tx.send(Command::Run(Box::new(hi_there))).unwrap();
    tx.send(Command::Run(Box::new(|| println!("I'm in a box"))))
        .unwrap();
    tx.send(Command::Quit).unwrap();

    handle.join().unwrap();
}

fn hi_there() {
    println!("hi there");
}
