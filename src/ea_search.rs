use qap;
use permutation::Permutation;
use time::SteadyTime;
use time::Duration;
// use std::num;
extern crate rand;
use self::rand::Rng;
use std::cmp::Ordering;

fn mutate(problem : &qap::Problem, original : &qap::Solution) -> qap::Solution {

    let mut mutant = original.clone();

    // for i in 0..3 {
        let transp = Permutation::random_transposition(mutant.size);
        mutant.perm = mutant.perm.compose(&transp);
    // }

    mutant.value = problem.value(&mutant.perm);

    // println!("ea_search, mutate: {}", mutant.value);
    mutant
}

fn crossover(problem : &qap::Problem, parent_i : &qap::Solution, parent_j : &qap::Solution) -> qap::Solution {
    let mut rng = rand::thread_rng();

    let mut new_perm = Permutation::identity(problem.size);

    // Find dissimilar elements:
    let mut candidates = Vec::with_capacity(problem.size);
    for k in 0..problem.size {
        let i = parent_i.perm.image[k];
        let j = parent_j.perm.image[k];
        if i != j {
            candidates.push(i);
        }
    }

    // Randomise dissimilar elements:
    for k in 0..problem.size {
        let i = parent_i.perm.image[k];
        let j = parent_j.perm.image[k];
        if i != j {
            let remove_index = rng.gen_range(0, candidates.len());
            let rand_elem = candidates.swap_remove(remove_index);
            new_perm.image[k] = rand_elem;
        } else {
            new_perm.image[k] = i;
        }
    }

    problem.solution(&new_perm)
}

pub fn solve(problem : &qap::Problem, duration : Duration) -> qap::Solution {
    let mut rng = rand::thread_rng();

    let size = problem.size;

    let pop_size = 20;
    let champion_count = 2;
    let worst_count = pop_size - champion_count;
    let crossover_prob = 0.5;

    // Generate initial population:
    // Evaluate fitnesses:
    let mut population : Vec<qap::Solution> = Vec::new();
    for i in 0..pop_size {
        let init_perm = Permutation::random(size);
        let soln = problem.solution(&init_perm);
        population.push(soln);
    }

    let mut best_soln = population[0].clone();

    // While time remains:
    let mut num_steps = 0;
    let start = SteadyTime::now();
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
                let unif = rng.gen::<f32>();
                if unif < crossover_prob {
                    // crossover:
                    let parent_i = rng.choose(best_pop).unwrap();
                    let parent_j = rng.choose(&population).unwrap();
                    let child = crossover(problem, parent_i, parent_j);
                    new_pop.push(child);
                } else {
                    // mutation:
                    let original = rng.choose(&population).unwrap();
                    let mutant = mutate(problem, original);
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
            println!("ea_search: {}", best_soln.value);
            // println!("ea_search, best_soln.value: {:?}", &population.iter().map(|ref x| x.value).collect::<Vec<_>>());
        }
    }
    println!("ea_search, num_steps: {}", num_steps);

    best_soln
}
