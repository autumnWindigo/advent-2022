use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
enum FileType{
    Dir,
    File
}

#[derive(Debug)]
struct Node<> {
    name: String,
    children: Vec<Node>,
    parent: Option<Box<Node>>,
    value: usize,
    f_type: FileType
}

impl<> Node<> {
    fn new_child<>(&mut self, file_info: String, name: String, parent: Box<Node>) {
        match self.f_type {
            FileType::Dir {..} => {
                if file_info == "dir" {
                    self.children.push(Node{
                        name,
                        children: Vec::new(),
                        parent: Some(parent),
                        value: 0,
                        f_type: FileType::Dir
                    });
                } else {
                    self.children.push(Node {
                        name,
                        children: Vec::new(),
                        parent: Some(parent),
                        value: file_info.parse().unwrap(),
                        f_type: FileType::File
                    });
                }
            },
            FileType::File {..} => {
                println!("Cannot add child to File Node");
            }
        }
    }


    fn new<>(file_info: &str, name: String) -> Node{
        if file_info == "dir" {
            Node {
                name,
                children: Vec::new(),
                value: 0,
                parent: None,
                f_type: FileType::Dir
            }
        } else {
            Node {
                name,
                children: Vec::new(),
                parent: None,
                value: file_info.parse().unwrap(),
                f_type: FileType::File
            }

        }
    }

    fn get_children<>(&self) -> Option<&Vec<Node<>>> {
        match self.f_type {
            FileType::Dir {..} => {
                Some(&self.children)
            },
            FileType::File {..} => {
                None
            }
        }
    }


    fn print_nodes<>(&self, level: u32){
        for _ in 0..level {print!("|\t");}
        print!("|");
        print!("---");
        println!("{}  ({:?})", self.name, self.f_type);
        if self.get_children().is_some() {
            match self.f_type {
                FileType::Dir {..} => {
                    for child in &self.children {
                        child.print_nodes(level+1);
                    }
                },
                FileType::File {..} => {
                    println!("{}, {}", self.name, self.value);
                }
            }
        }
    }
}


fn part_one(input: BufReader<File>) -> usize {
    let mut lines = input.lines();

    //Create File System
    let root = Node::new("dir", "/".to_string());
    let mut working_dir = &root;

    println!("{:?}", working_dir);

    while let Some(Ok(line)) = lines.next() {
        let split = line.split(' ').collect::<Vec<_>>();
        if split[0] == "$" {
            if split[1] == "ls" {
                working_dir.print_nodes(0);
            } else if split[1] == "cd" {
                for node in &working_dir.children {
                    if node.name == split[2] {
                        working_dir = &node;
                    }
                }
            }

        } else if split[0] == "dir" {
            match &working_dir.parent {
                Some(parent) => {
            working_dir.new_child(
                "dir".to_string(), split[1].to_string(), parent);
                },
                None => ()
            }

        } else {

        }
    }

    0
}



fn main() {
    let input = get_file("src/input.txt");
    part_one(input);
}

fn get_file(path: &str) -> BufReader<File> {
    let data_path = Path::new(path);
    let file = match File::open(data_path) {
        Err(e) => panic!("Coultn't open {}: {}", data_path.display(), e),
        Ok(file) => file,
    };
    BufReader::new(file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = get_file("src/example.txt");
        assert_eq!(part_one(input), 95437);
    }

}
