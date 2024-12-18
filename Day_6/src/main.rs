use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}
impl Direction {
    fn turn_right(&self) -> Direction {
        match &self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }
}

struct Location {
    x: i16,
    y: i16,
}

struct Guard {
    location: Location,
    direction: Direction,
}

fn main() {
    let input = read_lines("input.txt").ok().expect("To have read file");

    let mut world: Vec<Vec<char>> = Vec::new();
    let mut guard: Guard = Guard {
        location: Location { x: 0, y: 0 },
        direction: Direction::NORTH,
    };
    create_world_with_guard(input, &mut guard, &mut world);
    println!(
        "World created, {} lines, startpos = {} {}, {:?}",
        world.len(),
        guard.location.x,
        guard.location.y,
        guard.direction
    );
    let_guard_step_around(&mut guard, &mut world);
 
    println!("Star 1: {}", get_number_of_x(&world));
}

fn let_guard_step_around(guard: &mut Guard, world: &mut [Vec<char>]) {
    loop {
        let next_position = get_next_position(guard);
        if next_position.x < 0
            || next_position.y < 0
            || next_position.y >= world.len() as i16
            || next_position.x >= world[next_position.y as usize].len() as i16
        {
            //We done here.
            break;
        }
        
        let next_pos_char = world[next_position.y as usize][next_position.x as usize];

        if next_pos_char == '#' {
            guard.direction = guard.direction.turn_right();
        } else {
            world[next_position.y as usize][next_position.x as usize] = 'X';
            guard.location = next_position;
        }
    }
}

fn get_next_position(guard: &Guard) -> Location {
    match guard.direction {
        Direction::NORTH => Location {
            x: guard.location.x,
            y: guard.location.y - 1,
        },
        Direction::EAST => Location {
            x: guard.location.x + 1,
            y: guard.location.y,
        },
        Direction::SOUTH => Location {
            x: guard.location.x,
            y: guard.location.y + 1,
        },
        Direction::WEST => Location {
            x: guard.location.x - 1,
            y: guard.location.y,
        },
    }
}

fn create_world_with_guard(
    input: io::Lines<io::BufReader<File>>,
    guard: &mut Guard,
    world: &mut Vec<Vec<char>>,
) {
    for (i, ele) in input.flatten().enumerate() {
        let mut line: Vec<char> = Vec::new();
        for (j, c) in ele.chars().enumerate() {
            if c == '.' || c == '#' {
                line.push(c);
            } else {
                guard.location = Location {
                    x: j as i16,
                    y: i as i16,
                };
                guard.direction = get_direction_from_char(c);
                line.push('X');
            }
        }
        world.push(line);
    }
}

fn get_number_of_x(world: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;
    for ele in world {
        for ele in ele {
            if ele == &'X' {
                total += 1;
            }
        }
    }
    total
}

fn get_direction_from_char(c: char) -> Direction {
    let direction = match c {
        '^' => Direction::NORTH,
        '<' => Direction::WEST,
        '>' => Direction::EAST,
        _ => Direction::SOUTH,
    };
    direction
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
