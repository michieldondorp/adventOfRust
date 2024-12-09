use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut input: Vec<char> = Vec::new();
        lines.flatten().for_each(|line| {
            line.chars().for_each(|c| {
                input.push(c);
            });
        });

        let mut index: u32 = 0;
        let mut total: i32 = 0;
        let mut mul_disabled = false;

        let left_mut = ['m', 'u', 'l', '('];
        let do_command = ['d', 'o', '(', ')'];
        let dont_command = ['d', 'o', 'n', '\'', 't', '(', ')'];
        let del_left = ',';
        let del_right = ')';
        while index as usize != input.len() {
            if mul_disabled {
                if lookahead(&mut index, &input, &do_command) {
                    mul_disabled = false;
                } else {
                    index += 1;
                    continue;
                }
            }
            if lookahead(&mut index, &input, &dont_command) {
                mul_disabled = true;
                continue;
            }

            if !find_next_index(&mut index, &input, &left_mut) {
                continue;
            }
            let left = get_next_digit(&mut index, &input, 3, del_left);
            if left == -1 {
                continue;
            }
            let right = get_next_digit(&mut index, &input, 3, del_right);
            if right == -1 {
                continue;
            }

            println!(" {total} + {} - {left} * {right} add", left * right);


            total += left * right;
        }
        println!("{total} Done");
    }
}

fn lookahead(cursor: &mut u32, text: &Vec<char>, chars_to_find: &[char]) -> bool {
    let max_length = chars_to_find.len();
    for value in 0..max_length {
        let cur_index = *cursor as usize + value;
        if cur_index > text.len() {
            return false
        }
        let cur = text[cur_index];
        if chars_to_find[value] != cur {
            return false;
        }
    }
    *cursor += max_length as u32;
    return true;
}

fn get_next_digit(cursor: &mut u32, text: &Vec<char>, max_length: u8, delimiter: char) -> i32 {
    let mut digit: Vec<char> = Vec::new();
    for _value in 0..(max_length + 1) {
        if text.len() < *cursor as usize {
            return -1;
        }
        let cur = &text[*cursor as usize];
        *cursor += 1;
        if cur == &delimiter {
            break;
        }
        if _value == max_length + 1 {
            break;
        }
        if cur.is_numeric() {
            digit.push(*cur);
        } else {
            return -1;
        }
    }
    if digit.len() == 0 {
        return -1;
    }


    let x: String = digit.iter().collect();
    return x.parse().unwrap();
}

fn find_next_index(cursor: &mut u32, text: &Vec<char>, chars_to_find: &[char]) -> bool {
    let mut chars_found = 0;
    let max_len = text.len();
    while *cursor as usize != max_len && chars_found != chars_to_find.len() {
        if text.get(*cursor as usize) == Some(&chars_to_find[chars_found]) {
            chars_found += 1;
            *cursor += 1;
        } else {
            *cursor += 1;
            return false;
        }
    }
    return true;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
