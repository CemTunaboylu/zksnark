// #[path="../tests/unit_tests.rs"] mod unit_tests;
// pub mod qap_creator;
// mod unit_tests;

#[path="./qap_creator.rs"] pub mod qap;
use crate::qap::qap::r1cs_to_qap;
use crate::qap::qap::solution_poly;


fn main() {
    let a : Vec<Vec<f64>> = [vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
                             vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
                             vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0],
                             vec![5.0, 0.0, 0.0, 0.0, 0.0, 1.0]].to_vec();

    let b : Vec<Vec<f64>> = [vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
                             vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
                             vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                             vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0]].to_vec();

    let c : Vec<Vec<f64>> = [vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
                             vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
                             vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
                             vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0]].to_vec();

    let (a_qap, b_qap, c_qap) = r1cs_to_qap(&a, &b, &c);

    let answer = vec![1.0, 3.0, 35.0, 9.0, 27.0, 30.0];


    solution_poly(answer, a_qap, b_qap, c_qap);

    let correct_r1cs : Vec<Vec<f64>> = [
        vec![-5.0, 9.166666666666666, -5.0, 0.8333333333333334],
        vec![8.0, -11.333333333333332, 5.0, -0.6666666666666666],
        vec![0.0, 0.0, 0.0, 0.0],
        vec![-6.0, 9.5, -4.0, 0.5],
        vec![4.0, -7.0, 3.5, -0.5],
        vec![-1.0, 1.8333333333333333, -1.0, 0.16666666666666666]
        ].to_vec();

}

