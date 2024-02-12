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
   let latt_shaking : Vec<f64> = vec![1.83259571, 0., 1.83259571, 2.87979327, 1.83259571, 1.83259571, 1.83259571, 3.40339204, 3.66519143,
   3.40339204, 3.40339204, 3.14159265, 3.92699082, 3.92699082, 2.35619449, 2.35619449, 3.92699082, 3.92699082,
   3.92699082, 3.66519143, 3.66519143, 3.66519143, 2.61799388, 3.66519143, 1.57079633, 1.57079633, 1.57079633,
   1.04719755, 1.04719755, 1.04719755, 1.04719755, 1.57079633];//1param acc
   // Create file
   let _file2 = File::create("./testing_cfi/test.txt").unwrap();

   // Open file
   let file = OpenOptions::new()
      .write(true)
      .append(true)
      .open("./testing_cfi/test.txt").unwrap();

   // Wrap file in Mutex for thread safety
   let file = Mutex::new(file);

   // We multithread iterator over acceleration, but not lattice depth. 
   // Sufficient for my laptop/desktop.
   println!("Testing CFI calculations using the augmented state method");

   let no_of_runs : u64 = 501; 

   let bar = ProgressBar::new(no_of_runs );
   bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {wide_bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap()
    .progress_chars("##-"));

   let tof = PI/11.5 *32.0;
   let f_mz = ( 4.0 * 2.0 * tof.powi(2) ).powi(2)/4.0;
   println!("{}", f_mz);
   
   let _sum : Vec<f64> = (0..no_of_runs).into_par_iter().map(|x| {
     // let acc = -0.00225 + (0.00225*2.0 * x as f64)/(1000 as f64);
     let acc = -0.0225 + ( 0.0225*2.0 * x as f64)/( (no_of_runs-1) as f64);
     for y in 0..1 {
        // let latdep : f64 =  9.0 + (2.0* y as f64)/(50 as f64);
        let latdep : f64 =  10.0;
      


         let mut latt = Lattice::new(acc, latdep);

         let mut sign = TOGGLE_INIT;
         for ampl in &latt_shaking{
             latt.step( *ampl*sign, FREQ);
             sign *= -1.0;
         };


         let result = vec![latt.acc_cfi() ];
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

         
      }
      
      bar.inc(1); acc}).collect();


      // test my cfi module
      let d_psi = DVector::from_vec( vec! [ Complex64::new( 0.5, 0.0), Complex64::new(0.2,0.3)]) ;
      let psi = DVector::from_vec( vec! [ Complex64::new( 0.9, 0.1), Complex64::new(0.2,0.3)]) ;

      let p_a : f64 = d_psi.iter().zip(psi.iter()).map(|(&dp, &p)|
      { (2.0*(dp*p.conj()).re ).powi(2)/( p.norm_sqr())}
      ).collect::<Vec<f64>>().iter().sum();

      println!( "{p_a}");

      let byhand = (2.0*( 0.5*0.9))*(2.0*( 0.5*0.9))/(0.81+0.01) + (2.0*( 0.2*0.2 + 0.3*0.3))*(2.0*( 0.2*0.2 + 0.3*0.3)) /(0.04+0.09);
      println!("{byhand}");

   
}
