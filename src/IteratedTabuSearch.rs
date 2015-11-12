extern crate rand;
use self::rand::Rng;
use std::cmp;
use std::num;
use std::f64;
use tabulist::TabuList;
use permutation::Permutation;
use qap::Problem;
use qap::Solution;
use qap::SearchResult;
use time::SteadyTime;
use time::Duration;
// extern crate collections;
// use self::collections::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub struct Decoder {
    pub decode1 : Vec<usize>,
    pub decode2 : Vec<usize>,
}

impl Decoder {
    fn new(size : usize) -> Decoder {
        let M = size*(size-1)/2;
        let mut decode1 = vec!(0; M);
        let mut decode2 = vec!(0; M);
        // println!("Decoder::new: {:?}", size);
        // println!("Decoder::new: {:?}", M);

        let mut k = 0;
        for i in (0..size) {
            for j in (0..i) {
                // println!("Decoder::new: {:?}", (i,j));
                // println!("Decoder::new: {:?}", k);

                decode1[k] = i;
                decode2[k] = j;
                k = k + 1;
            }
        }

        Decoder {
            decode1: decode1,
            decode2: decode2,
        }
    }

    fn decode(&self, m : usize) -> (usize, usize) {
        // println!("Decoder::decode, len: {:?}", (self.decode1.len()));
        // println!("Decoder::decode, m: {:?}", m);
        (self.decode1[m], self.decode2[m])
    }
}

#[derive(Debug, Clone)]
pub struct IteratedTabuSearch {
    pub problem : Problem,
    pub search_extensity : usize, // Q
    pub search_intensity : usize, // W
    pub mutation_rate_range : (usize, usize), // [μ_min, μ_max]
    pub tabu_tenure_range : (usize, usize), // [h_low, h_high]
    pub randomisation_level : f64, // α
    pub intra_intensification_interval : usize, // I1
    pub inter_intensification_interval : usize, // I2
    pub num_mutation_trials : usize, // η
    pub disruptiveness_control : usize, // λ
    // pub tabu_list : TabuList,
}

impl IteratedTabuSearch {
    // Create a new ITS instance with the parameters reccomended by Misevicius.
    pub fn new(problem : Problem) -> IteratedTabuSearch {
        let lifelike = false;
        let size = problem.size;
        IteratedTabuSearch {
            problem: problem,
            search_extensity: if lifelike {5*size} else {25*size},
            search_intensity: if lifelike {size*size} else {size},
            mutation_rate_range: ((3*size)/10, (5*size)/10),
            tabu_tenure_range: if size > 50 {((2*size)/10, (4*size)/10)} else {((1*size)/10, (2*size)/10)},
            randomisation_level: 0.05,
            intra_intensification_interval: 2*size,
            inter_intensification_interval: (5*size)/10,
            num_mutation_trials: (3*size)/10,
            disruptiveness_control: 10,
            // tabu_list: TabuList::new(size),
        }
    }

    pub fn solve(problem : &Problem, duration : Duration) -> SearchResult {
        let its = IteratedTabuSearch::new(problem.clone());
        its.search(duration)
    }

    pub fn search(&self, duration : Duration) -> SearchResult {
        let start = SteadyTime::now();

        let initial_perm = Permutation::random(self.problem.size);
        let initial = self.problem.solution(&initial_perm);
        let mut best_solution = initial.clone();

        let mut best_soln_time = SteadyTime::now();

        // Preliminary improvement of the initial solution:
        let mut candidate = self.improved_robust_tabu_search(initial, self.search_intensity);

        let mut mu = self.mutation_rate_range.0 - 1;

        let mut num_steps = 0;
        while SteadyTime::now() - start < duration {
            num_steps += 1;
        // for q in (1..self.search_extensity) {
            // Update the mutation rate:
            if mu < self.mutation_rate_range.1 {
                mu = mu + 1;
            } else {
                mu = self.mutation_rate_range.0;
            }

            let mutant = self.controlled_chained_mutation(&candidate.perm, mu);
            let mutant_solution = self.problem.solution(&mutant);
            candidate = self.improved_robust_tabu_search(mutant_solution, self.search_intensity);

            // let candidate_solution = self.problem.solution(&candidate);
            if candidate.value < best_solution.value {
                // Save the best solution so far:
                best_solution = candidate.clone();
                best_soln_time = SteadyTime::now();

                // Reset the mutation rate:
                mu = self.mutation_rate_range.0 - 1;

                println!("IteratedTabuSearch: {}", best_solution.value);
            }
        }

        println!("IteratedTabuSearch, total_time: {}", SteadyTime::now() - start);
        println!("IteratedTabuSearch, num_steps: {}", num_steps);

        let search_duration = SteadyTime::now() - start;
        let best_soln_duration = best_soln_time - start;
        SearchResult::new(best_solution, search_duration, best_soln_duration)
    }

