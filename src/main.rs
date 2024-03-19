extern crate seahash;

use seahash::hash;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin, stdout, Write};
use std::process::exit;

struct BitArray {
    array: Vec<u8>,
    word_count: usize,
}

impl BitArray {
    fn new(s: usize) -> BitArray {
        let b = BitArray {
            array: vec!(0; s / 8),
            word_count: 0,
        };
        b
    }

    fn len(&self) -> usize {
        self.word_count
    }

    fn add_word(&mut self, s: &str) -> () {
        let h = hash(s.as_bytes());
        let i = split_hash(h);
        let wrap_value = (self.array.len() * 8) as u16;

        for x in i.iter() {
            let index = (x % wrap_value) / 8;
            let current = self.array[index as usize];
            let sub_index = {
                let m = (x % wrap_value) % 8;
                1 << (7 - m)
            };
            self.array[index as usize] = current | sub_index;
        }
        self.word_count += 1; // Naive!
    }

    fn check(&self, s: &str) -> bool {
        let h = hash(s.as_bytes());
        let i = split_hash(h);
        let wrap_value = (self.array.len() * 8) as u16;

        for x in i.iter() {
            let index = (x % wrap_value) / 8;
            let sub_index = {
                let m = (x % wrap_value) % 8;
                1 << (7 - m)
            };
            
            if (self.array[index as usize] & sub_index).count_ones() == 1 {
                continue;
            }
            else {
                return false;
            }
        }
        true
    }
}

fn split_hash(h: u64) -> Vec<u16> {
    let a = h & 0xFFFF;
    let b = (h >> 16) & 0xFFFF;
    let c = (h >> 32) & 0xFFFF;
    let d = (h >> 48) & 0xFFFF;
    vec!(a as u16, b as u16, c as u16, d as u16)
}

fn get_words() -> File {
    match File::open("words.txt") {
        Err(e) => {
            println!("Can't find words.txt.\n{}", e);
            exit(1);
        },
        Ok(f) => f,
    }
}

fn main() {
    let s: usize = 64_000;
    println!("Creating {} byte bloom filter...", s / 8);
    
    let mut my_dict = BitArray::new(s);
    let file = BufReader::new(get_words());
    for line in file.lines() {
        let word = match line {
            Err(e) => {
                println!("Error reading word.\n{}", e);
                continue;
            },
            Ok(w) => w,
        };
        my_dict.add_word(word.as_str());
    };      
    println!("Finished: {} words.\nq to quit.", &my_dict.len());

    loop {
        print!("» ");
        stdout().flush().unwrap();
        
        let mut uw = String::new();
        match stdin().read_line(&mut uw) {
            Err(e) => {
                println!("Error with input!\n{}", e);
                continue;
            },
            Ok(_) => ()
        };
        
        let uw = uw.trim();
        println!("{}", &uw);
        
        match my_dict.check(uw) {
            true => println!(" ✔"),
            false => println!(" ✗"),
        };

        if uw == "q" {
            break;
        };
    };
}
