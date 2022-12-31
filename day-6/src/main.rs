use std::fs::File;
use std::io::Read;
use std::collections::VecDeque;

fn main() -> std::io::Result<()> {
    const MSG_LENGTH: usize = 14;
    const PATH: &str = "src/input.txt";

    let mut f = File::open(PATH)?;
    let mut buffer = Vec::new();
    let mut marker = VecDeque::<u8>::new();
    let mut marker_check: Vec::<u8>;
    let mut result: u32 = 0;
    let mut char_check: u8;

    f.read_to_end(&mut buffer)?;

    for &char in &buffer {
        //Insert Char
        result += 1;
        marker.push_back(char);
        marker_check = marker.clone().into();
        //Remove Dups
        marker_check.sort();
        marker_check.dedup();

        //check size
        if marker_check.len() == MSG_LENGTH {
            break;
        }
        //Remove Old
        if marker.len() >= MSG_LENGTH {
            char_check = marker.pop_front().unwrap();
            marker_check.retain(|&x| x != char_check);
        }
    }

    println!("Answer: {}", result);
    Ok(())
}

