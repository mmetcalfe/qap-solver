use qap;
use permutation::Permutation;
use time::SteadyTime;
use time::Duration;
use std::num;
extern crate rand;
use self::rand::Rng;

pub fn temperature(progress : f32) -> f32{
    (1.0 - progress) * 50000.0 // linear cooling schedule
}

pub fn probability(energy : f32, energy_new : f32, temp : f32) -> f32 {
    let energy_diff = energy_new - energy;
    if energy_diff < 0.0 {
        // println!("move: {}", energy_diff);
        return 1.0;
    } else {
        let prob = (-energy_diff/temp).exp();
        // println!("temp: {}, energy_diff: {}, probability: {}", temp, energy_diff, prob);
        return prob;
    }
}

pub fn duration_ratio(elapsed : Duration, duration : Duration) -> f32 {
    let elapsed_ns = elapsed.num_nanoseconds().expect("duration overflow");
    let duration_ns = duration.num_nanoseconds().expect("duration overflow");

    elapsed_ns as f32 / duration_ns as f32
}

pub fn solve(problem : &qap::Problem, duration : Duration) -> qap::Solution {
    let size = problem.size;

    let init_perm = Permutation::random(size);
    let mut soln = problem.solution(&init_perm);
    let mut best_soln = soln.clone();

    let start = SteadyTime::now();
    while SteadyTime::now() - start < duration {
        let elapsed = SteadyTime::now() - start;
        let progress = duration_ratio(elapsed, duration);

        let temp = temperature(progress);

        let transp = Permutation::random_transposition(size);
        let neighbour = soln.perm.compose(&transp);
        let neighbour_soln = problem.solution(&neighbour);

        if rand::random::<f32>() < probability(soln.value, neighbour_soln.value, temp) {
            soln = neighbour_soln;

            if soln.value < best_soln.value {
                best_soln = soln.clone();
                println!("sa_search: {}", soln.value);
            }
        }
    }

    best_soln
}
