// IKinder

use pulldown_cmark::{Options, Parser};
use std::env;
use std::ffi;
use std::fs;
use std::io;
use std::io::Write;
use std::path;
use std::sync::mpsc;
use std::thread;

pub fn main() {
    match all_md_files() {
        Ok(files) => {
            let (sender, receiver) = mpsc::channel();
            let processes = convert_parallel(files, sender);

            for (index, handle) in processes.into_iter().enumerate() {
                match handle.join() {
                    Ok(()) => println!("Process {index} is finished!"),
                    Err(e) => {
                        if let Some(s) = e.downcast_ref::<String>() {
                            println!("Thread {index} panicked: {s:?}");
                        } else {
                            println!("Unknown error when processing a thread {index}");
                        }
                    }
                }
            }

            for rec in receiver {
                match rec {
                    ProcessorMessage::Success(msg) => println!("Message incoming: {msg}"),
                    ProcessorMessage::Error(e) => println!("Error incoming: {e}"),
                }
            }
        }
        Err(e) => println!("Error collecting MD files: {e}"),
    }
}

enum ProcessorMessage {
    Success(String),
    Error(String),
}

fn all_md_files() -> io::Result<Vec<path::PathBuf>> {
    let mut md_files_path = env::current_dir()?;
    md_files_path.push("md");
    let entries = fs::read_dir(&md_files_path)?;
    let mut paths = Vec::new();
    for entry in entries {
        paths.push(entry?.path());
    }

    Ok(paths)
}

fn convert_parallel(
    files: Vec<path::PathBuf>,
    sender: mpsc::Sender<ProcessorMessage>,
) -> Vec<thread::JoinHandle<()>> {
    files
        .into_iter()
        .enumerate()
        .map(|(index, file)| {
            let sender = sender.clone();
            thread::spawn(move || {
                let contents = match fs::read_to_string(&file) {
                    Ok(contents) => contents,
                    Err(error) => {
                        sender
                            .send(ProcessorMessage::Error(format!(
                                "Failed to read file : {error}"
                            )))
                            .unwrap();
                        return;
                    }
                };

                let parser = Parser::new_ext(&contents, Options::empty());
                let mut html_output = String::new();
                pulldown_cmark::html::push_html(&mut html_output, parser);

                let def = format!("file{index}");
                let current_filename = file
                    .file_name()
                    .unwrap_or(ffi::OsStr::new(&def))
                    .to_str()
                    .unwrap();

                let mut html_file = env::current_dir().expect("Can't read current directory");
                html_file.push("html");
                html_file.push(current_filename);
                html_file.set_extension("html");

                let mut output = fs::File::create(&html_file).expect("Can't create HTML file");
                output
                    .write_all(html_output.as_bytes())
                    .expect("Can't write to HTML file");

                sender
                    .send(ProcessorMessage::Success(format!(
                        "File {file:?} is processed"
                    )))
                    .expect("Failed to send message");
            })
        })
        .collect()
}
