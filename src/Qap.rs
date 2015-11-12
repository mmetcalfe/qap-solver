use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
extern crate rand;
use rand::Rng;
use time::Duration;
use std::f64;
use std::fs::OpenOptions;

use permutation::Permutation;

#[derive(Debug, Clone)]
pub struct Problem {
    pub size      : usize,
    pub distances : Vec<Vec<f64>>,
    pub weights   : Vec<Vec<f64>>,
}

#[derive(Debug, Clone)]
pub struct Solution {
    pub size  : usize,
    pub value : f64,
    pub perm  : Permutation,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub solution  : Solution,
    pub best_known_solution : Option<Solution>,
    pub running_time_seconds : f64,
    pub instance_name : String,
    pub time_to_best_solution_seconds : f64,
}

impl SearchResult {
    pub fn new(best_soln : Solution, search_duration : Duration, best_soln_duration : Duration) -> SearchResult {
        let total_time_seconds = (search_duration.num_milliseconds() as f64)/1000.0;
        let best_soln_seconds = (best_soln_duration.num_milliseconds() as f64)/1000.0;
        SearchResult {
            solution: best_soln,
            running_time_seconds: total_time_seconds,
            best_known_solution: Option::None,
            instance_name: String::new(),
            time_to_best_solution_seconds: best_soln_seconds,
        }
    }

    pub fn append_to_file(&self, fname : &str) {
        let mut file = OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(true)
                            .open(fname)
                            .unwrap();

        let mut best_known_solution_value = f64::INFINITY;
        if self.best_known_solution.is_some() {
            let sln = self.best_known_solution.clone().unwrap();
            best_known_solution_value = sln.value;
        }

        let deviation = (self.solution.value-best_known_solution_value) / best_known_solution_value;
        println!("Deviation: {}%", deviation*100.0);

        writeln!(file, " - {{ instance_name: {}", self.instance_name);
        writeln!(file, ", soln_value: {}", self.solution.value);
        writeln!(file, ", best_known_solution_value: {}", best_known_solution_value);
        writeln!(file, ", time_to_best_solution_seconds: {}", self.time_to_best_solution_seconds);
        writeln!(file, ", running_time_seconds: {}", self.running_time_seconds);
        write!(file, ", soln_perm: [");
        for i in 0..self.solution.perm.image.len() {
            write!(file, "{}", self.solution.perm.image[i]);
            if i + 1 < self.solution.perm.image.len() {
                write!(file, ", ");
            }
        }
        writeln!(file, "]");
        writeln!(file, "}}");
    }
}

impl Problem {
    pub fn random(size : usize) -> Problem {
        let mut distances : Vec<Vec<f64>> = vec!(vec!(0.0; size); size);
        let mut weights : Vec<Vec<f64>> = vec!(vec!(0.0; size); size);
        let mut rng = rand::thread_rng();
        for i in 0..size {
            for j in 0..size {
                distances[i][j] = rng.gen();
                weights[i][j] = rng.gen();
            }
        }

        for k in 0..size {
            distances[k][k] = 0.0;
            weights[k][k] = 0.0;
        }

        Problem {
            size: size,
            distances: distances,
            weights: weights
        }
    }

    pub fn random_integral(size : usize) -> Problem {
        let mut distances : Vec<Vec<f64>> = vec!(vec!(0.0; size); size);
        let mut weights : Vec<Vec<f64>> = vec!(vec!(0.0; size); size);
        let mut rng = rand::thread_rng();
        for i in 0..size {
            for j in 0..size {
                distances[i][j] = rng.gen_range(0 as usize, 2*size) as f64;
                weights[i][j] = rng.gen_range(0 as usize, 2*size) as f64;
            }
        }

        for k in 0..size {
            distances[k][k] = 0.0;
            weights[k][k] = 0.0;
        }

        Problem {
            size: size,
            distances: distances,
            weights: weights
        }
    }

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

        let values : Vec<_> = input.trim()
            .split(|c| " \n".contains(c))
            .map(|s| s.parse::<f64>())
            .filter_map(Result::ok)
            .collect();

        let size = values[0] as usize;

        let arrays : Vec<_> = values[1..].chunks(size).map(|c| c.to_vec()).collect();

        // let result = "123".parse::<f64>().unwrap();
        // println!("size: {}, len: {} (should be {})", size, arrays.len(), size*2);

