use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug)]
struct Equation {
    total: i64,
    ints: Vec<i64>,
}

fn main() {
    let lines = read_lines("input.txt").ok().expect("To have read file");
    let sum = lines.flatten().map(|line| -> Equation {
        line_to_equation(line.clone())
    }).filter( |eq| {
        solve_with_operators(eq, 1, eq.ints[0])
    }).map(|eq| {
        eq.total
    }).sum::<i64>();

    println!("Star 1: {}", sum);
}

fn solve_with_operators(eq: &Equation, pos: i32, total: i64) -> bool {
    if total > eq.total {
        return false;
    }
    if pos == eq.ints.len() as i32 {
        return total == eq.total;
    }

    if solve_with_operators(
        eq,
        pos + 1,
        total + eq.ints[pos as usize]
    ) {
        return true;
    }
    return solve_with_operators(
        eq,
        pos + 1,
        total * eq.ints[pos as usize]
    )
}

fn line_to_equation(input: String) -> Equation {
    let split: Vec<_> = input.split(':').collect();
    Equation {
        total: split[0].parse().unwrap(),
        ints: split[1]
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect(),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
