// extern crate qap_solver;
//
// use qap_solver::permutation::Permutation;
//
// #[test]
// fn compose_with_identity() {
//     let id = Permutation::from_image(vec!(0, 1, 2, 3, 4));
//     let p1 = Permutation::from_image(vec!(1, 0, 2, 4, 3));
//     let r1 = id.compose(&p1);
//     println!("r1: {:?}", r1.image);
//     println!("p1: {:?}", p1.image);
//     assert!(r1.image == p1.image);
// }
