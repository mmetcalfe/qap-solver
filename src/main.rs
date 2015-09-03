pub mod permutation;
pub mod qap;

extern crate glob;

fn main() {
    for path in glob::glob("data/qapdata/chr12a*").unwrap().filter_map(Result::ok) {
        println!("{}", path.display());

        let path_str = path.to_string_lossy();

        let problem = qap::Qap::from_file(&path_str);

        problem.print();

        // let sln_str = path_str[..path_str.len()-3].to_string() + "sln";
        let solution = qap::Solution::from_file("data/qapsoln/chr12a.sln");

        println!("{:?}", problem);
        println!("{:?}", solution);

        println!("{:?}", problem.value(solution.perm));
    }
}
