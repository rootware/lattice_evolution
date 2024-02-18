pub mod lattice;
use lattice::Lattice;
use std::f64::consts::PI;

use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
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
   let _file2 = File::create("./testing_QFI_basis/unchanged_momentum_basis.txt").unwrap();

   // Open file
   let mut file = OpenOptions::new()
      .write(true)
      .append(true)
      .open("./testing_QFI_basis/unchanged_momentum_basis.txt").unwrap();

   // We multithread iterator over acceleration, but not lattice depth. 
   // Sufficient for my laptop/desktop.
   println!("Testing CFI calculations using the augmented state method");

   let no_of_runs : u64 = latt_shaking.len() as u64; 

   let bar = ProgressBar::new(no_of_runs );
   bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {wide_bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap()
    .progress_chars("##-"));

   let tof = PI/11.5 *32.0;
   let f_mz = ( 4.0 * 2.0 * tof.powi(2) ).powi(2)/4.0;
   println!("{}", f_mz);
   

   // let latdep : f64 =  9.0 + (2.0* y as f64)/(50 as f64);
   let acc: f64 = 0.0; let latdep : f64 =  10.0;
   let mut latt = Lattice::new(acc, latdep);

   let mut sign = TOGGLE_INIT;
   let mut total_time = 0.0;
   for ampl in &latt_shaking{

         let amplitude = *ampl*sign;

         let mut time : f64 = 0.0;
         let period : f64 = PI/FREQ;    
         let no_iter = 10000;
         let mut it = 0;
         let dt = period/(no_iter as f64);
         while it < no_iter {
            latt.rk4step( dt,  amplitude, FREQ, time);
            it+=1; time +=dt;
            total_time += dt;
            let result = vec![total_time, latt.acc_cfi() , latt.acc_qfi(), latt.depth_cfi(), latt.depth_qfi()];
            let mut s = String::new();

            for num in result {
               s.push_str(&num.to_string());
               s.push_str("\t");
            }
            s.push_str("\n");
         
            file.write_all( s.as_bytes())
            .unwrap();


         }
         latt.set_time(total_time);
         sign *= -1.0;
         bar.inc(1); 

   };
   
}
