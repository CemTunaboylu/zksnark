// polynomials are kept in arrays as follows : 3x^2 + 4x + 5 -> [5,4,3] i.e. index i of the array is the ith degree coefficient 
pub use std::cmp::max; //min,

pub mod qap {
    use crate::qap::max;

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
    pub fn div_poly(a: &Vec<f64>, b: &Vec<f64>) -> (Vec<f64>, Vec<f64>) {
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

    // Assuming vec[0] = p(1), vec[1] = p(2) etc. result -> [deg0 coeff, deg1 coeff ... ]

    pub fn lagrange(vec : &Vec<f64>) -> Vec<f64> {
        let mut out : Vec<f64> = vec![];
        let l : usize = vec.len();
        // let mut inter_eval : f64;
        for i in 1..=l{
            out = add_poly(&out, &(mk_singleton(i, vec[i-1] as isize, l)), true);
            // inter_eval = eval_poly(vec, i as isize);
            // assert!( inter_eval - vec[i-1] < (10 as f64).powi(-10),"{:?}", (vec, inter_eval, i) )
        }
        out
    }

    pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
        assert!(!v.is_empty());
        let len = v[0].len();
        let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect::<Vec<T>>()
            })
            .collect()
    }

    pub fn transpose2<T>(mut v: Vec<Vec<T>>) -> Vec<Vec<T>> {
        assert!(!v.is_empty());
        for inner in &mut v {
            inner.reverse();
        }
        (0..v[0].len())
            .map(|_| {
                v.iter_mut()
                    .map(|inner| inner.pop().unwrap())
                    .collect::<Vec<T>>()
            })
            .collect()
    }

    pub fn r1cs_to_qap(a : &Vec<Vec<f64>>, b : &Vec<Vec<f64>>, c : &Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<f64>){
        let a_transpose = transpose(a.to_vec());
        let b_transpose = transpose(b.to_vec());
        let c_transpose = transpose(c.to_vec());
        let a_t = &a_transpose;

        let lagrange_a : Vec<Vec<f64>> = a_transpose.iter().map(|row| lagrange(&row)).collect();
        let lagrange_b : Vec<Vec<f64>> = b_transpose.iter().map(|row| lagrange(&row)).collect();
        let lagrange_c : Vec<Vec<f64>> = c_transpose.iter().map(|row| lagrange(&row)).collect();

        let mut Z : Vec<f64> = vec![1.0];
        let col = a_t[0].len();
        for i in 1..=col {
            Z = mult_poly(&Z, &vec![i as f64 * (-1.0), 1.0]);
        }
        (lagrange_a, lagrange_b, lagrange_c, Z)

    }

    pub fn solution_poly(answer : Vec<f64>, qap_a: Vec<Vec<f64>>, qap_b: Vec<Vec<f64>>, qap_c: Vec<Vec<f64>>) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>){
        let mut a_poly : Vec<f64> = vec![];
        let mut b_poly : Vec<f64> = vec![];
        let mut c_poly : Vec<f64> = vec![];

        let mut a_iter = qap_a.into_iter();
        let mut b_iter = qap_b.into_iter();
        let mut c_iter = qap_c.into_iter();


        // |a| = |b| = |c| -> we can merge them in one loop
        for ans_elm in answer.into_iter(){
            a_poly = add_poly(&a_poly, &mult_poly( &vec![ans_elm], &a_iter.next().unwrap()), true);
            b_poly = add_poly(&b_poly, &mult_poly(&vec![ans_elm], &b_iter.next().unwrap()), true);
            c_poly = add_poly(&c_poly, &mult_poly(&vec![ans_elm], &c_iter.next().unwrap()), true);
        }

        let o = sub_poly(&mult_poly(&a_poly, &b_poly), &c_poly);

        // println!("{:?}", a_poly);
        // println!("{:?}", b_poly);
        // println!("{:?}", c_poly);
        (a_poly, b_poly, c_poly, o)

    }

    pub fn create_divisor_poly(sol : &Vec<f64>, Z : &Vec<f64>) -> (Vec<f64>, Vec<f64>){
        let (quot, rem) = div_poly(&sol, &Z);
        (quot, rem)
    }

}