    pub fn improved_robust_tabu_search(&self, initial : Solution, search_intensity : usize) -> Solution {
        // println!("IteratedTabuSearch: improved_robust_tabu_search");
        let mut rng = rand::thread_rng();

        // Initialise the tabu list:
        let mut tabu_list = TabuList::new(self.problem.size);
        let decoder = Decoder::new(self.problem.size);

        let mut best_solution = initial.clone(); //self.problem.solution(&initial);
        let mut candidate = initial;
        // let mut candidate_solution = self.problem.solution(&candidate);

        // Calculate differences:
        // let d = |ref perm, uv| self.solution_value_difference(&perm, decoder.decode(uv));

        // Intialise variables:
        let n = self.problem.size;
        let M = n*(n-1)/2; // Number of objective function differences.
        tabu_list.tabu_tenure = self.tabu_tenure_range.0;
        let mut wp = 0;
        let mut m_old = 0;
        let threshold = 3*n;

        let mut mp = 0;

        for w in (0..search_intensity) {
            let mut d_min = f64::INFINITY;
            // Find best neighbouring permutation:
            for m in (0..M) {
                // let candidate_diff = d(&candidate, m);
                let candidate_diff = self.problem.solution_value_difference(&candidate.perm, decoder.decode(m));
                if w % self.intra_intensification_interval != 0 {
                    let tabu_criterion = tabu_list.is_tabu(decoder.decode(m), w) && rng.gen::<f64>() >= self.randomisation_level;
                    let aspiration_criterion = candidate.value + candidate_diff < best_solution.value && tabu_criterion;
                    if (candidate_diff < d_min && !tabu_criterion) || aspiration_criterion {
                        d_min = candidate_diff;
                        mp = m;
                    }
                } else if candidate_diff < d_min && m != m_old {
                    d_min = candidate_diff;
                    mp = m;
                }
            }

            if d_min < f64::INFINITY {
                let uv = decoder.decode(mp);

                // TODO: Update differences.

                // Replace current solution with the new one:
                candidate.perm = Permutation::apply_transposition(candidate.perm, uv);
                candidate.value += d_min;

                // Add the move uv to the tabu list:
                let h = tabu_list.tabu_tenure;
                tabu_list.make_tabu(uv, w + h);
                tabu_list.tabu_tenure = cmp::max(self.tabu_tenure_range.0, (tabu_list.tabu_tenure % self.tabu_tenure_range.1) + 1);

                if d_min < 0.0 && w - wp > self.inter_intensification_interval {
                    // println!("IteratedTabuSearch: uv:{:?}, dmin:{}, w:{}, wp:{}, int:{}", uv, d_min, w, wp, self.inter_intensification_interval);
                    candidate = self.fast_steepest_descent(candidate, &decoder, &mut tabu_list, w);
                    wp = w;
                }
                //  else {
                //     println!("IteratedTabuSearch: uv:{:?}", uv);
                // }

                m_old = mp;

                // Save the best solution:
                // candidate_solution = self.problem.solution(&candidate); // TODO: Use the difference instead.
                if candidate.value < best_solution.value {
                    // best_solution = candidate_solution.clone(); // TODO: Remove clone.
                    // best_solution = self.problem.solution(&candidate); // TODO: Remove clone.
                    best_solution = candidate.clone(); //self.problem.solution(&candidate); // TODO: Remove clone.

                    // Correct the tabu list:
                    if w > threshold {
                        let h = tabu_list.tabu_tenure;
                        tabu_list.subtract_from_all_counts(h/2);
                    }
                }
            }
        }

        // best_solution.perm
        best_solution
    }

