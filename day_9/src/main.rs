use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut lint: Vec<i32> = Vec::new();
    read_lines("input.txt")
        .expect("Proper line")
        .flatten()
        .for_each(|line| populate_lint(&mut lint, line.chars().collect()));

    let total = process_lint_star_1(&mut lint.clone());
    let total2 = process_lint_star_2(&mut lint);

    println!("Star 1 {total}");
    println!("Star 2 {total2}");
}

fn process_lint_star_2(lint: &mut [i32]) -> i64 {
    let mut cur_id = get_last_id(&lint);

    loop {
        if cur_id == 0 {
            break;
        }
        let range = get_range(&lint, cur_id);
        let open_spaces = get_empty_spaces(&lint, range.len(), cur_id);

        if open_spaces.len() > 0 {
            open_spaces.iter().enumerate().for_each(|(index, el)| {
                lint.swap(*el as usize,range[index] as usize);
            });
        }
        cur_id -= 1;
    }

    checksum(lint)
}

fn get_empty_spaces(lint: &[i32], len: usize, cur_id:i32) -> Vec<i32> {
    let mut range: Vec<i32> = Vec::new();
    let mut counter = 0;
    for index in 0..lint.len() {
        if lint[index] == cur_id {
            break;
        }
        if lint[index] == -1 {
            counter += 1;
            if counter == len {
                for free_spots in 0..counter {
                    range.push(index as i32 - free_spots as i32);
                }
                return range;
            }
        } else {
            counter = 0;
        }
    }
    range
}

fn get_range(lint: &[i32], id: i32) -> Vec<i32> {
    let mut range: Vec<i32> = Vec::new();
    for index in (0..lint.len()).rev() {
        if lint[index] == id {
            range.push(index as i32);
        }
    }
    range
}

fn get_last_id(lint: &[i32]) -> i32 {
    let mut last = lint.len() - 1;
    loop {
        if lint[last] == -1 {
            last -= 1;
        } else {
            break;
        }
    }
    lint[last] as i32
}

fn checksum(lint: &[i32]) -> i64 {
    let mut total: i64 = 0;
    lint.iter()
        .enumerate()
        .for_each(|(id, val)| {
            if *val != -1 {
                total += (val * (id as i32)) as i64;
            }
        });
    total
}

fn process_lint_star_1(lint: &mut [i32]) -> i64 {
    let mut cursor = 0;
    let mut end_cursor = lint.len() - 1;
    loop {
        if cursor < end_cursor {
            if lint[cursor] == -1 {
                swap_with_end(lint, cursor, &mut end_cursor);
            }
            cursor += 1;
        } else {
            break;
        }
    }
    checksum(lint)
}

fn swap_with_end(lint: &mut [i32], cursor: usize, end_cursor: &mut usize) {
    loop {
        if lint[*end_cursor] != -1 {
            return lint.swap(cursor, *end_cursor);
        }
        *end_cursor -= 1;
    }
}

fn populate_lint(lint: &mut Vec<i32>, chars: Vec<char>) {
    let mut id: u32 = 0;
    let mut cursor = 0;
    let char_count = chars.len();
    loop {
        if cursor == char_count {
            break;
        }
        for _x in 0..chars[cursor].to_digit(10u32).unwrap() {
            lint.push(id as i32);
        }
        if cursor + 1 == char_count {
            break;
        }
        for _x in 0..chars[cursor + 1].to_digit(10u32).unwrap() {
            lint.push(-1);
        }

        cursor += 2;
        id += 1;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
