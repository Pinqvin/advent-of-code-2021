use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn get_sliding_window_increases(path: &str) -> io::Result<u32> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    let mut window = Vec::new();
    let mut values: Vec<u32> = Vec::new();
    let mut increase_count: u32 = 0;

    for line in lines {
        if let Ok(string) = line {
            let value: u32 = string.parse().unwrap_or_default();

            window.push(value);

            if window.len() == 3 {
                values.push(window.iter().sum());
                window.remove(0);
            }

            if values.len() == 2 {
                if values[1] > values[0] {
                    increase_count += 1;
                }

                values.remove(0);
            }
        }
    }

    return Ok(increase_count);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Checking sliding window increases from file {}", filename);
    let result = get_sliding_window_increases(filename);

    match result {
        Ok(count) => println!("Found {} increases in values", count),
        Err(err) => panic!("Unexpeced error: {}", err),
    }
}
