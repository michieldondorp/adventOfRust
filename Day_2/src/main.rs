use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let total : u16 = lines.flatten()
        .map(|line| -> u16 { 
            let values: Vec<i32>= line.split_ascii_whitespace()
            .map(|value| -> i32 { value.parse().unwrap()})
            .collect();
            
            if is_valid_report(values, false) {
                return 1;
            }
            0
        }
        ).sum();
        println!("Safe reports: Valid {total}");
    }
    
    if let Ok(lines) = read_lines("./input.txt") {
        let total : u16 = lines.flatten()
        .map(|line| -> u16 { 
            let values: Vec<i32>= line.split_ascii_whitespace()
            .map(|value| -> i32 { value.parse().unwrap()})
            .collect();
            
            if is_valid_report(values, true) {
                return 1;
            }
            0
        }
        ).sum();
        println!("Safe reports with dampener: Valid {total}");
    }  
}

fn is_valid_report(values : Vec<i32>, with_dampening :bool) -> bool{
    if values.len() < 2 {
        return false;
    }
    let asc: bool = &values[0] < &values[1];

    for value in 1..values.len() {
        let cur = &values[value];
        let prev = &values[value -1];
        let diff = cur.abs_diff(*prev);
        if (diff > 0 && diff < 4) && (asc && cur > prev || !asc && cur < prev) {
            continue
        }
        if with_dampening {
            //values waarbij 1 waarde eruit is.
            for subset in 0..values.len() {
                let mut sub = values.clone();
                sub.remove(subset);
                if is_valid_report(sub, false) {
                    return true;
                }
            }
        }
        return false;
    }
    return true;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}