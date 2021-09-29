// #[path="../tests/unit_tests.rs"] mod unit_tests;
// pub mod qap_creator;
// mod unit_tests;

#[path="./qap_creator.rs"] pub mod qap;
use crate::qap::qap::r1cs_to_qap;
use crate::qap::qap::solution_poly;
use crate::qap::qap::create_divisor_poly;

extern crate colored;
use colored::*;

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

    let (a_qap, b_qap, c_qap, Z) = r1cs_to_qap(&a, &b, &c);
    println!("QAP A : {:?}", a_qap );
    println!("QAP B : {:?}", b_qap );
    println!("QAP C : {:?}", c_qap );
    println!("Z : {:?}", Z );

    let answer = vec![1.0, 3.0, 35.0, 9.0, 27.0, 30.0];

    let (a_sol, b_sol, c_sol, sol ) = solution_poly(answer, a_qap, b_qap, c_qap);
    println!("Solution for  A : {:?}", a_sol );
    println!("Solution for  B : {:?}", b_sol );
    println!("Solution for  C : {:?}", c_sol );
    println!("Sol : {:?}", sol );

    let (z_fac, rem) = create_divisor_poly(&sol ,&Z);
    println!("Z cofactor : {:?} and remainder : {:?}", z_fac, rem);


    let correct_a_sol = vec![43.0, -73.33333333333334, 38.5, -5.166666666666666];
    let correct_b_sol = vec![-3.0, 10.333333333333332, -5.0, 0.6666666666666666];
    let correct_c_sol = vec![-41.0, 71.66666666666663, -24.5, 2.833333333333332];
    let solution = vec![-88.0, 592.6666666666666, -1063.7777777777778, 805.8333333333334, -294.77777777777777, 51.49999999999999, -3.4444444444444438];
    let z_correct = vec![24.0, -50.0, 35.0, -10.0, 1.0];
    let z_fac_correct = vec![-3.666666666666657, 17.055555555555557, -3.4444444444444438];
    let rem_correct = vec![-0.00000000000022737367544323206, 0.0000000000003979039320256561, -0.0000000000003694822225952521, 0.00000000000017053025658242404];
    assert_eq!(a_sol, correct_a_sol);
    assert_eq!(b_sol, correct_b_sol);
    assert_eq!(c_sol, correct_c_sol);
    assert_eq!(sol, solution);
    assert_eq!(Z, z_correct);
    assert_eq!(z_fac, z_fac_correct);
    assert_eq!(rem, rem_correct);


    let m = "Everything went perfectly!";
    println!("{}", m.green());



    let _correct_r1cs : Vec<Vec<f64>> = [
        vec![-5.0, 9.166666666666666, -5.0, 0.8333333333333334],
        vec![8.0, -11.333333333333332, 5.0, -0.6666666666666666],
        vec![0.0, 0.0, 0.0, 0.0],
        vec![-6.0, 9.5, -4.0, 0.5],
        vec![4.0, -7.0, 3.5, -0.5],
        vec![-1.0, 1.8333333333333333, -1.0, 0.16666666666666666]
        ].to_vec();

}

