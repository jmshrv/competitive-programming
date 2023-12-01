use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::{Hash, Hasher},
    io,
};

#[derive(Clone)]
struct Directory {
    name: String,
    size: usize,
    children: Vec<Directory>,
}

impl Hash for Directory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Directory {}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name: name,
            size: 0,
            children: Vec::new(),
        }
    }

    fn get_child(&mut self, name: String) -> Option<&mut Directory> {
        for child in self.children.iter_mut() {
            if child.name == name {
                return Some(child);
            }
        }

        None
    }
}

fn main() {
    let input: Vec<_> = io::stdin()
        .lines()
        .map(|line_res| line_res.unwrap())
        .skip(2) // Don't care about initial cd and ls
        .collect();

    let mut fs_tree = Directory::new("/".to_string());
    let mut cur_dir_stack = vec![&mut fs_tree];

    for line in input {
        let mut cur_dir = cur_dir_stack.last_mut().unwrap();

        // Directory
        if line.starts_with("dir") {
            let dir_name = line.split(' ').last().unwrap();
            cur_dir.children.push(Directory::new(dir_name.to_string()));
        }
        // Change directory
        else if line.starts_with("$ cd") {
            if line == "$ cd .." {
                cur_dir_stack.pop();
                cur_dir = cur_dir_stack.last_mut().unwrap();
            } else {
                let dest_name = line.split(' ').last().unwrap();
                let dest = cur_dir.clone().get_child(dest_name.to_string()).unwrap();

                cur_dir_stack.push(dest);
            }
        }
        // We don't care about ls
        else if line == "$ ls" {
        }
        // File
        else {
            let (size_str, name) = line.split_once(' ').unwrap();
            cur_dir.size += size_str.parse::<usize>().unwrap();
        }
    }

    println!("hi");
}
