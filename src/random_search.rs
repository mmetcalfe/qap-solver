use qap;
use permutation::Permutation;
use time::SteadyTime;
use time::Duration;

pub fn solve(problem : &qap::Problem, duration : Duration) -> qap::Solution {
    let size = problem.size;

    let mut soln = Permutation::random(size);
    let mut value = problem.value(&soln);

    let mut num_steps = 0;
    let start = SteadyTime::now();
    while SteadyTime::now() - start < duration {
        num_steps += 1;

        let perm = Permutation::random(size);
        let perm_val = problem.value(&perm);

        if perm_val < value {
            soln = perm;
            value = perm_val;

            println!("random_search: {}", value);
        }
    }
    println!("random_search, num_steps: {}", num_steps);

    qap::Solution {
        size: size,
        value: value,
        perm: soln,
    }
}
