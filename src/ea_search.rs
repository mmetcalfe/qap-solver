use permutation::Permutation;
use time::SteadyTime;
use time::Duration;
// use std::num;
extern crate rand;
use self::rand::Rng;
use std::cmp::Ordering;
use qap::Problem;
use qap::Solution;
use std::collections::HashSet;
use qap::SearchResult;
use iteratedtabusearch::IteratedTabuSearch;

fn mutate(problem : &Problem, original : &Solution, mutation_degree : usize) -> Solution {

    let mutation = IteratedTabuSearch::random_sequence(problem.size, mutation_degree);
    let mutated = IteratedTabuSearch::apply_chained_mutation(&mutation, &original.perm);
    problem.solution(&mutated)

    // let mut mutant = original.clone();
    //
    // // for i in 0..3 {
    //     let transp = Permutation::random_transposition(mutant.size);
    //     mutant.perm = mutant.perm.compose(&transp);
    // // }
    //
    // mutant.value = problem.value(&mutant.perm);
    //
    // // println!("ea_search, mutate: {}", mutant.value);
    // mutant
}

// fn crossover(problem : &Problem, parent_i : &Solution, parent_j : &Solution) -> Solution {
//     let mut rng = rand::thread_rng();
//
//     let mut new_perm = Permutation::identity(problem.size);
//
//     // Find dissimilar elements:
//     let mut candidates = Vec::with_capacity(problem.size);
//     for k in 0..problem.size {
//         let i = parent_i.perm.image[k];
//         let j = parent_j.perm.image[k];
//         if i != j {
//             candidates.push(i);
//         }
//     }
//
//     // Randomise dissimilar elements:
//     for k in 0..problem.size {
//         let i = parent_i.perm.image[k];
//         let j = parent_j.perm.image[k];
//         if i != j {
//             let remove_index = rng.gen_range(0, candidates.len());
//             let rand_elem = candidates.swap_remove(remove_index);
//             new_perm.image[k] = rand_elem;
//         } else {
//             new_perm.image[k] = i;
//         }
//     }
//
//     problem.solution(&new_perm)
// }

pub fn crossover(problem : &Problem, parent_a : &Solution, parent_b : &Solution) -> Solution {
    // Uniform crossover:
    let mut rng = rand::thread_rng();

    let mut new_perm = Permutation::identity(problem.size);

    let mut free_indices : HashSet<usize, _> = HashSet::new();
    for i in (0..problem.size) {
        free_indices.insert(i);
    }

    for k in 0..problem.size {
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

    problem.solution(&new_perm)
}

pub fn solve(problem : &Problem, duration : Duration) -> SearchResult {
    let mut rng = rand::thread_rng();
    let start = SteadyTime::now();

    let size = problem.size;

    let pop_size = 40;
    let champion_count = 10;
    let worst_count = pop_size - champion_count;
    let crossover_prob = 0.25; // crossover vs mutation
    let mutation_degree = problem.size/3;

    // Generate initial population:
    // Evaluate fitnesses:
    let mut population : Vec<Solution> = Vec::new();
    for i in 0..pop_size {
        let init_perm = Permutation::random(size);
        let soln = problem.solution(&init_perm);
        population.push(soln);
    }

    let mut best_soln = population[0].clone();

    let mut best_soln_time = SteadyTime::now();

    // While time remains:
    let mut num_steps = 0;
    while SteadyTime::now() - start < duration {
        num_steps += 1;

        // Select best individuals for reproduction:
        population.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap_or(Ordering::Less));

        let split_index = champion_count;
        let mut new_pop = Vec::with_capacity(worst_count);

        {
            let best_pop = &population[..split_index];

            // Breed new individuals through crossover and mutation:
            // Evaluate fitnesses:
            while new_pop.len() < worst_count {
                let unif = rng.gen::<f64>();
                if unif < crossover_prob {
                    // crossover:
                    let parent_i = rng.choose(best_pop).unwrap();
                    let parent_j = rng.choose(&population).unwrap();
                    let child = crossover(problem, parent_i, parent_j);
                    new_pop.push(child);
                } else {
                    // mutation:
                    let original = rng.choose(&population).unwrap();
                    let mutant = mutate(problem, original, mutation_degree);
                    new_pop.push(mutant);
                }
            }
        }

        // Replace the least fit original individuals with the new individuals:
        population.truncate(champion_count);
        population.extend(new_pop);

        // Set the current best solution:
        if population.first().unwrap().value < best_soln.value {
            best_soln = population.first().unwrap().clone();
            best_soln_time = SteadyTime::now();
            // println!("ea_search: {}", best_soln.value);
            // println!("ea_search, best_soln.value: {:?}", &population.iter().map(|ref x| x.value).collect::<Vec<_>>());
        }
    }
    println!("ea_search, num_steps: {}", num_steps);

    let search_duration = SteadyTime::now() - start;
    let best_soln_duration = best_soln_time - start;
    SearchResult::new(best_soln, search_duration, best_soln_duration)
}
