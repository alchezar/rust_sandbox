// IKinder

// -----------------------------------------------------------------------------
// Channels: Intro
// -----------------------------------------------------------------------------

pub fn intro1() {
    let (sender, receiver) = crossbeam_channel::unbounded();
    sender.send("Hello, channel!").ok();

    match receiver.recv() {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("{:?}", e),
    }
}

pub fn intro2() {
    let (sender, receiver) = crossbeam_channel::unbounded();
    let handle = std::thread::spawn(move || match receiver.recv() {
        Ok(msg) => println!("Thread: {}", msg),
        Err(e) => println!("{:?}", e),
    });
    sender.send("Hello from main").ok();
    handle.join().ok();
}

pub fn intro3() {
    let (sender, receiver1) = crossbeam_channel::unbounded();
    let receiver2 = receiver1.clone();

    let handle1 = std::thread::spawn(move || match receiver1.recv() {
        Ok(msg) => println!("Thread1: {}", msg),
        Err(e) => println!("{:?}", e),
    });
    let handle2 = std::thread::spawn(move || match receiver2.recv() {
        Ok(msg) => println!("Thread2: {}", msg),
        Err(e) => println!("{:?}", e),
    });

    sender.send("Hello from main 2").ok();
    sender.send("Hello from main 1").ok();
    handle1.join().ok();
    handle2.join().ok();
}

// -----------------------------------------------------------------------------
// Channels: Demo 1
// -----------------------------------------------------------------------------

enum ThreadMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

pub fn demo1() {
    let (sender, receiver) = crossbeam_channel::unbounded();

    let handle = std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            match receiver.recv() {
                Ok(msg) => match msg {
                    ThreadMsg::PrintData(d) => println!("{}", d),
                    ThreadMsg::Sum(lhs, rhs) => println!("{}+{}={}", lhs, rhs, lhs + rhs),
                    ThreadMsg::Quit => {
                        println!("Thread terminating");
                        break;
                    }
                },
                Err(e) => {
                    println!("Disconnected: {:?}", e);
                    break;
                }
            }
        }
    });

    sender
        .send(ThreadMsg::PrintData("Hello from main".to_owned()))
        .ok();
    sender.send(ThreadMsg::Sum(1, 2)).ok();
    sender.send(ThreadMsg::Quit).ok();
    // drop(sender);

    handle.join().ok();
}

// -----------------------------------------------------------------------------
// Channels: Demo 2
// -----------------------------------------------------------------------------

enum WorkerMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

enum MainMsg {
    SumResult(i64),
    WorkerQuit,
}

pub fn demo2() {
    let (worker_transmit, worker_receiver) = crossbeam_channel::unbounded();
    let (main_transmit, main_receiver) = crossbeam_channel::unbounded();

    let worker = std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            match worker_receiver.recv() {
                Ok(msg) => match msg {
                    WorkerMsg::PrintData(d) => println!("Worker: {}", d),
                    WorkerMsg::Sum(lhs, rhs) => {
                        println!("Worker: {}+{}={}", lhs, rhs, lhs + rhs);
                        main_transmit.send(MainMsg::SumResult(lhs + rhs)).unwrap()
                    }
                    WorkerMsg::Quit => {
                        println!("Worker: Thread terminating");
                        break;
                    }
                },
                Err(e) => {
                    println!("Worker: Disconnected: {:?}", e);
                    main_transmit.try_send(MainMsg::WorkerQuit).ok();
                    break;
                }
            }
        }
    });

    worker_transmit
        .send(WorkerMsg::PrintData("Main: Hello from main".to_owned()))
        .ok();
    worker_transmit.send(WorkerMsg::Sum(1, 2)).ok();
    drop(worker_transmit);

    while let Ok(msg) = main_receiver.recv() {
        match msg {
            MainMsg::SumResult(result) => println!("Main: {}", result),
            MainMsg::WorkerQuit => println!("Main: Worker quit"),
        }
    }

    worker.join().ok();
}

// -----------------------------------------------------------------------------
// Channels: Activity
// -----------------------------------------------------------------------------

// Topic: Channels
//
// Summary:
//   Using the existing code, create a program that simulates an internet-of-things
//   remote control light bulb. The color of the light can be changed remotely.
//   Use threads and channels to communicate what color the light bulb should display.
//
// Requirements:
// * Create a separate thread representing the light bulb
// * Use a channel to communicate with the thread
// * Display a color change message using the println! macro
// * The light bulb must also be able to turn on and off
//   * Display whether the light is on or off on each color change
// * Turn off the light when disconnecting from it
//
// Notes:
// * Remember to add `crossbeam-channel` to your Cargo.toml file
// * Use the `colored` crate if you want to get fancy and display actual colors
// * The docs.rs site can be used to read documentation for third-party crates
// * Disconnection can be accomplished by dropping the sender, or
//   by telling the thread to self-terminate

// use crossbeam_channel::{unbounded, Receiver};
// use std::thread::{self, JoinHandle};

enum LightMsg {
    ChangeColor(u8, u8, u8),
    Disconnect,
    Off,
    On,
}

enum LightStatus {
    Off,
    On,
}

fn spawn_light_thread(
    receiver: crossbeam_channel::Receiver<LightMsg>,
) -> std::thread::JoinHandle<LightStatus> {
    use colored::Colorize;

    let mut light_status = LightStatus::Off;
    std::thread::spawn(move || {
        loop {
            if let Ok(msg) = receiver.recv() {
                match msg {
                    LightMsg::ChangeColor(r, g, b) => {
                        println!("Color was changed to: {}", "  ".on_truecolor(r, g, b));
                        match light_status {
                            LightStatus::Off => println!("Light: Off"),
                            LightStatus::On => println!("Light: On"),
                        }
                    }
                    LightMsg::On => {
                        println!("Turned light on");
                        light_status = LightStatus::On;
                    }
                    LightMsg::Off => {
                        println!("Turned light off");
                        light_status = LightStatus::Off;
                    }
                    LightMsg::Disconnect => {
                        println!("Disconnecting");
                        light_status = LightStatus::Off;
                        break;
                    }
                }
            } else {
                println!("Channel disconnected");
                light_status = LightStatus::Off;
                break;
            }
        }
        light_status
    })
}

pub fn activity() {
    let (sender, receiver) = crossbeam_channel::unbounded();
    let light = spawn_light_thread(receiver);

    sender.send(LightMsg::On).ok();
    sender.send(LightMsg::ChangeColor(255, 0, 0)).ok();
    sender.send(LightMsg::ChangeColor(0, 255, 0)).ok();
    sender.send(LightMsg::ChangeColor(0, 0, 255)).ok();
    sender.send(LightMsg::Off).ok();
    sender.send(LightMsg::Disconnect).ok();

    let light_status = light.join();
}
