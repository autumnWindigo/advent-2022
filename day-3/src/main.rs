use std::io::Read;
use std::fs::File;
use std::path::Path;

fn main() {
    let mut rucksacks = get_file("src/input.txt");
    let test: u8 = match rucksacks.pop() {
        Some(item) => item,
        None => 0
    }; 
    let rucksacks_iter = rucksacks.into_iter();
    
}

/*
Returns file data in Vec<u8> from path in str
*/
fn get_file(path: &str) -> Vec<u8>{
    let data_path = Path::new(path);
    let mut file = match File::open(data_path) {
        Err(e) => panic!("Couldn't open{}: {}", data_path.display(), e.to_string()),
        Ok(file) => file,
    };
    let mut data: Vec<u8> = Vec::new();
    match file.read_to_end(&mut data) {
        Err(e) => panic!("Couldn't read file {}: {}", data_path.display(), e.to_string()),
        Ok(bytes) => println!("Read {} bytes from {}", bytes, data_path.display())
    };
    data
}