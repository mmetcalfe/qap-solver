extern crate rand;
// extern crate core;
use self::rand::Rng;
use std::cmp;
use std::num;
use std::io::prelude::*;
use std::io;
use std::usize;
use std::f64;
use tabulist::TabuList;
use permutation::Permutation;
use qap::Problem;
use qap::Solution;
use qap::SearchResult;
use time::SteadyTime;
use time::Duration;
use std::cmp::Ordering;
use std::collections::HashSet;
use iteratedtabusearch::IteratedTabuSearch;

#[derive(Debug, Clone)]
pub struct Bma {
    pub problem : Problem,
    pub pop_size : usize, // |P|
    pub num_bls_iterations_short : usize, // t_s
    pub num_bls_iterations_long : usize, // t_l
    pub mu_min : usize, // μ_min
    pub mu_increment : usize, // m
    pub tournament_pool_size : usize, // λ
    pub mutation_non_improvement_iterations : usize, // ν
    pub initial_jump_magnitude : usize, // L_0
    pub tabu_tenure_range : (usize, usize), // γ
    pub min_bls_peturb_prob : f64, // Q
    pub max_non_improvement_count : usize, // T
    pub random_vs_recency_prob : f64, // T
}

impl Bma {
    // Create a new ITS instance with the parameters reccomended by Misevicius.
    pub fn new(problem : Problem) -> Bma {
        let size = problem.size;
        let pop_size = 15;
        Bma {
            problem: problem,
            pop_size: pop_size,
            num_bls_iterations_short: cmp::max(1, (256*1)/size), // 50, // 5000
            num_bls_iterations_long:  cmp::max(1, (256*2)/size), // 100, // 10000
            mu_min: size / 2,
            mu_increment: cmp::max(1, size/10),
            tournament_pool_size: 4,
            mutation_non_improvement_iterations: pop_size,
            initial_jump_magnitude: cmp::max(1, size/10), // 0.05n for T:1,2 and 0.15n for T:3,4
            tabu_tenure_range: ((9*size)/10, (11*size)/10),
            min_bls_peturb_prob: 0.75,
            max_non_improvement_count: 25, // 2500
            random_vs_recency_prob: 0.7,
        }
    }

    pub fn solve(problem : &Problem, duration : Duration) -> SearchResult {
        let bma = Bma::new(problem.clone());
        bma.search(duration)
    }

    pub fn search(&self, duration : Duration) -> SearchResult {
        // Randomly generate initial population:
        println!("Bma: generating initial population...");
        let start = SteadyTime::now();

        let mut population : Vec<Solution> = Vec::new();
        for i in 0..self.pop_size {
            print!("{}, ", i); io::stdout().flush().ok().expect("Could not flush stdout");
            // println!("initial {}, ", i);

            let init_perm = Permutation::random(self.problem.size);
            // Improve each individual with t_s iterations of BLS:
            let soln = self.problem.solution(&init_perm);
            let improved = self.breakout_local_search(soln, self.num_bls_iterations_short);
            population.push(improved);
        }
        println!("");

        // Initialise the best individual:
        population.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap_or(Ordering::Less));
        let mut best_solution = population[0].clone();

        let mut best_soln_time = SteadyTime::now();
        let mut last_best_value = best_solution.value;

        // Initialise the current mutation degree:
        let mut mutation_degree = self.mu_min;

        let mut num_steps = 0;
        let mut last_improvement = 0;
        while SteadyTime::now() - start < duration {
            let iteration_number = num_steps;
            num_steps += 1;

            // Select a subset of parent individuals:
            let parent_a = self.tournament_selection(&population);
            let parent_b = self.tournament_selection(&population);

            // Generate an offspring:
            let offspring = self.crossover(&population[parent_a], &population[parent_b]);

            // Improve offspring with long BLS run:
            let candidate_solution = self.breakout_local_search(offspring, self.num_bls_iterations_long);

            // Insert offspring into population according to replacement strategy:
            // population = self.replacement_strategy(&mut population, &candidate_solution);
            self.replacement_strategy(&mut population, &candidate_solution);

            if iteration_number - last_improvement > self.mutation_non_improvement_iterations {
                // Mutate the population:
                // population = self.mutate_population(&mut population, mutation_degree);
                self.mutate_population(&mut population, mutation_degree);

                // Improve each individual using a short BLS runs:
                population = population.iter().map(|individual| {
                    self.breakout_local_search(individual.clone(), self.num_bls_iterations_short)
                }).collect();

                // Update the best individual:
                population.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap_or(Ordering::Less));
                if population[0].value < best_solution.value {
                    best_solution = population[0].clone();
                    last_improvement = iteration_number; // TODO: Check if this should be reset.
                }

                // println!("Bma: mutating population, iteration: {}, value: {}", iteration_number, best_solution.value);

                // Increase the mutation degree:
                mutation_degree += self.mu_increment;
            }

