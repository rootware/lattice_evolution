pub mod lattice_realistic;
pub mod read;
pub mod units;
pub mod shaking_sequences;
use num_complex::Complex64;
use lattice_realistic::Realistic_Lattice;
use read::read;
use rayon::prelude::*;
use shaking_sequences::shaking::MP_SHAKING;
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

// Acceleration is currently disabled

    // Create file
    let file = File::create("./plot_finer/test.txt").unwrap();

    // Open file
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./plot_finer/test.txt").unwrap();



    // We multithread iterator over acceleration, but not lattice depth. 
    // Sufficient for my laptop/desktop.
    println!("Begin Shaking");




    let acc = 0.0;
    let latdep = 10.0;
    
    let shakingfunction : Vec<f64> = MP_SHAKING.to_vec();
    let bar = ProgressBar::new(shakingfunction);
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {wide_bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap()
    .progress_chars("##-"));


    let mut latt = Lattice::new(acc, latdep);

        //----------------------
    let mut index: usize = 0;
    let mut total_time = 0.0;// just for consistency, compiler will complain, ideally should be 0.0

        
    for ampl in &shakingfunction{

    //  total_time = time_val[index]/units::TIME_UNIT;
        latt.set_time(total_time);



        let mut time : f64 = 0.0;
        let period : f64 = 50.0e-9/units::TIME_UNIT; // 50ns in code units    
        let no_iter = 10; // small
        let mut it = 0;
        let dt = period/(no_iter as f64);


        while it < no_iter {
            let amplitude = *ampl*f64::sin(FREQ*time);
            latt.rk4step( dt,  amplitude, FREQ, time);
            it+=1; time +=dt;

        }
        total_time += time;
        index+=1;




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

        file
        .write_all( s.as_bytes())
        .unwrap();


    }
   
}
