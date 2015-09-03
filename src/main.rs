pub mod permutation;

extern crate glob;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

struct Qap {
    size: usize,
    distances: Vec<Vec<f32>>,
    weights: Vec<Vec<f32>>,
}

impl Qap {
    fn from_file(path: &str) -> Qap {

        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut input = String::new();
        if let Err(why) = file.read_to_string(&mut input) {
            panic!("couldn't read {}: {}", path, Error::description(&why))
        }

        let arrays : Vec<_> = input.lines()
            .map(|l| {
              l.trim()
              .split(' ')
              .map(|s| s.parse::<f32>())
              .filter_map(Result::ok)
              .collect::<Vec<_>>()
            })
            .filter(|a| a.len() != 0)
            .collect();

        let size = arrays[0][0] as usize;

        // let result = "123".parse::<f32>().unwrap();
        println!("size: {}, len: {} (should be {})", size, arrays.len(), size*2 + 1);

        Qap {
            size: size,
            distances: arrays[1..size].to_vec(),
            weights: arrays[1 + size..1+size*2].to_vec(),
        }
    }

    fn print(&self) {

        let print_vec = |ref vec: &Vec<Vec<f32>>| {
            for r in vec.iter() {
                for v in r.iter() {
                    print!("{:4.1} ", v);
                }
                println!("");
            }
        };

        println!("QAP {{\n");
        println!("  size: {},\n", self.size);
        println!("  dist: ");
        print_vec(&self.distances);
        println!(",");
        println!("  flow: ");
        print_vec(&self.weights);
        println!(",");
        println!("}}\n");
    }
}


fn main() {
    for path in glob::glob("data/qapdata/chr12a*").unwrap().filter_map(Result::ok) {
        println!("{}", path.display());

        let qap = Qap::from_file(&path.to_string_lossy());

        qap.print();
    }
}
