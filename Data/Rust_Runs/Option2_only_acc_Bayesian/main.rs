pub mod lattice;
use num_complex::Complex64;
use lattice::Lattice;
use rayon::prelude::*;


use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

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

   // hard code our shaking sequence
   let latt_shaking : Vec<f64> = vec![1.83259571, 0., 1.83259571, 2.87979327, 1.83259571, 1.83259571, 1.83259571, 3.40339204, 3.66519143,
   3.40339204, 3.40339204, 3.14159265, 3.92699082, 3.92699082, 2.35619449, 2.35619449, 3.92699082, 3.92699082,
   3.92699082, 3.66519143, 3.66519143, 3.66519143, 2.61799388, 3.66519143, 1.57079633, 1.57079633, 1.57079633,
   1.04719755, 1.04719755, 1.04719755, 1.04719755, 1.57079633];//1param acc
   
   // Create file
   let _file2 = File::create("test.txt").unwrap();

   // Open file
   let file = OpenOptions::new()
      .write(true)
      .append(true)
      .open("test.txt").unwrap();

   // Wrap file in Mutex for thread safety
   let file = Mutex::new(file);

   // We multithread iterator over acceleration, but not lattice depth. 
   // Sufficient for my laptop/desktop.
   let _sum : Vec<f64> = (0..1001).into_par_iter().map(|x| {
     // let acc = -0.00225 + (0.00225*2.0 * x as f64)/(1000 as f64);
     let acc = -0.00225 + (0.00225*2.0 * x as f64)/(1000 as f64);
     // for y in 0..51 {
         //let latdep : f64 =  9.0 + (2.0* y as f64)/(50 as f64);
      
      // do single param
      for y in 0..1{
         let latdep : f64 =  10.0;// doing single param right now

      // end single param edits

         let mut latt = Lattice::new(acc, latdep);

         let mut sign = TOGGLE_INIT;
         for ampl in &latt_shaking{
             latt.step( *ampl*sign, FREQ);
             sign *= -1.0;
         };

         let out = latt.get_psi();
         let momentum_i: Vec<Complex64> = (out.conjugate().component_mul(&out)).data.into();
         let momentum : Vec<f64> = momentum_i.iter().map(|&m| m.re).collect();
         let mut s = String::new();
         s =  s + &format!("{x}\t{y}\t");
         s =  s + &format!("{acc}\t{latdep}\t");


         for num in momentum {
            s.push_str(&num.to_string());
            s.push_str("\t");
         }
         s.push_str("\n");

         file.lock()
         .unwrap()
         .write_all( s.as_bytes())
         .unwrap();

         
      }
      
      acc}).collect();


   
}
