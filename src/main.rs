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

fn make_symmetric(mat : &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut result = vec!(vec!(0.0; mat[0].len()); mat.len());
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if i != j {
                result[i][j] = mat[i][j] + mat[j][i];
            } else {
                result[i][j] = mat[i][j];
            }
        }
    }
    result
}

fn main() {
    for path in glob::glob("data/qapdata/*.dat").unwrap().filter_map(Result::ok) {
    // for path in glob::glob("data/qapdata/*.dat").unwrap().filter_map(Result::ok) {
        // println!("{}", path.display());

        let path_str = path.to_string_lossy();
        let mut problem = qap::Problem::from_file(&path_str);
        // problem.print();

        let re = Regex::new(r".*/(?P<prob_name>.*).dat").unwrap();
        let sln_name = re.captures(&path_str).unwrap().name("prob_name").unwrap();
        let sln_path_str = format!("data/qapsoln/{}.sln", sln_name);
        let solution = qap::Solution::from_file(&sln_path_str);

        if solution.is_some() {
            let sln = solution.clone().unwrap();
            let mut sym_problem = problem.clone();
            sym_problem.weights = make_symmetric(&sym_problem.weights);

            let sln_value = sln.value;
            let calculated_value = problem.value(&sln.perm);
            let sym_calculated_value = sym_problem.value(&sln.perm);

            // problem.print();
            // sym_problem.print();

            if sln_value != calculated_value {
                let mut swapped_problem = problem.clone();
                swapped_problem.weights = problem.distances.clone();
                swapped_problem.distances = problem.weights.clone();
                let swapped_calculated_value = swapped_problem.value(&sln.perm);

                problem = swapped_problem;

                if sln_value != swapped_calculated_value {
                    println!("{}:", sln_name);
                    println!("Value: {:?}", sln.value);
                    println!("Value (calculated): {:?}", calculated_value);
                    println!("Value (swapped): {:?}", swapped_calculated_value);
                    // println!("Value (calculated, symmetric): {:?}", sym_calculated_value);
                    assert!(false);
                }
            }

            // assert!(sln_value == calculated_value);
        } else {
            println!("Solution unknown: {}", sln_name);
        }

        let duration = Duration::milliseconds(5000);

        // let random_search_result = random_search::solve(&problem, duration);
        // println!("random_search_result: {:?}", random_search_result);
        //
        // let basic_search_result = basic_search::solve(&problem, duration);
        // println!("basic_search_result: {:?}", basic_search_result);
        //
        // let mut sa_search_result = sa_search::solve(&problem, duration);
        // sa_search_result.instance_name = String::from(sln_name);
        // sa_search_result.best_known_solution = solution.clone();
        // sa_search_result.append_to_file("results_sa.yaml");
        // println!("sa_search_result: {:?}", sa_search_result);

        // let mut ea_search_result = ea_search::solve(&problem, duration);
        // ea_search_result.instance_name = String::from(sln_name);
        // ea_search_result.best_known_solution = solution.clone();
        // ea_search_result.append_to_file("results_ea.yaml");
        // println!("ea_search_result: {:?}", ea_search_result);

        // let mut its_search_result = iteratedtabusearch::IteratedTabuSearch::solve(&problem, duration);
        // its_search_result.instance_name = String::from(sln_name);
        // its_search_result.best_known_solution = solution.clone();
        // its_search_result.append_to_file("results_its.yaml");
        // println!("its_search_result: {:?}", its_search_result);

        // let mut bma_search_result = bma::Bma::solve(&problem, duration);
        // bma_search_result.instance_name = String::from(sln_name);
        // bma_search_result.best_known_solution = solution.clone();
        // bma_search_result.append_to_file("results_bma.yaml");
        // println!("bma_search_result: {:?}", bma_search_result);
    }
}
