pub mod lattice;
use num_complex::Complex64;
use lattice::Lattice;
use rayon::prelude::*;


use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;



/// This const is usually used to fix $\omega$ for our shaking functions 
/// to be $\omega=11.5\omega_r$, since we only vary the amplitude of the shaking function in this RL.
const FREQ: f64 = 11.5;

/// Decide if we first shake to the +ve or -ve x direction.
/// Each subsequent shaking has opposite sign.
const TOGGLE_INIT : f64 = 1.0;


/// #Description of code:
/// 
/// AUTHOR: SHAH SAAD ALAM
/// 
/// main method takes in a hard-coded shaking sequence, multi-thread iterates over acceleration values and writes out in _test.txt_ 
/// the output. output is in the form for different $p$ values as determined by N_STATES
/// 
/// [acc index, lattice index, acceleration $a$ , lattice depth $V_0$ , P(p|a,V_0$ ]
fn main() {

   for i in (0..11) {
      for j in (0..11) {
         let acc = -0.0225 + (0.0225*2.0 * j as f64)/(10.0 as f64);
         let latt_depth = 9.0+ (i as f64 *2.0)/10.0;
         let lattice = Lattice::new(acc, latt_depth);
         let tuple = (acc, latt_depth, lattice.get_hamiltonian().symmetric_eigen().eigenvalues[1]);
         println!("{:?}", tuple );
      }
   }

   
}
