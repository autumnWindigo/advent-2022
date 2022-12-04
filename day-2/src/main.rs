use std::fs::File;
use std::io::Read;
use std::path::Path;


fn main() {
    //create result array    
    let score: Vec<Vec<u32>> = vec![vec![3, 6, 0],
                                   vec![0, 3, 6],
                                   vec![6, 0, 3]];

    //Load file
    let data_path = Path::new("src/input.txt");
    let mut file = match File::open(data_path) {
        Err(e) => panic!("Couldn't open{}: {}", data_path.display(), e.to_string()),
        Ok(file) => file,
    };

    let mut rps_data = Vec::new();

    match file.read_to_end(&mut rps_data) {
        Err(e) => panic!("Couldn't read file {}: {}", data_path.display(), e.to_string()),
        Ok(bytes) => println!("Read {} bytes from {}", bytes, data_path.display())
    };

    //Create Iter without whitespace or newLine
    let mut rps_data_iter = rps_data.into_iter().filter(|&x| x != 10 && x != 32);

    //Option(firstChar, secondChar)
    let mut rps_result;
    let mut points: u32 = 0;

    loop {
        rps_result = rps_data_iter.next().zip(rps_data_iter.next());
        match rps_result  {
            Some(result) => {
                points += (result.1-87) as u32 + score[(result.0-65) as usize][(result.1-88) as usize];
            }, 
            None => break
        };
    }
    println!("Total points: {}", points);
}