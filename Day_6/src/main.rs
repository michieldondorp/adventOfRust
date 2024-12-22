use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug, Clone)]
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

    fn is_looping(&self, next_pos: char) -> bool {
        match &self {
            Direction::NORTH | Direction::SOUTH => next_pos == '|',
            Direction::EAST | Direction::WEST => next_pos == '-'
        }
    }

    fn get_next_step_char(&self) -> char {
        match &self {
            Direction::NORTH => '|',
            Direction::EAST => '-',
            Direction::SOUTH => '|',
            Direction::WEST => '-',
        }
    }
}

#[derive(Clone, Debug)]
struct Location {
    x: i16,
    y: i16,
}
#[derive(Clone)]
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
    let loops: i32 = let_guard_step_around(&mut guard, &mut world, true);

    println!("Star 1: {}", get_number_of_x(&world));
    println!("Star 2: {}", loops);
}

fn let_guard_step_around(guard: &mut Guard, world: &mut [Vec<char>], is_main_world: bool) -> i32 {
    let mut looping_worlds: HashSet<String> = HashSet::new();
    let mut backticks = 0;
    let mut turnloop = 0;
    loop {
        let next_position = get_next_position(guard);
        if outside_off_world(world, &next_position) {
            //We're done here.
            return looping_worlds.len() as i32;
        }

        let next_pos = get_pos_in_world(&next_position, world);

        if next_pos == '#' || next_pos == 'O' {
            if turnloop > 3 {
                return 0;
            }
            turnloop+=1;

            update_world(world, &guard.location, '+');
            guard.direction = guard.direction.turn_right();
        } else {
            turnloop=0;
            if is_main_world && next_pos == '.' {
                let mut world_clone: Vec<Vec<char>> = clone_world(world);
                let world_name: String = format! {"{:?}", next_position};
                update_world(&mut world_clone, &next_position, 'O');
        
                if let_guard_step_around(&mut guard.clone(), &mut world_clone, false) > 0
                    && !looping_worlds.contains(&world_name)
                {
                    looping_worlds.insert(world_name);
                }
            } else {
                if guard.direction.is_looping(next_pos) || next_pos == '+' {
                    backticks += 1;
                    if backticks > 1000 {
                        return 1;
                    }
                }
            }

            if next_pos == '|' || next_pos == '-' {
                update_world(world, &next_position, '+');
            } else {
                update_world(world, &next_position, guard.direction.get_next_step_char());
            }

            guard.location = next_position;
        }
    }
}

// fn print_world(world: &mut [Vec<char>]) {
//     for ele in world {
//         for ele in ele {
//             print!("{}", &ele);    
//         }
//         println!();
//     }
// }

fn outside_off_world(world: &[Vec<char>], next_position: &Location) -> bool {
    next_position.x < 0
        || next_position.y < 0
        || next_position.y >= world.len() as i16
        || next_position.x >= world[next_position.y as usize].len() as i16
}

fn get_pos_in_world(location: &Location, world: &mut [Vec<char>]) -> char {
    world[location.y as usize][location.x as usize]
}

fn update_world(world: &mut [Vec<char>], next_position: &Location, value: char) {
    world[next_position.y as usize][next_position.x as usize] = value
}

fn clone_world(world: &mut [Vec<char>]) -> Vec<Vec<char>> {
    let mut world_clone: Vec<Vec<char>> = Vec::new();

    for ele in &mut *world {
        let mut cloned_row: Vec<char> = Vec::new();
        for ele in ele {
            cloned_row.push(ele.clone())
        }
        world_clone.push(cloned_row);
    }
    world_clone
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
                line.push(guard.direction.get_next_step_char());
            }
        }
        world.push(line);
    }
}

fn get_number_of_x(world: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;
    for ele in world {
        for ele in ele {
            if ele == &'X' || ele == &'+' || ele == &'-' || ele == &'|' {
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
