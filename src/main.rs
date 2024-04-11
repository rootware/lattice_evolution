pub mod jittery_lattice;
pub mod lattice;
pub mod statistics;
pub mod read_test;
/*use plotly::common::{
   ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};*/
use rayon::prelude::*;
use num_complex::Complex64;
use ndarray::Array2;
use lattice::Lattice;
use read_test::{load_data};
// use jittery_lattice::JitteryLattice;
// use statistics::jenson_shannon_divergence;


use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

//use plotly::{Plot, Scatter};


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
   let _mp_shaking: Vec<f64> = vec![1.83259571, 0., 1.83259571, 2.87979327, 1.83259571, 1.83259571, 1.83259571, 3.40339204, 3.66519143,
   3.40339204, 3.40339204, 3.14159265, 3.92699082, 3.92699082, 2.35619449, 2.35619449, 3.92699082, 3.92699082,
   3.92699082, 3.66519143, 3.66519143, 3.66519143, 2.61799388, 3.66519143, 1.57079633, 1.57079633, 1.57079633,
   1.04719755, 1.04719755, 1.04719755, 1.04719755, 1.57079633];//Option 2 sequence, or "MP" sequence

   let _option1_shaking: Vec<f64>= vec![3.92699082, 3.66519143, 2.35619449, 1.30899694, 1.30899694, 2.35619449, 3.40339204, 3.14159265, 3.40339204,
   3.40339204, 3.40339204, 3.40339204, 2.87979327, 2.35619449, 2.35619449, 2.35619449, 2.35619449, 2.35619449,
   2.35619449, 2.61799388, 3.66519143, 3.14159265, 3.66519143, 3.66519143, 3.14159265, 2.87979327, 3.14159265,
   3.40339204, 1.04719755, 1.04719755, 0.78539816, 2.61799388] ;



   let sp_shaking: Vec<f64> = vec![3.92699082, 3.92699082, 3.40339204, 0., 3.66519143, 0., 3.14159265, 3.66519143, 3.92699082, 3.92699082,
   3.66519143, 3.14159265, 3.14159265, 3.14159265, 3.14159265, 3.14159265, 2.35619449, 1.83259571, 1.83259571,
   1.83259571, 0.78539816, 3.40339204, 3.40339204, 2.87979327, 3.40339204, 3.40339204, 3.40339204, 3.40339204,
   1.04719755, 1.04719755, 0.78539816, 0.78539816];//1param acc
   
   let _l_shaking: Vec<f64> =vec![1.83259571, 1.83259571, 1.83259571, 1.83259571, 1.83259571, 1.83259571, 1.04719755, 1.04719755, 1.04719755,
   0., 0., 0.52359878, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.26179939, 0., 0., 0., 0., 0., 0., 0., 0.];//lattice sensitive sequence



   let latt_shaking : Vec<f64> =  sp_shaking; //In ideal Rust, you could handle this via enums
   
   /* 
   // Create file
   let _file2 = File::create("./SP_Bayesianpriors_FinerRun/test.txt").unwrap();

   // Open file
   let file = OpenOptions::new()
      .write(true)
      .append(true)
      .open("./SP_Bayesianpriors_FinerRun/test.txt").unwrap();

   // Wrap file in Mutex for thread safety
   let file = Mutex::new(file);

   // We multithread iterator over acceleration, but not lattice depth. 
   // Sufficient for my laptop/desktop.
   println!("Generating multiparam Bayesian priors for SP sequence");

   let bar = ProgressBar::new(1001 );
   bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {wide_bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap()
    .progress_chars("##-"));

   let _sum : Vec<f64> = (0..1001).into_par_iter().map(|x| {
      let acc = -0.0225 + (0.0225*2.0 * x as f64)/(1000 as f64);
    // let acc = -0.0225 + (0.0225*2.0 * x as f64)/(500 as f64);
    //let acc = -0.01 + (0.01*2.0 * x as f64)/(500 as f64);

     for y in 0..101 {
         let latdep : f64 =  9.0 + (2.0* y as f64)/(100 as f64);
      


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
      
      bar.inc(1); acc}).collect();


   */
  let mydata : Array2<f64> = load_data();
  println!("{}", mydata);
}
