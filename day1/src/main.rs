use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn get_increases(path: &str) -> io::Result<u32> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    let mut increase_count: u32 = 0;
    let mut previous_value: u32 = u32::MAX;

    for line in lines {
        if let Ok(string) = line {
            let value: u32 = string.parse().unwrap_or_default();

            if value > previous_value {
                increase_count += 1;
            }

            previous_value = value;
        }
    }

    return Ok(increase_count);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Checking increases from file {}", filename);
    let result = get_increases(filename);

    match result {
        Ok(count) => println!("Found {} increases in values", count),
        Err(err) => panic!("Unexpeced error: {}", err),
    }
}
