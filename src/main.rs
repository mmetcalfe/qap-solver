pub mod permutation;
pub mod qap;
pub mod random_search;
pub mod basic_search;
pub mod sa_search;

extern crate glob;
extern crate time;
extern crate rand;

use time::Duration;

fn main() {
    for path in glob::glob("data/qapdata/chr12a*").unwrap().filter_map(Result::ok) {
        println!("{}", path.display());

        let path_str = path.to_string_lossy();
        let problem = qap::Problem::from_file(&path_str);
        // problem.print();

        // let sln_str = path_str[..path_str.len()-3].to_string() + "sln";
        let solution = qap::Solution::from_file("data/qapsoln/chr12a.sln");
        println!("{:?}", solution);
        println!("Value (calculated): {:?}", problem.value(&solution.perm));

        let duration = Duration::seconds(5);

        let random_search_result = random_search::solve(&problem, duration);
        println!("random_search_result: {:?}", random_search_result);

        let basic_search_result = basic_search::solve(&problem, duration);
        println!("basic_search_result: {:?}", basic_search_result);

        let sa_search_result = sa_search::solve(&problem, duration);
        println!("sa_search_result: {:?}", sa_search_result);

    }
}
