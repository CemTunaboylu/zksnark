// polynomials are kept in arrays as follows : 3x^2 + 4x + 5 -> [5,4,3] i.e. index i of the array is the ith degree coefficient 
use std::cmp::{ max}; //min,


pub fn add_poly(a: &Vec<f64>, b: &Vec<f64>, add: bool) ->  Vec<f64> {
    let mut v = vec![0.0; max(a.len(), b.len())];
    for (i, x) in a.iter().enumerate(){
        v[i] += x;
    }
    for (i, x) in b.iter().enumerate(){
        v[i] += x * (if add {1 as f64} else {-1 as f64} );
    }
    v
}

pub fn mult_poly(a: &Vec<f64>, b: &Vec<f64>) ->  Vec<f64> {
    let mut v = vec![0.0; a.len()+b.len()-1 ];
    for (i, a_x) in a.iter().enumerate(){
        for (j, b_x) in b.iter().enumerate(){
            v[i+j] += a_x*b_x;
        }
    }
    v
}

pub fn sub_poly(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    add_poly(&a, &b, false)
}

// a/b -> returns quotient and remainder
pub fn div_poly(a: &Vec<f64>, b: &Vec<f64>) -> (Vec<f64>, Vec<f64>){
    let mut v = vec![0.0; a.len()-b.len() + 1];
    let mut remainder = a.clone();
    let mut leading_fac : f64;
    let mut pos : usize;
    let mut intermediate : Vec<f64>;

    while remainder.len() >= b.len(){
        leading_fac = (remainder.last().unwrap()/ b.last().unwrap())as f64;
        pos = remainder.len() - b.len();
        v[pos] = leading_fac;
        intermediate = vec![0.0;pos+1];
        intermediate[pos] = leading_fac;
        remainder = sub_poly(&remainder, &mult_poly(&b, &intermediate ));
        remainder = (&remainder[..remainder.len()-1]).to_vec();
    }
    (v, remainder)
}

// evaluate a polynomial at a specific point
pub fn eval_poly(poly: &Vec<f64>, p:isize) -> f64 {
    let mut sum : f64 = 0.0;
    for (i,x) in poly.iter().enumerate(){
        sum += x * p.pow(i as u32) as f64;
    }
    sum
}

pub fn mk_singleton(x : usize, y : isize, total: usize ) -> Vec<f64>{
    let mut fac : isize = 1;
    for i in 1..=total{
        if i!=x{
            fac *= x as isize - i as isize;
        }
    }
    let mut v : Vec<f64> = vec![y as f64 / fac as f64 ];
    for i in 1..=total{
        if i != x{
            v = mult_poly(&v, &(vec![-1.0*i as f64 , 1.0]) );
        }
    }
    v
}