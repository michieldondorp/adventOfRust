use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

struct OrdeningRule{
    x: i8,
    y: i8
}

fn main() {

    let input = read_lines("input.txt").ok().expect("To have read file");

    let mut rules_ended = false;
    let mut rules: Vec<OrdeningRule> = Vec::new();

    let mut total: i32 = 0;
    let mut total2: i32 = 0;

    input.flatten().for_each(|line: String|{
        if line.is_empty() {
            rules_ended = true;
            return;
        }
        if !rules_ended {        
           rules.push(input_line_to_ordening_rule(&line));
        } else {
            let pages: Vec<i8> = input_line_to_page_list(line);
            let mut page_set = HashSet::new();
            for ele in &pages {
                page_set.insert(ele);
            } 

            total += validate_rules(&rules, &page_set, &pages) as i32;
            total2 += validate_rules_star2(&rules, &page_set, &pages, false) as i32;
        }    
    });
    println!("Star 1: successes {total}");
    println!("Star 2: successes {total2}");
}

fn validate_rules(rules: &Vec<OrdeningRule>, page_set: &HashSet<&i8>, pages: &Vec<i8>) -> i8{
    for rule in rules.iter().filter(|rule| -> bool {
        return page_set.contains(&rule.x) && page_set.contains(&rule.y);
    }) {
        if get_position_of_page(&pages, rule.x) > get_position_of_page(&pages, rule.y) {
            return 0;
        }
    }
    return pages[(pages.len() -1) / 2];
}

fn validate_rules_star2(rules: &Vec<OrdeningRule>, page_set: &HashSet<&i8>, pages: &Vec<i8>, retry : bool) -> i8{
    for rule in rules.iter().filter(|rule| -> bool {
        return page_set.contains(&rule.x) && page_set.contains(&rule.y);
    }) {
        let pos_x = get_position_of_page(&pages, rule.x);
        let pos_y = get_position_of_page(&pages, rule.y);
        if get_position_of_page(&pages, rule.x) > get_position_of_page(&pages, rule.y) {
            let mut clone = pages.clone();
            let tmp = clone[pos_x];
            clone[pos_x] = clone[pos_y];
            clone[pos_y] = tmp;
            return validate_rules_star2(rules, page_set , &clone, true);
        }
    }
    if retry {
        return pages[(pages.len() -1) / 2];
    }
    0
}

fn get_position_of_page(pages: &Vec<i8>, x: i8) -> usize {
    pages.iter().position(|&r| r == x).unwrap()
}

fn input_line_to_page_list(line: String) -> Vec<i8> {
    return line.split(',').map(|x: &str| -> i8 {
        return x.parse().unwrap();
    }).collect()
}

fn input_line_to_ordening_rule(line: &String) -> OrdeningRule {
    let split : Vec<i8> = line.split('|').map(|x| -> i8 {
        return x.parse().unwrap();
    }).collect();
    OrdeningRule {x : split[0], y: split[1]}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}