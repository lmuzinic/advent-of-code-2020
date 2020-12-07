use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn input_vec<T: std::str::FromStr>(filename: &str) -> Vec<T> {
    let mut input: Vec<T> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(value) = line {
                if let Ok(parsed) = value.parse::<T>() {
                    input.push(parsed);
                }
            }
        }
    }

    input
}
