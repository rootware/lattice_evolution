pub mod lattice_realistic;
pub mod read;
pub mod units;
pub mod shaking_sequences;
use num_complex::Complex64;
use lattice_realistic::Realistic_Lattice;
use read::read;
use rayon::prelude::*;
use shaking_sequences::shaking::MP_SHAKING;
use std::f64::consts::PI;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

// use rustfft::{FftPlanner, num_complex::Complex};


/// This const is usually used to fix $\omega$ for our shaking functions 
/// to be $\omega=11.5\omega_r$, since we only vary the amplitude of the shaking function in this RL.
const FREQ: f64 = 11.5;

const TOGGLE_INIT: f64 = 1.0;

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

    println!("Creating New test.txt for Data");
    // Create file
    let file = File::create("./plot_finer/test_with_tof.txt").unwrap();

    // Open file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./plot_finer/test_with_tof.txt").unwrap();



    // We multithread iterator over acceleration, but not lattice depth. 
    // Sufficient for my laptop/desktop.


    println!("Initializing Lattice");
    let acc = 0.0;
    let latdep = 10.0;
    
    let shakingfunction : Vec<f64> = MP_SHAKING.to_vec();
    let bar = ProgressBar::new(shakingfunction.len() as u64);
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {wide_bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap()
    .progress_chars("##-"));


    let mut latt = Realistic_Lattice::new(acc, latdep);

        //----------------------
    let mut index: usize = 0;
    let mut total_time = 0.0;// just for consistency, compiler will complain, ideally should be 0.0

    println!("Record t=0 info");

    let p_prob = latt.get_psi();
    let mut s = String::new();
    s =  s + &format!("{acc},{latdep},{total_time}, 0.0");


    for num in  p_prob.iter() {
        s.push_str(",");
        //s.push_str(&num.to_string());
        s= s + &format!("{}+{}j",num.re, num.im );
    }
    s.push_str("\n");

    file
    .write_all( s.as_bytes())
    .unwrap();
   // println!("Initial Wavepacket: {}", latt.get_psi());

    let no_iter = 100; // small
    let period : f64 = PI/FREQ; // 50ns in code units   
    let dt = period/(no_iter as f64);
 
    let mut sign : f64 = TOGGLE_INIT;

    println!("Begin Shaking");
    for ampl in &shakingfunction{
    //  total_time = time_val[index]/units::TIME_UNIT;
        latt.set_time(total_time);



        let mut time : f64 = 0.0;
        let mut it = 0;
        let A = *ampl;


        while it < no_iter {
            let amplitude = sign*A*f64::sin(FREQ*time);
            latt.rk4step( dt,  amplitude, FREQ, time);
            it+=1; time +=dt;



           // let momentum : Vec<f64>= latt.get_momentum().data.into();
           let p_prob: nalgebra::Matrix<nalgebra::Complex<f64>, nalgebra::Dyn, nalgebra::Const<1>, nalgebra::VecStorage<nalgebra::Complex<f64>, nalgebra::Dyn, nalgebra::Const<1>>> = latt.get_psi();
        //   let mut planner = FftPlanner::<f32>::new();
         //  let fft = planner.plan_fft_forward(p_prob.len());
           
         //  let data: Vec<Complex64> = p_prob.as_array();
           //let mut buffer = data;
           
          // fft.process(&mut buffer);
            
            let mut s = String::new();
            s =  s + &format!("{acc},{latdep},{}, {}", total_time+time, amplitude);




            for num in p_prob.iter() {
                s.push_str(",");
                //s.push_str(&num.to_string());
                s= s + &format!("{}+{}j",num.re, num.im );
            }
            s.push_str("\n");

            file
            .write_all( s.as_bytes())
            .unwrap();


        }
        total_time += PI/FREQ;
        index+=1;
        sign *= -1.0;
        bar.inc(1);


    //-----------------------------
    }

    println!("End Shaking");
    
    println!("Begin Time of Flight Now");
    latt.toggle_begin_tof();
   
    let tof_time : f64 = 20.0*period;
    let A = 0.0;

    let mut time = 0.0;

    while time < tof_time {
        latt.rk4step( dt,  A, FREQ, time);
        time += dt;

        let p_prob = latt.get_psi();

        let mut s = String::new();
        s =  s + &format!("{acc},{latdep},{}, {}", total_time+time, A);

        for num in p_prob.iter() {
            s.push_str(",");
            //s.push_str(&num.to_string());
            s= s + &format!("{}+{}j",num.re, num.im );
        }
        s.push_str("\n");

        file
        .write_all( s.as_bytes())
        .unwrap();
    }

    

}
