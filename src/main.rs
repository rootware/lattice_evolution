pub mod lattice;
pub mod read;
pub mod units;
use num_complex::Complex64;
use lattice::Lattice;
use read::read;
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



/// #Description of code:
/// 
/// AUTHOR: SHAH SAAD ALAM
/// 
/// main method takes in a hard-coded shaking sequence, multi-thread iterates over acceleration values and writes out in _test.txt_ 
/// the output. output is in the form for different $p$ values as determined by N_STATES
/// 
/// [acc index, lattice index, acceleration $a$ , lattice depth $V_0$ , P(p|a,V_0$ ]
fn main() {

    let (time_val, shakingfunction) = read();
    assert_eq!(time_val.len(), shakingfunction.len());

    // Create file
    let _file1 = File::create("./Catie/test_longer_withprop.txt").unwrap();
    let _file2 = File::create("./Catie/cfi_qfi_longer_withprop.txt").unwrap();

    // Open file
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./Catie/test_longer_withprop.txt").unwrap();

    // Wrap file in Mutex for thread safety
    let file = Mutex::new(file);

        // Open file
    let file2 = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./Catie/cfi_qfi_longer_withprop.txt").unwrap();

    // Wrap file in Mutex for thread safety
    let file2 = Mutex::new(file2);


    // We multithread iterator over acceleration, but not lattice depth. 
    // Sufficient for my laptop/desktop.
    println!("Generating priors for Catie's sequence");

    let bar = ProgressBar::new(101 );
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {wide_bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap()
    .progress_chars("##-"));

    let _sum : Vec<f64> = (0..101).into_par_iter().map(|x| {
     // let acc = -0.00225 + (0.00225*2.0 * x as f64)/(1000 as f64);
    let acc = -0.1 + (0.1*2.0 * x as f64)/(100 as f64);
     for y in 0..51 {
        let latdep : f64 =  9.0 + (2.0* y as f64)/(50 as f64);
    


        let mut latt = Lattice::new(acc, latdep);

        //----------------------
        let mut index: usize = 0;
        let mut total_time = 0.0;// just for consistency, compiler will complain, ideally should be 0.0
        let propagatime_time = 100.0e-6/units::TIME_UNIT;
        let time_of_start_propagation = 117.0e-6/units::TIME_UNIT;
        
        for ampl in &shakingfunction{

          //  total_time = time_val[index]/units::TIME_UNIT;
            latt.set_time(total_time);

            let amplitude = *ampl;

            let mut time : f64 = 0.0;
            let period : f64 = 50.0e-9/units::TIME_UNIT; // 50ns in code units    
            let no_iter = 10; // small
            let mut it = 0;
            let dt = period/(no_iter as f64);


            while it < no_iter {
                latt.rk4step( dt,  amplitude, FREQ, time);
                it+=1; time +=dt;
    
            }
            total_time += time;
            index+=1;


  


        }
        if x==1 && y==1 {
            println!("{}",total_time*units::TIME_UNIT);
        }
    //-----------------------------

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

        if y == 25 {
            let result = vec![acc, latdep, latt.acc_cfi(), latt.acc_qfi()];
            let mut s = String::new();
            for num in result {
                s.push_str(&num.to_string());
                s.push_str("\t");
            }
            s.push_str("\n");
    
            file2.lock()
            .unwrap()
            .write_all( s.as_bytes())
            .unwrap();

        }

        
      }
      
      bar.inc(1); acc}).collect();


   
}
