use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Antenna {
    x: i32,
    y: i32,
}
impl Antenna {
    fn fits_on_map(&self, max_x: usize, max_y: usize) -> bool {
        self.x >= 0 && self.x <= max_x as i32 && self.y >= 0 && self.y <= max_y as i32
    }
}

fn main() {
    let mut max_y = 0;
    let mut max_x = 0;
    let mut antenna_map: HashMap<char, Vec<Antenna>> = HashMap::new();
    read_lines("input.txt")
        .expect("We should get lines")
        .flatten()
        .enumerate()
        .for_each(|(y, line)| {
            create_antenna_map(&mut max_x, &mut antenna_map, y, line);
            if y > max_y {
                max_y = y;
            }
        });
    let count_star_1 = number_of_antinodes(max_y, max_x, &antenna_map, true);
    let count_star_2 = number_of_antinodes(max_y, max_x, &antenna_map, false);
    println!("star 1 {count_star_1}");
    println!("star 2 {count_star_2}");
}

fn number_of_antinodes(
    max_y: usize,
    max_x: usize,
    antenna_map: &HashMap<char, Vec<Antenna>>,
    star1: bool,
) -> usize {
    let mut all_antinodes: HashSet<Antenna> = HashSet::new();
    for ele in antenna_map {
        let vec_size = ele.1.len();
        for (i, antenna1) in ele.1.iter().enumerate() {
            for x in i + 1..vec_size {
                if star1 {
                    for antinode in antinodes_for_antenna(antenna1, &ele.1[x]) {
                        if antinode.fits_on_map(max_x, max_y) {
                            all_antinodes.insert(antinode);
                        }
                    }
                } else {
                    for antinode in
                        antinodes_for_antenna_resonate(antenna1, &ele.1[x], max_x, max_y)
                    {
                        all_antinodes.insert(antinode);
                    }
                }
            }
        }
    }
    all_antinodes.len()
}

fn antinodes_for_antenna_resonate(
    antenna1: &Antenna,
    antenna2: &Antenna,
    max_x: usize,
    max_y: usize,
) -> Vec<Antenna> {
    let mut map: Vec<Antenna> = Vec::new();
    let diffx = antenna2.x - antenna1.x;
    let diffy = antenna2.y - antenna1.y as i32;

    let mut next_x = antenna1.x - diffx;
    let mut next_y = antenna1.y - diffy;
    map.push(antenna1.clone());
    map.push(antenna2.clone());

    loop {
        let lower = Antenna {
            x: next_x,
            y: next_y,
        };
        if !lower.fits_on_map(max_x, max_y) {
            break;
        }
        map.push(lower);
        next_x -= diffx;
        next_y -= diffy;
    }

    next_x = antenna2.x + diffx;
    next_y = antenna2.y + diffy;
    loop {
        let higher = Antenna {
            x: next_x,
            y: next_y,
        };
        if !higher.fits_on_map(max_x, max_y) {
            break;
        }
        map.push(higher);
        next_x += diffx;
        next_y += diffy;
    }
    map
}

fn create_antenna_map(
    max_x: &mut usize,
    antenna_map: &mut HashMap<char, Vec<Antenna>>,
    y: usize,
    line: String,
) {
    for (x, ele) in line.chars().enumerate() {
        if x > *max_x {
            *max_x = x;
        }
        if ele == '.' {
            continue;
        }
        if !antenna_map.contains_key(&ele) {
            antenna_map.insert(ele, Vec::new());
        }
        antenna_map.get_mut(&ele).unwrap().push(Antenna {
            x: x as i32,
            y: y as i32,
        });
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn antinodes_for_antenna(antenna1: &Antenna, antenna2: &Antenna) -> [Antenna; 2] {
    let diffx = antenna2.x - antenna1.x as i32;
    let diffy = antenna2.y - antenna1.y as i32;

    [
        Antenna {
            x: antenna1.x - diffx,
            y: antenna1.y - diffy,
        },
        Antenna {
            x: antenna2.x + diffx,
            y: antenna2.y + diffy,
        },
    ]
}
