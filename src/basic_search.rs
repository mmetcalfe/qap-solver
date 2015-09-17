use qap;
use permutation::Permutation;
// use time::SteadyTime;
use time::Duration;

pub fn solve(problem : &qap::Problem, duration : Duration) -> qap::Solution {
    let size = problem.size;

    let mut soln = Permutation::random(size);
    let mut value = problem.value(&soln);

    let mut num_steps = 0;
    loop {
        num_steps += 1;
        println!("basic_search: {}", value);

        let mut improvement = false;

        // For each neighbour:
        let transp_count = Permutation::num_transpositions(size);
        for i in (0..transp_count) {
            let transp = Permutation::indexed_transposition(size, i);
            let neighbour = soln.compose(&transp);
            let neighbour_value = problem.value(&neighbour);

            if neighbour_value < value {
                soln = neighbour;
                value = neighbour_value;
                improvement = true;
            }
        }

        if !improvement {
            break;
        }
    }
    println!("basic_search, num_steps: {}", num_steps);

    qap::Solution {
        size: size,
        value: value,
        perm: soln,
    }
}
