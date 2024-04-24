use std::io::{self, BufRead};
use std::{collections::HashMap, fs::File, path::Path};

use crate::config;

pub fn run(args: config::Config) {
    let mut result = HashMap::new();

    if let Ok(lines) = read_lines(args.file_path) {
        for line in lines.flatten() {
            // println!("line: {}", line);

            let v: Vec<&str> = line.split(":").collect();
            let name = v[0].to_lowercase();
            let balances = v[1].trim();

            let balance: i32 = balances
                .split(" ")
                .map(|v: &str| v.trim().parse::<i32>().unwrap())
                .sum();

            // println!("{}: {}", name, balance);

            result
                .entry(name)
                .and_modify(|v| *v += balance)
                .or_insert(balance);
        }
    }

    println!("\n\n");
    for (key, value) in &result {
        println!("{}: {}", key, value);
    }
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}