            if best_solution.value > candidate_solution.value || mutation_degree > self.problem.size {
                // Reset mutation degree to default:
                mutation_degree = self.mu_min;
            }

            if best_solution.value > candidate_solution.value {
                best_solution = candidate_solution;
                last_improvement = iteration_number;
                println!("Bma: iteration: {}, value: {}", iteration_number, best_solution.value);
            }

            if best_solution.value < last_best_value {
                last_best_value = best_solution.value;
                best_soln_time = SteadyTime::now();
            }
        }

        println!("Bma, total_time: {}", SteadyTime::now() - start);
        println!("Bma, num_steps: {}", num_steps);

        let search_duration = SteadyTime::now() - start;
        let best_soln_duration = best_soln_time - start;
        SearchResult::new(best_solution, search_duration, best_soln_duration)
    }

    pub fn steepest_descent_search(&self, initial : Solution, delta_matrix : &mut Vec<f64>, tabu_list : &mut TabuList, current_iter : &mut usize) -> Solution {
        // println!("Bma, steepest_descent_search");

        let mut rng = rand::thread_rng();
        let mut solution = initial.clone();

        loop {
            let M = initial.size*(initial.size-1)/2;
            // Find the best improving move:
            let mut best_index = 0;
            let mut best_delta = 0.0;
            for m in 1..M {
                // TODO: Use a built-in find method.
                if delta_matrix[m] < best_delta {
                    best_index = m;
                    best_delta = delta_matrix[m];
                }
            }

            // Stop the search if no improving move is found:
            if best_delta == 0.0 {
                break;
            }
            // println!("Bma, steepest_descent_search: delta = {}", best_delta);

            // Perform the move:
            let best_move = Permutation::triangle_index(best_index);
            solution.perm = Permutation::apply_transposition(solution.perm, best_move);
            solution.value += delta_matrix[best_index];

            // Update the tabu list:
            let tenure = rng.gen_range(self.tabu_tenure_range.0, self.tabu_tenure_range.1);
            tabu_list.make_tabu(best_move, *current_iter + tenure);

            // Update the delta matrix:
            *delta_matrix = self.problem.compute_neighbourhood_delta_matrix(&solution.perm);

            *current_iter += 1;
        }

        solution
    }

    pub fn breakout_local_search(&self, initial : Solution, num_iterations : usize) -> Solution {
        // println!("Bma, breakout_local_search: {}", num_iterations);

        let mut rng = rand::thread_rng();
        // rng.gen::<f64>() <
        // min_bls_peturb_prob

        let mut tabu_list = TabuList::new(self.problem.size);

        let mut candidate = initial.clone();

        // Compute the initial matrix of delta move gains:
        let mut delta_matrix = self.problem.compute_neighbourhood_delta_matrix(&candidate.perm);

        let mut best_solution = candidate.clone();

        let mut last_descent_best_value = best_solution.value; // c_p
        let mut non_improvement_count = 0; // ω

        // TODO: Find the BMA value/name for T.
        let max_jump_magnitude = ((4*initial.size)/10, (6*initial.size)/10); // L_max

        let mut jump_magnitude = self.initial_jump_magnitude; // L

        let mut current_iter = 0;

        for i in 0..num_iterations {
            // println!("Bma, breakout_local_search: iteration {}", i);

            candidate = self.steepest_descent_search(candidate, &mut delta_matrix, &mut tabu_list, &mut current_iter);

            if candidate.value < best_solution.value {
                // Update best recorded solution.
                best_solution = candidate.clone();
                // Reset non-improvement counter.
                non_improvement_count = 0;
            } else if candidate.value != last_descent_best_value {
                non_improvement_count += 1;
            }

            // Determine the perturbation strength:
            if non_improvement_count > self.max_non_improvement_count {
                // Search seems to be stagnating, set L to large value:
                jump_magnitude = rng.gen_range(max_jump_magnitude.0, max_jump_magnitude.1);
                non_improvement_count = 0;
            } else if (candidate.value - last_descent_best_value).abs() < 1e-5 {
                // Search returned to the previous local optimum, increase jump magnitude by one:
                jump_magnitude += 1;
            } else {
                // Search escaped from the previous local optimum, reinitialize jump magnitude:
                jump_magnitude = self.initial_jump_magnitude;
            }

            // Update the current objective value of the previous local optimum:
            last_descent_best_value = candidate.value;

            // Peturb the current local optimum with L perturbation moves:
            candidate = self.perturbation(candidate, jump_magnitude, &mut tabu_list, &mut current_iter, &mut delta_matrix, &mut non_improvement_count, &mut best_solution);
        }

        best_solution
    }

    pub fn perturbation(&self, candidate : Solution, jump_magnitude : usize, tabu_list : &mut TabuList,
                       current_iter : &mut usize, delta_matrix : &mut Vec<f64>,
                       non_improvement_count : &mut usize, best_solution : &mut Solution) -> Solution {
    // println!("Bma, perturbation: jump_magnitude {}", jump_magnitude);

       let mut rng = rand::thread_rng();
       // rng.gen::<f64>() <

       // Determine probability P:
       let expTerm = (-(*non_improvement_count as f64)/(self.max_non_improvement_count as f64)).exp();
       let prob = if self.min_bls_peturb_prob > expTerm {
           self.min_bls_peturb_prob
       } else {
           expTerm
       };

       let rand_val = rng.gen::<f64>();
       if rand_val < prob {
           // Apply directed perturbation:
           return self.perturb_with(
               candidate, jump_magnitude, tabu_list, current_iter,
               delta_matrix, non_improvement_count, best_solution,
               &Bma::generate_move_index_tabu_search
         );

    //    } else if rand_val < prob + (1-prob)*random_vs_recency_prob {
    //        // Apply recency-based perturbation:

       } else {
           // Apply random perturbation:
           return self.perturb_with(
               candidate, jump_magnitude, tabu_list, current_iter,
               delta_matrix, non_improvement_count, best_solution,
               &Bma::generate_move_random
           );
       }
    }

    fn perturb_with(&self, candidate : Solution, jump_magnitude : usize, tabu_list : &mut TabuList,
                   current_iter : &mut usize, delta_matrix : &mut Vec<f64>,
                   non_improvement_count : &mut usize, best_solution : &mut Solution,
                   move_gen_func : &Fn(&Bma, f64, &Vec<f64>, &TabuList, usize) -> usize
       ) -> Solution {
        let mut rng = rand::thread_rng();
        let mut perturbed = candidate.clone();

        for i in (0..jump_magnitude) {
            let best_solution_delta = best_solution.value - perturbed.value;
            let move_index = move_gen_func(&self, best_solution_delta, delta_matrix, tabu_list, *current_iter);

            let perturbation_move = Permutation::triangle_index(move_index); // TODO: Replace with decoder.
            perturbed.perm = Permutation::apply_transposition(perturbed.perm, perturbation_move);
            perturbed.value += delta_matrix[move_index];

            let tenure = rng.gen_range(self.tabu_tenure_range.0, self.tabu_tenure_range.1);
            tabu_list.make_tabu(perturbation_move, *current_iter + tenure);

            // Update the delta matrix:
            *delta_matrix = self.problem.compute_neighbourhood_delta_matrix(&perturbed.perm);

            *current_iter += 1;

            if perturbed.value < best_solution.value {
                // Update best recorded solution.
                *best_solution = perturbed.clone();

                // Reset non-improvement counter.
                *non_improvement_count = 0;
            }
        }

        perturbed
    }

    pub fn generate_move_random(&self, best_solution_delta : f64, delta_matrix : &Vec<f64>, tabu_list : &TabuList, current_iter : usize) -> usize {
        let mut rng = rand::thread_rng();
        let M = self.problem.size*(self.problem.size-1)/2;
        let m = rng.gen_range(0, M);
        m
    }

    pub fn generate_move_index_tabu_search(&self, best_solution_delta : f64, delta_matrix : &Vec<f64>, tabu_list : &TabuList, current_iter : usize) -> usize {
        let mut rng = rand::thread_rng();

        // Start from a random position:
        let M = self.problem.size*(self.problem.size-1)/2;
        let mut m = rng.gen_range(0, M);
        let mut index_count = 0;

        // Find the best improving move:
        let mut best_index = 0;
        let mut best_delta = f64::INFINITY;
        let mut best_non_tabu_index = 0;
        let mut best_non_tabu_delta = f64::INFINITY;
        loop {
            // TODO: Use a built-in find method.
            if delta_matrix[m] < best_delta {
                best_index = m;
                best_delta = delta_matrix[m];
            }

            let m_move = Permutation::triangle_index(m); // TODO: Replace with decoder.
            if !tabu_list.is_tabu(m_move, current_iter) && delta_matrix[m] < best_non_tabu_delta {
                best_non_tabu_index = m;
                best_non_tabu_delta = delta_matrix[m];
            }

            // Update the move index:
            m = (m + 1) % M;
            index_count += 1;
            if index_count == M {
                break;
            }
        }

        // If the best move is better than the global optimum, choose the best move:
        if best_delta < best_solution_delta {
            return best_index;
        }

        // Choose a non-tabu move if possible:
        if best_non_tabu_delta.is_finite() && !(best_non_tabu_delta < best_solution_delta) {
            return best_non_tabu_index;
        }

        // If all moves are tabu, choose the best move:
        return best_index;
    }

    pub fn tournament_selection(&self, population : &Vec<Solution>) -> usize {
        let mut rng = rand::thread_rng();

        let mut best_index = 0;
        let mut best_value = f64::INFINITY;

        let sample = rand::sample(&mut rng, 0..population.len(), self.tournament_pool_size);
        for i in sample {
            if population[i].value < best_value {
                best_index = i;
                best_value = population[i].value;
            }
        }

        best_index
    }

    pub fn crossover(&self, parent_a : &Solution, parent_b : &Solution) -> Solution {
        // Uniform crossover:
        let mut rng = rand::thread_rng();

        let mut new_perm = Permutation::identity(self.problem.size);

        let mut free_indices : HashSet<usize, _> = HashSet::new();
        for i in (0..self.problem.size) {
            free_indices.insert(i);
        }

        for k in 0..self.problem.size {
            let i = parent_a.perm.image[k] as usize;
            let j = parent_b.perm.image[k] as usize;
            let p = rng.gen::<f64>() < 0.5;

            if free_indices.contains(&i) && free_indices.contains(&j) {
                // If both are free, choose randomly between them:
                new_perm.image[k] = if p { i } else { j } as u32;
            } else if free_indices.contains(&i) {
                // If i is free, use i:
                new_perm.image[k] = i as u32;
            } else if free_indices.contains(&j) {
                // If j is free, use j:
                new_perm.image[k] = j as u32;
            } else {
                // If neither is free, choose randomly from the free indices:
                let sample = rand::sample(&mut rng, free_indices.iter(), 1);
                new_perm.image[k] = *sample[0] as u32;
            }

            let new_index = new_perm.image[k] as usize;
            free_indices.remove(&new_index);
        }

        self.problem.solution(&new_perm)
    }

    pub fn replacement_strategy(&self, population : &mut Vec<Solution>, new_soln : &Solution) {
        let mut worst_index = 0;
        let mut worst_value = f64::NEG_INFINITY;
        for i in 0..population.len() {
            let curr_soln = &population[i];
            let dist = Permutation::hamming_distance(&curr_soln.perm, &new_soln.perm);
            if dist == 0 {
                return;
            }

            if curr_soln.value > worst_value {
                worst_value = curr_soln.value;
                worst_index = i;
            }
        }

        if new_soln.value < worst_value {
            population[worst_index] = new_soln.clone();
        }
    }

    pub fn mutate_population(&self, population : &mut Vec<Solution>, mutation_degree : usize) {
        for i in 0..population.len() {
            let mutation = IteratedTabuSearch::random_sequence(self.problem.size, mutation_degree);
            let mutated = IteratedTabuSearch::apply_chained_mutation(&mutation, &population[i].perm);
            population[i] = self.problem.solution(&mutated);
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use permutation::Permutation;
//     use *;
//
//     #[test]
//     fn test_decoder() {
//         let decoder = Decoder::new(26);
//
//         let uv = decoder.decode(3);
//         println!("uv: {:?}", uv);
//         assert!(uv == (1, 1));
//     }
// }
