use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Clone)]
struct Equation {
    total: i64,
    ints: Vec<i64>,
}

fn main() {
    let lines = read_lines("input.txt").ok().expect("To have read file");
    let operators: Vec<Equation> = lines
        .flatten()
        .map(|line| -> Equation { line_to_equation(line.clone()) })
        .collect();

    let sum1: i64 = operators
        .clone()
        .into_iter()
        .filter(|eq| solve_with_operators(eq, 1, eq.ints[0]))
        .map(|eq| eq.total)
        .sum::<i64>();

    let sum2: i64 = operators
        .into_iter()
        .filter(|eq| solve_with_operators_with_concat(eq.ints[0], &eq.ints.as_slice()[1..], eq.total))
        .map(|eq| eq.total)
        .sum::<i64>();

    println!("Star 1: {}", sum1);
    println!("Star 2: {}", sum2);
}

fn operator_concat(left: i64, right: i64) -> i64 {
    let mut stringed_concat: String = left.to_string();
    stringed_concat.push_str(&right.to_string());
    stringed_concat
        .parse()
        .expect("input should really be an positive int")
}

fn solve_with_operators_with_concat(current_sum: i64, ints: &[i64], total: i64) -> bool {
    if total < current_sum {
        return false;
    }
    if ints.len() > 0 {
        let next: i64 = ints[0];

        if solve_with_operators_with_concat(current_sum + next, &ints[1..ints.len()], total) {
            return true
        }
        if solve_with_operators_with_concat(current_sum * next, &ints[1..ints.len()], total) {
            return true
        }
        return solve_with_operators_with_concat(operator_concat(current_sum, next), &ints[1..ints.len()], total);
    }
    return total == current_sum;
}

fn solve_with_operators(eq: &Equation, pos: i32, total: i64) -> bool {
    if total > eq.total {
        return false;
    }
    if pos == eq.ints.len() as i32 {
        return total == eq.total;
    }

    if solve_with_operators(eq, pos + 1, total + eq.ints[pos as usize]) {
        return true;
    }
    return solve_with_operators(eq, pos + 1, total * eq.ints[pos as usize]);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(operator_concat(1, 2), 12);
    }

    #[test]
    fn test_concat_2() {
        assert_eq!(operator_concat(123, 456), 123456);
    }

    #[test]
    #[should_panic]
    fn test_concat_error() {
        assert_eq!(operator_concat(123, -1), 123456);
    }
}
