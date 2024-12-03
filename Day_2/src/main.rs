use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    //read line
    if let Ok(lines) = read_lines("./input.txt") {
        let total : i32 = lines.flatten()
        .map(|line| -> i32 { 
            let values: Vec<i32>= line.split_ascii_whitespace()
            .map(|value| -> i32 { value.parse().unwrap()})
            .collect();
            
            let asc: bool = &values[0] < &values[1];
            let mut valid: bool = true;

            for value in 1..values.len() {
                let cur = &values[value];
                let prev = &values[value -1];
                let diff = cur.abs_diff(*prev);
                if diff < 1 || diff > 3 {
                    valid = false;
                    break;
                }
                if asc {
                    if cur > prev {
                        continue
                    }
                } else {
                    if cur < prev {
                        continue
                    }
                }
                valid = false;
                break;
            }
            if valid { 
                return 1 
            }
            0
        }
        ).sum();
        println!("Valid {total}");
    }    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}