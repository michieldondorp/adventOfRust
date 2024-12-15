use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let mut map = Vec::new();
    read_lines("input.txt")
        .expect("To have content")
        .flatten()
        .for_each(|line| {
            let mut vec: Vec<char> = Vec::new();
            line.chars().for_each(|c| {
                vec.push(c);
            });
            map.push(vec);
        });

    star_one(&map);
    star_two(&map);
}

fn star_two(map: &Vec<Vec<char>>) {
    let mut matches: u16 = 0;
    for cursor_x in 0i16..map.len() as i16 {
        for cursor_y in 0i16..map[cursor_x as usize].len() as i16 {
            if map[cursor_x as usize][cursor_y as usize] == 'A' {
                if char_at_pos('M', &map, cursor_x - 1, cursor_y - 1)
                    && char_at_pos('S', &map, cursor_x - 1, cursor_y + 1)
                    && char_at_pos('M', &map, cursor_x + 1, cursor_y - 1)
                    && char_at_pos('S', &map, cursor_x + 1, cursor_y + 1)
                {
                    matches += 1;
                }

                if char_at_pos('S', &map, cursor_x - 1, cursor_y - 1)
                    && char_at_pos('M', &map, cursor_x - 1, cursor_y + 1)
                    && char_at_pos('S', &map, cursor_x + 1, cursor_y - 1)
                    && char_at_pos('M', &map, cursor_x + 1, cursor_y + 1)
                {
                    matches += 1;
                }

                if char_at_pos('M', &map, cursor_x - 1, cursor_y - 1)
                    && char_at_pos('M', &map, cursor_x - 1, cursor_y + 1)
                    && char_at_pos('S', &map, cursor_x + 1, cursor_y - 1)
                    && char_at_pos('S', &map, cursor_x + 1, cursor_y + 1)
                {
                    matches += 1;
                }

                if char_at_pos('S', &map, cursor_x - 1, cursor_y - 1)
                    && char_at_pos('S', &map, cursor_x - 1, cursor_y + 1)
                    && char_at_pos('M', &map, cursor_x + 1, cursor_y - 1)
                    && char_at_pos('M', &map, cursor_x + 1, cursor_y + 1)
                {
                    matches += 1;
                }
            }
        }
    }
    println!("Star 2 : Matches {matches}")
}

fn star_one(map: &Vec<Vec<char>>) {
    let word = ['X', 'M', 'A', 'S'];
    let mut matches: u16 = 0;
    for cursor_x in 0..map.len() {
        for cursor_y in 0..map[cursor_x].len() {
            if map[cursor_x][cursor_y] == word[0] {
                //horizontal, vertical, diagonal, written backwards, or even overlapping other words.
                matches += (has_word_left(&word, &map, cursor_x as i16, cursor_y as i16)
                    + has_word_right(&word, &map, cursor_x as i16, cursor_y as i16)
                    + has_word_above(&word, &map, cursor_x as i16, cursor_y as i16)
                    + has_word_below(&word, &map, cursor_x as i16, cursor_y as i16)
                    + has_word_cross_left_top(&word, &map, cursor_x as i16, cursor_y as i16)
                    + has_word_cross_right_top(&word, &map, cursor_x as i16, cursor_y as i16)
                    + has_word_cross_left_bottom(&word, &map, cursor_x as i16, cursor_y as i16)
                    + has_word_cross_right_bottom(&word, &map, cursor_x as i16, cursor_y as i16))
                    as u16;
            }
        }
    }
    println!("Star 1 : Matches {matches}")
}

fn has_word_left(word: &[char], map: &Vec<Vec<char>>, x: i16, y: i16) -> u8 {
    for _value in 1..word.len() {
        if !char_at_pos(word[_value], &map, x, y - _value as i16) {
            return 0;
        }
    }
    return 1;
}

fn has_word_right(word: &[char], map: &Vec<Vec<char>>, x: i16, y: i16) -> u8 {
    for _value in 1..word.len() {
        if !char_at_pos(word[_value], &map, x, y + _value as i16) {
            return 0;
        }
    }
    return 1;
}

fn has_word_above(word: &[char], map: &Vec<Vec<char>>, x: i16, y: i16) -> u8 {
    for _value in 1..word.len() {
        if !char_at_pos(word[_value], &map, x - _value as i16, y) {
            return 0;
        }
    }
    return 1;
}

fn has_word_below(word: &[char], map: &Vec<Vec<char>>, x: i16, y: i16) -> u8 {
    for _value in 1..word.len() {
        if !char_at_pos(word[_value], &map, x + _value as i16, y) {
            return 0;
        }
    }
    return 1;
}

fn has_word_cross_left_top(word: &[char], map: &Vec<Vec<char>>, x: i16, y: i16) -> u8 {
    for _value in 1..word.len() {
        if !char_at_pos(word[_value], &map, x - _value as i16, y - _value as i16) {
            return 0;
        }
    }
    return 1;
}

fn has_word_cross_right_top(word: &[char], map: &Vec<Vec<char>>, x: i16, y: i16) -> u8 {
    for _value in 1..word.len() {
        if !char_at_pos(word[_value], &map, x - _value as i16, y + _value as i16) {
            return 0;
        }
    }
    return 1;
}

fn has_word_cross_left_bottom(word: &[char], map: &Vec<Vec<char>>, x: i16, y: i16) -> u8 {
    for _value in 1..word.len() {
        if !char_at_pos(word[_value], &map, x + _value as i16, y - _value as i16) {
            return 0;
        }
    }
    return 1;
}

fn has_word_cross_right_bottom(word: &[char], map: &Vec<Vec<char>>, x: i16, y: i16) -> u8 {
    for _value in 1..word.len() {
        if !char_at_pos(word[_value], &map, x + _value as i16, y + _value as i16) {
            return 0;
        }
    }
    return 1;
}

// fn char_at_pos2(letter: char, map: &Vec<Vec<char>>, x: i8, y: i8) -> char {
//     if x < 0 || y < 0 || x >= map.len() as i8 || y >= map[x as usize].len() as i8 {
//         return 'C';
//     }
//     return map[x as usize][y as usize];
// }

fn char_at_pos(letter: char, map: &Vec<Vec<char>>, x: i16, y: i16) -> bool {
    if x < 0 || y < 0 || x >= map.len() as i16 || y >= map[x as usize].len() as i16 {
        return false;
    }
    return map[x as usize][y as usize] == letter;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
