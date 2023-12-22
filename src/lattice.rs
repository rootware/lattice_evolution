use num_complex::Complex64;
use std::{iter::*, convert::identity};
use nalgebra::{SMatrix, SVector, DMatrix,DVector};
const N_STATES : usize = 11; // Size of matrices
const FREQ : f64 = 20.0; //Sets time for RK in divisions of PI

#[derive(Debug)]
pub struct Lattice {
    depth: f64,
    h0: SMatrix<Complex64, N_STATES, N_STATES>,
    h1: SMatrix<Complex64, N_STATES, N_STATES>,
    h2: SMatrix<Complex64, N_STATES, N_STATES>,
    max_p: f64,
    psi0: SVector<Complex64, N_STATES>,
    psi: SVector<Complex64, N_STATES>,
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

        let diagonal : Vec<Complex64>= (0..N_STATES).map(|i| Complex64::new(i as f64 * 2.0 -max_p, 0.0)).collect();
        // Is there an easier way to convert the f64 diagonal into a Complex valued array?
        
       let h0 = SMatrix::<Complex64, N_STATES, N_STATES>::from_vec(diagonal);

        let mut h1 = SMatrix::<Complex64, N_STATES, N_STATES>::from_element(Complex64::new(0.0,0.0)); 
    
        let h2 = SMatrix::<Complex64, N_STATES, N_STATES>::identity();

        for i in 1..(N_STATES -1){
            h1[(i,i+1)] += Complex64::new(-10.0,0.0);
            h1[(i+1,i)] += Complex64::new(-10.0,0.0);
        }
        let hamiltonian = h0 + &h2;

        let vec = hamiltonian.symmetric_eigen().eigenvectors;
        println!("{:#?}",&vec);
        Self {
            depth: 10.0,
            h0,
            h1,
            h2,
            max_p,
            psi0 : SVector::<Complex64, N_STATES>::zeros(),
            psi: SVector::<Complex64, N_STATES>::zeros(),
        }
    }
}
