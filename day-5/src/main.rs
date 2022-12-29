use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use regex::Regex;
use std::collections::VecDeque;

fn main() {
    let mut reader = get_file("src/input.txt").lines();
    let mut line;
    let mut char_count: usize;
    let mut cargo: Vec<VecDeque<char>> = (0..9).map(|_| VecDeque::new()).collect();
    let re = Regex::new(r"(\d+).*(\d+).*(\d+)").unwrap();

    /*
     * Insert Innitial Boxes into Stacks
     */
    for _ in 0..9 {
        char_count = 0;
        line = match reader.next().unwrap() {
            Ok(line) => line,
            Err(e) => panic!("{}", e.to_string())
        };
        for char in line.chars() {
            char_count += 1;
            if char.is_alphabetic() {
                cargo[(char_count-2)/4].push_front(char);
            }
        }
    }
    reader.next(); //skip blank line

    /*
     * Iterate through file and move boxes
     */
    while let Some(lines) = reader.next() {
        match lines {
            Err(e) => panic!("Error reading line: {}", e.to_string()),
            Ok(lines) => {
                let cap = re.captures(&lines).unwrap();
                for _ in 0..cap[1].parse::<usize>().unwrap(){
                    match cargo[cap[2].parse::<usize>().unwrap()-1].pop_back() {
                        Some(x) => {cargo[cap[3].parse::<usize>().unwrap()-1].push_back(x);
                        },
                            None => ()
                        };
                }
            }
        }
    }

    /*
     * Output final phrase
     */
    for mut cube in cargo{
        print!("{:?}", cube.pop_back());
    }
    println!();
}
fn get_file(path: &str) -> BufReader<File> {
    let data_path = Path::new(path);
    let file = match File::open(data_path) {
        Err(e) => panic!("Coultn't open {}: {}", data_path.display(), e.to_string()),
        Ok(file) => file,
    };
    BufReader::new(file)
}
