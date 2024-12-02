use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let mut reader = my_reader::BufReader::open("input.txt")?;
    let mut buffer = String::new();

    let mut left_bucket:Vec<i32> = Vec::new();
    let mut right_bucket:Vec<i32> = Vec::new();

    while let Some(line) = reader.read_line(&mut buffer) {
        let words: Vec<&str> = line?.split_whitespace().collect();
        if words.len() == 2 {
            let left = &words[0];
            let right = &words[1];
            left_bucket.push(left.parse().expect("Not a number"));
            right_bucket.push(right.parse().expect("Not a number"));
        }
    }

    left_bucket.sort();
    right_bucket.sort();

    get_star_for_distance(left_bucket.clone(), right_bucket.clone());

    get_star_for_similarity(left_bucket, right_bucket);

    Ok(())
}

fn get_star_for_similarity(left_bucket: Vec<i32>, right_bucket:Vec<i32> ){
    let set: HashSet<i32> = HashSet::from_iter(left_bucket.iter().cloned());
    let mut total_distance: i32 = 0;

    set.iter().for_each(|x| {
        let mut occurring :i32 = 0;
        //Dit kan beter, ik kan een cursor gebruiken zodat we verder gaan waar we gebleven zijn, immers gesorteerd.
        right_bucket.iter().for_each(|y| {
            if x == y{
                occurring += 1;
            }
        });
        total_distance += occurring * x;
    });
    println!("{total_distance} - {}", set.len());
}

fn get_star_for_distance(left_bucket: Vec<i32>, right_bucket:Vec<i32> ) {
    let mut total_distance: i32 = 0;

    for (i, el) in left_bucket.iter().enumerate() {
        total_distance += get_diff(*el, right_bucket[i]);
     }


    println!("{total_distance} - {}", left_bucket.len());
}

fn get_diff(left : i32, right : i32) -> i32{
    if left > right {
        return left - right
    }
    right - left
}

//https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}