// #![feature(hashmap_hasher)]

extern crate glob;
extern crate time;
extern crate rand;
extern crate regex;

pub mod permutation;
pub mod qap;
pub mod random_search;
pub mod basic_search;
pub mod sa_search;
pub mod ea_search;
pub mod tabulist;
pub mod iteratedtabusearch;
pub mod bma;

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
            let sln = solution.clone();
            println!("{}: {:?}", sln_name, sln.unwrap());
            // println!("Value (calculated): {:?}", problem.value(&sln.perm));
        } else {
            println!("Solution unknown.");
        }

        let duration = Duration::milliseconds(5000);

        // let random_search_result = random_search::solve(&problem, duration);
        // println!("random_search_result: {:?}", random_search_result);
        //
        // let basic_search_result = basic_search::solve(&problem, duration);
        // println!("basic_search_result: {:?}", basic_search_result);
        //
        let mut sa_search_result = sa_search::solve(&problem, duration);
        sa_search_result.instance_name = String::from(sln_name);
        sa_search_result.best_known_solution = solution.clone();
        sa_search_result.append_to_file("results_sa.yaml");
        println!("sa_search_result: {:?}", sa_search_result);

        let mut ea_search_result = ea_search::solve(&problem, duration);
        ea_search_result.instance_name = String::from(sln_name);
        ea_search_result.best_known_solution = solution.clone();
        ea_search_result.append_to_file("results_ea.yaml");
        println!("ea_search_result: {:?}", ea_search_result);

        let mut its_search_result = iteratedtabusearch::IteratedTabuSearch::solve(&problem, duration);
        its_search_result.instance_name = String::from(sln_name);
        its_search_result.best_known_solution = solution.clone();
        its_search_result.append_to_file("results_its.yaml");
        println!("its_search_result: {:?}", its_search_result);

        let mut bma_search_result = bma::Bma::solve(&problem, duration);
        bma_search_result.instance_name = String::from(sln_name);
        bma_search_result.best_known_solution = solution.clone();
        bma_search_result.append_to_file("results_bma.yaml");
        println!("bma_search_result: {:?}", bma_search_result);
    }
}
