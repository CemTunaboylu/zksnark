// pub mod qap_creator;
pub use crate::qap_creator::{add_poly, mult_poly, div_poly, eval_poly, mk_singleton};



#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    #[ignore]
    fn qap_add_and_sub(){
        let a = vec![1.0; 3];
        let b = vec![1.0; 4];
        let c = vec![0.0,1.0];

        assert_eq!(add_poly(&a,&b,true), [2.0,2.0,2.0,1.0]);
        assert_eq!(add_poly(&a,&b,true), add_poly(&b,&a,true));
        assert_eq!(add_poly(&a,&b, false), [0.0,0.0,0.0,-1.0]);
        assert_eq!(add_poly(&a,&c,true), [1.0,2.0,1.0]);
        assert_eq!(add_poly(&a,&c,true), add_poly(&c,&a,true));
        assert_eq!(add_poly(&a,&c, false), [1.0,0.0,1.0]);
        assert_eq!(add_poly(&c,&a, false), [-1.0,0.0,-1.0]);

    }

    #[test]
    #[ignore]
    fn qap_mult(){
        let a = vec![1.0; 3];
        let b = vec![1.0; 4];
        let c = vec![0.0,1.0];
        let d = vec![1.1; 2];

        assert_eq!( mult_poly(&a,&b), mult_poly(&b,&a) );
        assert_eq!( mult_poly(&a,&b), [1.0,2.0,3.0,3.0,2.0,1.0] );

        assert_eq!( mult_poly(&b,&c), mult_poly(&c,&b));
        assert_eq!( mult_poly(&b,&c), [0.0,1.0,1.0,1.0,1.0]);

        assert_eq!( mult_poly(&a,&c), mult_poly(&c,&a));
        assert_eq!( mult_poly(&a,&c), [0.0,1.0,1.0,1.0]);

        assert_eq!( mult_poly(&a,&d), mult_poly(&a,&d));
        assert_eq!( mult_poly(&a,&d), [1.0,2.0,2.0,1.0]);

        assert_eq!( mult_poly(&b,&d), mult_poly(&d,&b));
        assert_eq!( mult_poly(&b,&d), [1.0,2.0,2.0,2.0,1.0]);
    }

    #[test]
    #[ignore]
    fn qap_div(){
        let a = vec![1.0,2.0,1.0];
        let b = vec![1.0; 2];
        let c = vec![1.0; 3];
        let d = vec![1.0, 1.0, 0.0, 1.0, 0.0, 1.0];
        let e = vec![1.0, 0.0, 1.0];

        let mut q_r : (Vec<f64>, Vec<f64>) = div_poly(&a, &b);
        assert_eq!(q_r.0, [1.0; 2]);
        assert_eq!(q_r.1, [0.0]);

        q_r = div_poly(&c, &b);
        assert_eq!(q_r.0, [0.0, 1.0]);
        assert_eq!(q_r.1, [1.0]);
        
        q_r = div_poly(&d, &e);
        assert_eq!(q_r.0, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(q_r.1, [1.0; 2]);
    }
    #[test]
    #[ignore]
    fn qap_eval(){
        let a = vec![1.0,2.0,1.0];
        let b = vec![1.0; 2];
        let c = vec![1.0; 3];
        let d = vec![1.0, 1.0, 0.0, 1.0, 0.0, 1.0];


        assert_eq!( eval_poly(&a, 1), 4.0 );
        assert_eq!( eval_poly(&a, 0), 1.0 );

        assert_eq!( eval_poly(&b, 2), 3.0 );

        assert_eq!( eval_poly(&c, 2), 7.0 );

        assert_eq!( eval_poly(&d, 2), 43.0 );

        assert_eq!( eval_poly(&d, -2), -41.0 );

    }

    #[test]
    #[ignore]
    fn qap_singleton(){

        assert_eq!( mk_singleton(1,2,3), [6.0, -5.0, 1.0] );
    }
    #[test]
    fn qap_lagrange(){
        let A : Vec<Vec<f64>> = [vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
                                 vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
                                 vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0],
                                 vec![5.0, 0.0, 0.0, 0.0, 0.0, 1.0]].to_vec();
        let B : Vec<Vec<f64>> = [vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
                                 vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
                                 vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                                 vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0]].to_vec();
        let C : Vec<Vec<f64>> = [vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
                                 vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
                                 vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
                                 vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0]].to_vec();

        
    }
}