        Problem {
            size: size,
            // distances: arrays[0..size].to_vec(),
            // weights: arrays[size..size*2].to_vec(),

            weights: arrays[0..size].to_vec(),
            distances: arrays[size..size*2].to_vec(),
        }
    }

    pub fn print(&self) {

        let print_vec = |ref vec: &Vec<Vec<f64>>| {
            for r in vec.iter() {
                for v in r.iter() {
                    print!("{:8.1} ", v);
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

    pub fn value(&self, perm : &Permutation) -> f64 {
        // TODO: Profile this method, and make it fast.

        let mut sol : f64 = 0.0;
        for i in 0..self.size {
            for j in 0..self.size {
                let pi = perm.image[i] as usize;
                let pj = perm.image[j] as usize;

                sol += self.weights[i][j] * self.distances[pi][pj];
                // sol += self.distances[i][j] * self.weights[pi][pj];
            }
        }

        // // Very slightly faster:
        // for (i, pi) in perm.image.iter().enumerate() {
        //     for (j, pj) in perm.image.iter().enumerate() {
        //         // sol += self.distances[i][j] * self.weights[*pi as usize][*pj as usize];
        //         unsafe {
        //             let dists_i = self.distances.get_unchecked(i);
        //             let weights_pi = self.weights.get_unchecked(*pi as usize);
        //             sol += dists_i.get_unchecked(j) * weights_pi.get_unchecked(*pj as usize);
        //         }
        //     }
        // }

        sol
    }

    pub fn solution_value_difference(&self, perm : &Permutation, transp : (usize, usize)) -> f64 {
        let (i, j) = transp;

        let a = &self.distances;
        let b = &self.weights;
        let p = &perm.image;

        let pi = p[i] as usize;
        let pj = p[j] as usize;

        let mut diff = (a[i][i] - a[j][j]) * (b[pj][pj] - b[pi][pj]) +
                       (a[i][j] - a[j][i]) * (b[pj][pi] - b[pi][pj]);

        for k in (0..p.len()) {
            let pk = p[k] as usize;

            if k != i && k != j {
                diff += (a[i][k] - a[j][k]) * (b[pj][pk] - b[pi][pk]) +
                        (a[k][i] - a[k][j]) * (b[pk][pj] - b[pk][pi]);
            }
        }

        diff
    }

    pub fn compute_neighbourhood_delta_matrix_very_slow(&self, perm : &Permutation) -> Vec<f64> {
        let M = self.size*(self.size-1)/2;
        let mut delta_matrix = Vec::with_capacity(self.size);

        let soln_value = self.value(perm);

        for m in 0..M {
            let transp = Permutation::triangle_index(m);

            let neighbour = Permutation::apply_transposition(perm.clone(), transp);
            let neighbour_value = self.value(&neighbour);

            delta_matrix.push(neighbour_value - soln_value);
        }
        delta_matrix
    }

    pub fn compute_neighbourhood_delta_matrix(&self, perm : &Permutation) -> Vec<f64> {
        let M = self.size*(self.size-1)/2;
        let mut delta_matrix = Vec::with_capacity(self.size);
        for m in 0..M {
            let transp = Permutation::triangle_index(m);
            delta_matrix.push(self.solution_value_difference(&perm, transp));
        }
        delta_matrix
    }

    pub fn transposition_neighbourhood(&self, perm : &Permutation) -> Vec<(usize,usize)> {
        let M = self.size*(self.size-1)/2;
        let mut transpositions = Vec::with_capacity(self.size);
        for m in 0..M {
            let transp = Permutation::triangle_index(m);
            transpositions.push(transp);
        }
        transpositions
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
    pub fn from_file(path: &str) -> Option<Solution> {
        let mut file = match File::open(&path) {
            Err(why) => return None, // panic!("couldn't open {}: {}", path, Error::description(&why)),
            Ok(file) => file,
        };
        let mut input = String::new();
        if let Err(why) = file.read_to_string(&mut input) {
            panic!("couldn't read {}: {}", path, Error::description(&why))
        }

        // println!("{:?}", input);

        let value_strings : Vec<_> = input.trim()
            .split(|c| ", \n".contains(c))
            .map(|s| s.trim())
            .filter(|s| s.len() != 0)
            .collect();

        // println!("{:?}", value_strings);

        let size = match value_strings[0].parse::<usize>() {
            Ok(val)  => val,
            Err(why) => panic!("Couldn't parse size {}: {}", value_strings[0], Error::description(&why)),
        };
        let value = match value_strings[1].parse::<f64>() {
            Ok(val)  => val,
            Err(why) => panic!("Couldn't parse value {}: {}", value_strings[1], Error::description(&why)),
        };

        let mut image : Vec<_> = value_strings[2..].iter()
              .map(|s| s.parse::<u32>())
              .filter_map(Result::ok)
            .collect();

        if !image.contains(&0) {
            image = image.iter().map(|v| v - 1).collect();
        }

        // println!("{:?}", value);

        Some(Solution {
            size: size,
            value: value,
            perm: Permutation::from_image(image),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use permutation::Permutation;

    fn test_vector_equality(va : &Vec<f64>, vb : &Vec<f64>, tol : f64) -> bool {
        for i in (0..va.len()) {
            if (va[i] - vb[i]).abs() > tol {
                return false;
            }
        }

        true
    }

    #[test]
    fn test_compute_neighbourhood_delta_matrix() {
        let problem = Problem::random_integral(3);

        problem.print();

        for mu in (1..10) {
            let perm = Permutation::random(problem.size);
            println!("perm: {:?}", perm.image);
            println!("solution: {:?}", problem.value(&perm));

            let transpositions = problem.transposition_neighbourhood(&perm);
            println!("transpositions: {:?}", transpositions);

            let deltas_very_slow = problem.compute_neighbourhood_delta_matrix_very_slow(&perm);
            let deltas = problem.compute_neighbourhood_delta_matrix(&perm);

            println!("TEST FAST METHOD:");
            println!("deltas_very_slow: {:?}", deltas_very_slow);
            println!("deltas: {:?}", deltas);
            assert!(test_vector_equality(&deltas_very_slow, &deltas, 1e-9));
        }
    }
}
