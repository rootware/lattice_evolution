pub mod lattice;
use nalgebra::DVector;
use num_complex::Complex64;
use lattice::Lattice;
use rayon::prelude::*;
use std::f64::consts::PI;

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

   // hard code our shaking sequence
   let latt_shaking : Vec<f64> = vec![1.83259571, 1.83259571, 1.83259571, 1.83259571, 1.83259571, 1.83259571, 1.04719755, 1.04719755, 1.04719755,
   0., 0., 0.52359878, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.26179939, 0., 0., 0., 0., 0., 0., 0., 0.];
   // Create file
   let _file2 = File::create("./testing_cfi_lattice/test_longer_withqfi.txt").unwrap();

   // Open file
   let file = OpenOptions::new()
      .write(true)
      .append(true)
      .open("./testing_cfi_lattice/test_longer_withqfi.txt").unwrap();

   // Wrap file in Mutex for thread safety
   let file = Mutex::new(file);

   // We multithread iterator over acceleration, but not lattice depth. 
   // Sufficient for my laptop/desktop.
   println!("Testing CFI calculations using the augmented state method");

   let no_of_runs : u64 = 100; 

   let bar = ProgressBar::new(no_of_runs );
   bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {wide_bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap()
    .progress_chars("##-"));

   let tof = PI/11.5 *32.0;
   let f_mz = ( 4.0 * 2.0 * tof.powi(2) ).powi(2)/4.0;
   println!("{}", f_mz);
   
   let _sum : Vec<f64> = (0..no_of_runs).into_par_iter().map(|y| {
     // let acc = -0.00225 + (0.00225*2.0 * x as f64)/(1000 as f64);
     let x = 0;
     let acc = 0.0;
        let latdep : f64 =  9.0 + (2.0* y as f64)/(no_of_runs as f64);
 
      


         let mut latt = Lattice::new(acc, latdep);

         let mut sign = TOGGLE_INIT;
         for ampl in &latt_shaking{
             latt.step( *ampl*sign, FREQ);
             sign *= -1.0;
         };


         let result = vec![latt.depth_cfi() , latt.depth_qfi()];
         let mut s = String::new();
         s =  s + &format!("{x}\t{y}\t");
         s =  s + &format!("{acc}\t{latdep}\t");


         for num in result {
            s.push_str(&num.to_string());
            s.push_str("\t");
         }
         s.push_str("\n");

         file.lock()
         .unwrap()
         .write_all( s.as_bytes())
         .unwrap();

         
      
      
      bar.inc(1); acc}).collect();



   
}
