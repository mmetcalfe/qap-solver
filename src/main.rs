
pub mod permutation;
pub mod qap;
pub mod random_search;
pub mod basic_search;
pub mod sa_search;
pub mod ea_search;
pub mod tabulist;

extern crate glob;
extern crate time;
extern crate rand;
extern crate regex;

use time::Duration;
use regex::Regex;

fn main() {
    for path in glob::glob("data/qapdata/*.dat").unwrap().filter_map(Result::ok) {
        println!("{}", path.display());

        let path_str = path.to_string_lossy();
        let problem = qap::Problem::from_file(&path_str);
        // problem.print();

        let re = Regex::new(r".*/(?P<prob_name>.*).dat").unwrap();
        let sln_name = re.captures(&path_str).unwrap().name("prob_name").unwrap();
        let sln_path_str = format!("data/qapsoln/{}.sln", sln_name);
        let solution = qap::Solution::from_file(&sln_path_str);

        if solution.is_some() {
            let sln = solution.unwrap();
            // println!("{:?}", sln);
            // println!("Value (calculated): {:?}", problem.value(&sln.perm));
        } else {
            println!("Solution unknown.");
        }

        let duration = Duration::milliseconds(100);

        // let random_search_result = random_search::solve(&problem, duration);
        // println!("random_search_result: {:?}", random_search_result);
        //
        // let basic_search_result = basic_search::solve(&problem, duration);
        // println!("basic_search_result: {:?}", basic_search_result);
        //
        // let sa_search_result = sa_search::solve(&problem, duration);
        // println!("sa_search_result: {:?}", sa_search_result);

        let ea_search_result = ea_search::solve(&problem, duration);
        println!("ea_search_result: {:?}", ea_search_result);
    }
}
