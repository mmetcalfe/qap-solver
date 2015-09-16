use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use permutation::Permutation;

#[derive(Debug)]
pub struct Problem {
    pub size      : usize,
    pub distances : Vec<Vec<f32>>,
    pub weights   : Vec<Vec<f32>>,
}

#[derive(Debug, Clone)]
pub struct Solution {
    pub size  : usize,
    pub value : f32,
    pub perm  : Permutation,
}

impl Problem {
    pub fn from_file(path: &str) -> Problem {

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

        Problem {
            size: size,
            distances: arrays[1..][0..size].to_vec(),
            weights: arrays[1..][size..size*2].to_vec(),
        }
    }

    pub fn print(&self) {

        let print_vec = |ref vec: &Vec<Vec<f32>>| {
            for r in vec.iter() {
                for v in r.iter() {
                    print!("{:4.1} ", v);
                }
                println!("");
            }
        };

        println!("QAP {{");
        println!("  size: {},", self.size);
        println!("  dist: ");
        print_vec(&self.distances);
        println!("  flow: ");
        print_vec(&self.weights);
        println!("}}");
    }

    pub fn value(&self, perm : &Permutation) -> f32 {
        // TODO: Profile this method, and make it fast.

        let mut sol : f32 = 0.0;
        for i in 0..self.size {
            for j in 0..self.size {
                let pi = perm.image[i] as usize;
                let pj = perm.image[j] as usize;

                sol += self.distances[i][j] * self.weights[pi][pj];
            }
        }
        sol
    }

    pub fn solution(&self, perm : &Permutation) -> Solution {
        Solution {
            size: perm.image.len(),
            perm: perm.clone(),
            value: self.value(perm),
        }
    }
}

impl Solution {
    pub fn from_file(path: &str) -> Solution {
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
            Ok(file) => file,
        };
        let mut input = String::new();
        if let Err(why) = file.read_to_string(&mut input) {
            panic!("couldn't read {}: {}", path, Error::description(&why))
        }

        println!("{:?}", input);

        let value_strings : Vec<_> = input.trim()
            .split(|c| " \n".contains(c))
            .collect();

        println!("{:?}", value_strings);

        let size = match value_strings[0].parse::<usize>() {
            Ok(val)  => val,
            Err(why) => panic!("Couldn't parse size {}: {}", value_strings[0], Error::description(&why)),
        };
        let value = match value_strings[1].parse::<f32>() {
            Ok(val)  => val,
            Err(why) => panic!("Couldn't parse value {}: {}", value_strings[1], Error::description(&why)),
        };

        let image : Vec<_> = value_strings[2..].iter()
              .map(|s| s.parse::<u32>())
              .filter_map(Result::ok)
              .map(|v| v - 1) // convert to 0-based indexing
            .collect();

        Solution {
            size: size,
            value: value,
            perm: Permutation::from_image(image),
        }
    }
}