    pub fn fast_steepest_descent(&self, initial : Solution, decoder : &Decoder, tabu_list : &mut TabuList, iteration : usize) -> Solution {
        // println!("IteratedTabuSearch: fast_steepest_descent");

        let M = self.problem.size*(self.problem.size-1)/2; // Number of objective function differences.

        let mut candidate = initial;

        loop {
            let mut d_min = 0.0;
            let mut mp = 0;

            for m in (0..M) {
                let candidate_diff = self.problem.solution_value_difference(&candidate.perm, decoder.decode(m));
                if candidate_diff < d_min {
                    d_min = candidate_diff;
                    mp = m;
                }
            }

            if d_min < 0.0 {
                // TODO: Update differences.
                let uv = decoder.decode(mp);
                candidate.perm = Permutation::apply_transposition(candidate.perm, uv);
                candidate.value += d_min;
                let h = tabu_list.tabu_tenure;
                tabu_list.make_tabu(uv, iteration + h);
                tabu_list.tabu_tenure = cmp::max(self.tabu_tenure_range.0, (h % self.tabu_tenure_range.1) + 1);
            }

            if d_min == 0.0 {
                break;
            }
        }

        candidate
    }

    pub fn random_sequence(size : usize, mu : usize) -> Vec<u32> {
        let mut rng = rand::thread_rng();

        // produce a starting sequence of integers [0, 1, 2, ..., n-1]
        let mut numbers : Vec<u32> = (0..size as u32).collect();
        // rng.shuffle(&mut numbers[..]);

        for j in (0..mu) {
            let k = rng.gen_range(j, size);

            if j != k {
                // numbers.as_mut_slice().swap(j, k);
                let tmp = numbers[j];
                numbers[j] = numbers[k];
                numbers[k] = tmp;
            }
        }

        numbers.truncate(mu);

        numbers
    }

    pub fn apply_chained_mutation(mutation : &Vec<u32>, perm : &Permutation) -> Permutation {
        let mut result = perm.clone();

        for i in (0..mutation.len()-1) {
            let j = i+1;
            let tmp = result.image[j];
            result.image[j] = result.image[i];
            result.image[i] = tmp;
        }

        result
    }

    pub fn disruptiveness(seq : &Vec<u32>) -> u32 {
        let mut zeta = 0;
        for win in seq.windows(2) {
            let a : i32 = win[0] as i32;
            let b : i32 = win[1] as i32;
            zeta += (a - b).abs() as u32;
        }
        zeta
    }

    pub fn disruptive_sequence(size : usize, mu : usize, lambda : usize) -> Vec<u32> {
        let mut best_seq = IteratedTabuSearch::random_sequence(size, mu);
        let mut best_zeta = IteratedTabuSearch::disruptiveness(&best_seq);
        for i in (0..lambda - 1) {
            let seq = IteratedTabuSearch::random_sequence(size, mu);
            let zeta = IteratedTabuSearch::disruptiveness(&seq);
            if zeta > best_zeta {
                best_seq = seq;
                best_zeta = zeta;
            }
        }
        best_seq
    }

    pub fn controlled_chained_mutation(&self, perm : &Permutation, mu : usize) -> Permutation {
        // println!("IteratedTabuSearch: controlled_chained_mutation");
        let mut best_mutant = perm.clone();
        let mut best_value = f64::INFINITY;
        for l in (0..self.num_mutation_trials) {
            // Generate a disruptive random sequence:
            let seq = IteratedTabuSearch::disruptive_sequence(self.problem.size, mu, self.disruptiveness_control);
            let mutant = IteratedTabuSearch::apply_chained_mutation(&seq, &perm);
            let mutant_value = self.problem.value(&mutant);

            if mutant_value < best_value {
                best_mutant = mutant;
                best_value = mutant_value;
            }
        }

        best_mutant
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use permutation::Permutation;
    use qap::*;

    #[test]
    fn test_apply_chained_mutation() {
        let problem = Problem::random(50);
        let mut its = IteratedTabuSearch::new(problem);

        for mu in (2..its.problem.size) {
            println!("mu: {}", mu);
            for i in (0..100) {
                let perm = Permutation::random(its.problem.size);
                println!("p: {:?}", perm.image);

                let mutation = IteratedTabuSearch::random_sequence(its.problem.size, mu);
                println!("m: {:?}", mutation);

                let result = IteratedTabuSearch::apply_chained_mutation(&mutation, &perm);
                println!("r: {:?}", result.image);

                let dist = IteratedTabuSearch::hamming_distance(&perm.image, &result.image);
                println!("dist: {}", dist);

                assert!(dist == mu);
            }
        }
    }

    #[test]
    fn test_decoder() {
        let decoder = Decoder::new(5);

        let uv = decoder.decode(0);
        println!("uv: {:?}", uv);
        assert!(uv == (1, 0));

        let uv = decoder.decode(7);
        println!("uv: {:?}", uv);
        assert!(uv == (4, 1));
    }

}
