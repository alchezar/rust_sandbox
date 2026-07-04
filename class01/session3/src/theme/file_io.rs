// IKinder

use std::io::BufRead;
use std::{fs, io, time};

pub fn main() {
    crate::show_name(file!());

    let now = time::Instant::now();
    let mut line_count = 0;
    if let Ok(lines) = read_lines("../war_and_peace.txt") {
        lines.for_each(|line| {
            if let Ok(l) = line {
                if !l.trim().is_empty() {
                    line_count += 1;
                }
            }
        });
    }
    println!(
        "Read {} lines in {:.3} seconds.",
        line_count,
        now.elapsed().as_secs_f32()
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[tokio::main]
async fn tokio() -> anyhow::Result<()> {
    println!("Reading war_and_peace...");
    let now = time::Instant::now();
    let (c1, c2) = tokio::join!(
        line_count_real_async("../war_and_peace.txt".to_string()),
        line_count_real_async("../war_and_peace.txt".to_string())
    );
    println!("Total lines: {}", c1? + c2?);
    println!("In {:.3} seconds.", now.elapsed().as_secs_f32());
    Ok(())
}

async fn line_count(filename: String) -> anyhow::Result<usize> {
    let now = time::Instant::now();
    let mut line_count = 0;
    if let Ok(lines) = read_lines("../war_and_peace.txt") {
        lines.for_each(|line| {
            if let Ok(l) = line {
                if !l.trim().is_empty() {
                    line_count += 1;
                }
            }
        });
    }
    println!(
        "Read {} lines in {:.3} seconds",
        line_count,
        now.elapsed().as_secs_f32()
    );
    Ok(line_count)
}

async fn line_count_real_async(filename: String) -> anyhow::Result<usize> {
    use tokio::fs::File;
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;

    println!("Reading {filename}");
    let now = tokio::time::Instant::now();
    let mut line_count = 0;

    let file = File::open(&filename).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if !line.trim().is_empty() {
            line_count += 1;
        }
    }

    println!(
        "Read {} lines in {:.3} seconds",
        line_count,
        now.elapsed().as_secs_f32()
    );
    Ok(line_count)
}
