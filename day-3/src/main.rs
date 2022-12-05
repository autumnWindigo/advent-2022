use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::collections::BTreeSet;

fn main() {
    let mut rucksacks = get_file("src/input.txt");
    let mut optional = rucksacks.pop();
    let mut total: u32 = 0;
    
    let mut line_length;
    let mut line_vec: Vec<u8> = Vec::new();
    
    let mut half_one: BTreeSet<u8> = BTreeSet::new();
    let mut half_two: BTreeSet<u8> = BTreeSet::new();

    
    while !rucksacks.is_empty() {
        optional = rucksacks.pop(); //pops new line
        //Pop from Vec until (10) newLine
        while let Some(item) = optional {
            if item == 10 { // new line, stop reading
                optional = None;
            }   else {
                line_vec.push(item);
                optional = rucksacks.pop();
            }    
        } //End of while let
        line_length = line_vec.len();
        for value in 0 .. line_length {
            if value <= line_length/2-1 {
                match line_vec.pop(){
                   Some(value) => half_one.insert(value),
                   None => false
                };
            }   else {
                match line_vec.pop() {
                    Some(value) => half_two.insert(value),
                    None => false
                };
            }
        }
        let intersection: Vec<_> = half_one.intersection(&half_two).cloned().collect();
        if intersection[0] > 96 {
            total += (intersection[0]-96) as u32;
        } else {
            total += (intersection[0]-38) as u32;
        }
        half_one.clear();
        half_two.clear();
    }
    println!("total: {}", total);
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
