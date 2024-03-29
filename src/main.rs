pub mod jittery_lattice;
pub mod lattice;
pub mod statistics;

/*use plotly::common::{
   ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};*/
use rayon::prelude::*;
use num_complex::Complex64;

use lattice::Lattice;
use jittery_lattice::JitteryLattice;
use statistics::jenson_shannon_divergence;


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

   let mp_shaking: Vec<f64> = vec![1.83259571, 0., 1.83259571, 2.87979327, 1.83259571, 1.83259571, 1.83259571, 3.40339204, 3.66519143,
   3.40339204, 3.40339204, 3.14159265, 3.92699082, 3.92699082, 2.35619449, 2.35619449, 3.92699082, 3.92699082,
   3.92699082, 3.66519143, 3.66519143, 3.66519143, 2.61799388, 3.66519143, 1.57079633, 1.57079633, 1.57079633,
   1.04719755, 1.04719755, 1.04719755, 1.04719755, 1.57079633];//Option 2 sequence, or "MP" sequence

   let sp_shaking: Vec<f64> = vec![3.92699082, 3.92699082, 3.40339204, 0., 3.66519143, 0., 3.14159265, 3.66519143, 3.92699082, 3.92699082,
   3.66519143, 3.14159265, 3.14159265, 3.14159265, 3.14159265, 3.14159265, 2.35619449, 1.83259571, 1.83259571,
   1.83259571, 0.78539816, 3.40339204, 3.40339204, 2.87979327, 3.40339204, 3.40339204, 3.40339204, 3.40339204,
   1.04719755, 1.04719755, 0.78539816, 0.78539816];//1param acc
   
   let latt_shaking : Vec<f64> =  mp_shaking; //In ideal Rust, you could handle this via enums
   
   // Create file
   let _file2 = File::create("./test_jitter_updated/jitter_MP.txt").unwrap();
   // Open file
   let file = OpenOptions::new()
      .write(true)
      .append(true)
      .open("./test_jitter_updated/jitter_MP.txt").unwrap();

   // Wrap file in Mutex for thread safety
   let file = Mutex::new(file);



   // let jitter_vec = Vec::<f64>::new();
   // let jsd_vec = Vec::<f64>::new();
   // let jitter_vec = Mutex::new(jitter_vec);
   // let jsd_vec = Mutex::new(jsd_vec);

   println!("#Testing jitter#");
   print!("Executing Shaking Sequence for Regular Lattice.. ");

   //-------------------------------------------------------------------------------------
   let latdep : f64 =  10.0;      let acc = 0.0;
   let mut latt = Lattice::new(acc, latdep);

   let mut sign = TOGGLE_INIT;
   for ampl in &latt_shaking{
       latt.step( *ampl*sign, FREQ);
       sign *= -1.0;
   };

   let out = latt.get_psi();
   let momentum_i: Vec<Complex64> = (out.conjugate().component_mul(&out)).data.into();
   let momentum_0 : Vec<f64> = momentum_i.iter().map(|&m| m.re).collect();
   let mut s = String::new();
   let jsd_trivial = jenson_shannon_divergence(momentum_0.clone(), momentum_0.clone() );

   s =  s + &format!("0.0\t{jsd_trivial}\t");
   s =  s + &format!("{acc}\t{latdep}\t");


   for num in &momentum_0 {
      s.push_str(&num.to_string());
      s.push_str("\t");
   }
   s.push_str("\n");

   file.lock()
   .unwrap()
   .write_all( s.as_bytes())
   .unwrap();
   println!("Completed!"   );
   //---------------------------------------------------------------------------------------



   let max_sigma_index = 4;
   let averaging_number = 5; // For each sigma, run independent lattice runs this many times for averaging
   // The averaging can be done in the Python plotting code
   let max_acc = 501;

   let bar = ProgressBar::new(max_acc );
   bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap()
    .progress_chars("##-"));




   // We multithread iterator over acceleration, but not lattice depth. 
   // Sufficient for my laptop/desktop.

   println!("Now calculating jittery lattices..");
   println!("Progress bar shows acceleration values, since that's the largest for loop and the one we multithread");
   
   let _sum : Vec<f64> = (0..max_acc).into_par_iter().map(|x| {
      let acc = -0.0225 + (0.0225*2.0 * x as f64)/((max_acc - 1) as f64);
     // let acc = -0.0225 + (0.0225*2.0 * x as f64)/(500 as f64);
         let _sumd : Vec<f64> = (1..=max_sigma_index).into_iter().map(|sg| {
            let jitter_sigma = sg as f64/max_sigma_index as f64; 

            for _count in 0..averaging_number {
               let mut jittery_latt = JitteryLattice::new( acc, latdep, jitter_sigma);

               let mut sign = TOGGLE_INIT;
               for ampl in &latt_shaking{
                     jittery_latt.step( *ampl*sign, FREQ);
                     sign *= -1.0;
               };

               let out = jittery_latt.get_psi();
               let momentum_i: Vec<Complex64> = (out.conjugate().component_mul(&out)).data.into();
               let momentum : Vec<f64> = momentum_i.iter().map(|&m| m.re).collect();
               let mut s = String::new();

               let jsd = jenson_shannon_divergence(momentum_0.clone(), momentum.clone());

               s =  s + &format!("{jitter_sigma}\t{jsd}\t");
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

               /*/
               jsd_vec.lock()
                  .unwrap()
                  .push(jsd);

               jitter_vec.lock().unwrap().push(jitter_sigma);
               */
            };
         
          sg as f64}).collect();

          bar.inc(1); x as f64}).collect();

      

      /*
      let mut plot = Plot::new();
      let trace = Scatter::new(jitter_vec.into_inner().unwrap(), jsd_vec.into_inner().unwrap()).mode(Mode::Markers);
      plot.add_trace(trace);
      plot.write_html("./test_jitter_updated/jitter_MP.html");
      */

}
