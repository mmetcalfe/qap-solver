use qap::Problem;
use qap::Solution;
use qap::SearchResult;
use permutation::Permutation;
use time::SteadyTime;
use time::Duration;
extern crate rand;
// use self::rand::Rng;

pub fn temperature(progress : f64) -> f64{
    (1.0 - progress) * 50000.0 // linear cooling schedule
}

pub fn probability(energy : f64, energy_new : f64, temp : f64) -> f64 {
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

pub fn duration_ratio(elapsed : Duration, duration : Duration) -> f64 {
    let elapsed_ns = elapsed.num_nanoseconds().expect("duration overflow");
    let duration_ns = duration.num_nanoseconds().expect("duration overflow");

    elapsed_ns as f64 / duration_ns as f64
}

pub fn solve(problem : &Problem, duration : Duration) -> SearchResult {
    let start = SteadyTime::now();
    let size = problem.size;

    let init_perm = Permutation::random(size);
    let mut soln = problem.solution(&init_perm);
    let mut best_soln = soln.clone();

    let mut best_soln_time = SteadyTime::now();

    let mut num_steps = 0;
    while SteadyTime::now() - start < duration {
        num_steps += 1;

        let elapsed = SteadyTime::now() - start;
        let progress = duration_ratio(elapsed, duration);

        let temp = temperature(progress);

        let transp = Permutation::random_transposition(size);
        let neighbour = soln.perm.compose(&transp);
        let neighbour_soln = problem.solution(&neighbour);

        if rand::random::<f64>() < probability(soln.value, neighbour_soln.value, temp) {
            soln = neighbour_soln;

            if soln.value < best_soln.value {
                best_soln = soln.clone();
                best_soln_time = SteadyTime::now();
                // println!("sa_search: {}", soln.value);
            }
        }
    }
    println!("sa_search, num_steps: {}", num_steps);

    let search_duration = SteadyTime::now() - start;
    let best_soln_duration = best_soln_time - start;
    SearchResult::new(best_soln, search_duration, best_soln_duration)
}
