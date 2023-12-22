use ndarray::prelude::*;
use num_complex::Complex64;
use ndarray_linalg::eigh::Eigh;
const N_STATES : usize = 11; // Size of matrices
const FREQ : f64 = 20.0; //Sets time for RK in divisions of PI


#[derive(Debug)]
pub struct Lattice {
    depth: f64,
    h0: Array2<Complex64>,
    h1: Array2<Complex64>,
    h2: Array2<Complex64>,
    max_p: f64,
    psi0: Array1<Complex64>,
    psi: Array1<Complex64>,
}

impl Lattice {
    pub fn depth(&self) -> f64 {
        self.depth
    }

    pub fn set_depth(&mut self, depth: f64) {
        self.depth = depth;
    }
}

impl Default for Lattice {
    fn default() -> Self {

        let max_p: f64 = (N_STATES as f64)-1.0 ; 

        let diagonal = Array::linspace(-max_p, max_p, N_STATES);

        // Is there an easier way to convert the f64 diagonal into a Complex valued array?
        
        let h0 = (Array2::from_diag(&diagonal)).mapv(|x| Complex64::from(x));

        let mut h1 = Array::from_elem((N_STATES,N_STATES), Complex64::new(0.0,0.0)); 
    
        let h2 = Array2::<Complex64>::eye(N_STATES);
        for i in 1..(N_STATES -1){
            h1[[i,i+1]] += Complex64::new(-10.0,0.0);
            h1[[i+1,i]] += Complex64::new(-10.0,0.0);
        }
        let hamiltonian = h0 + &h2;

        let (val, vec) = eigh( &hamiltonian).unwrap();

        Self {
            depth: 10.0,
            h0,
            h1,
            h2,
            max_p,
            psi0 : &vec(0),
            psi: &vec,
        }
    }
}