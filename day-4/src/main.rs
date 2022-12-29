use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;
fn main() {
    let reader = get_file("src/input.txt");
    let mut total: u32 = 0;    
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    for line in reader.lines().flatten() {
        match re.captures(&line) {
            Some(caps) => { // 7-11 and 14-79
                if caps.get(1).unwrap().as_str().parse::<u8>().unwrap() <= caps.get(3).unwrap().as_str().parse::<u8>().unwrap() &&
                    caps.get(2).unwrap().as_str().parse::<u8>().unwrap() >= caps.get(4).unwrap().as_str().parse::<u8>().unwrap() ||
                    caps.get(3).unwrap().as_str().parse::<u8>().unwrap() <= caps.get(1).unwrap().as_str().parse::<u8>().unwrap() &&
                    caps.get(4).unwrap().as_str().parse::<u8>().unwrap() >= caps.get(2).unwrap().as_str().parse::<u8>().unwrap() { 
                        total += 1;
                    }
            },
            None => println!("Failed Somehow")
        };
    } 

    println!("Result: {}", total);
}


/*
Returns BufReader from path
*/
fn get_file(path: &str) -> BufReader<File>{
    let data_path = Path::new(path);
    let file = match File::open(data_path) {
        Err(e) => panic!("Couldn't open{}: {}", data_path.display(), e.to_string()),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    reader
}
