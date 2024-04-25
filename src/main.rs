pub mod lattice_realistic;
pub mod read;
pub mod units;
pub mod shaking_sequences;
use lattice_realistic::Realistic_Lattice;
use std::f64::consts::PI;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use read::read;
// use rustfft::{FftPlanner, num_complex::Complex};

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

    println!("Creating New test.txt for Data");
    // Create file
    let _file = File::create("./Recom0/test_with_tof2.txt").unwrap();

    // Open file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./Recom0/test_with_tof2.txt").unwrap();

    println!("Initializing Lattice");
    let acc = 0.0;
    let latdep = 10.0;


   let (time_val, shakingfunction) = read("./Recom0/combined_shaking.txt");
   assert_eq!(time_val.len(), shakingfunction.len());

    let mut latt = Realistic_Lattice::new(acc, latdep);

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

    let dt = 50.0e-9/units::TIME_UNIT;

    println!("Begin Shaking, total time: {total_time}");
    for ampl in &shakingfunction{
        latt.set_time(total_time);

        let amplitude = *ampl;
        latt.rk4step( dt,  amplitude, FREQ, total_time);
        total_time +=dt;


        let p_prob: nalgebra::Matrix<nalgebra::Complex<f64>, nalgebra::Dyn, nalgebra::Const<1>, nalgebra::VecStorage<nalgebra::Complex<f64>, nalgebra::Dyn, nalgebra::Const<1>>> = latt.get_psi();
        
        let mut s = String::new();
        s =  s + &format!("{acc},{latdep},{}, {}", total_time, amplitude);


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

    //-----------------------------

    println!("End Shaking: total time: {total_time}");
    
    println!("Begin Time of Flight Now");

    latt.toggle_begin_tof();

    let period = PI/FREQ;
    let no_of_half_periods : u64 = 80;
    let tof_time : f64 = no_of_half_periods as f64 * period;

    let mut time = 0.0;

    let mut current_period : u64 = 0;

    println!("Half Periods of TOF completed:");
    while time < tof_time {

        latt.rk4step( dt,  0.0, FREQ, time);
        time += dt;

        let p_prob = latt.get_psi();

        let mut s = String::new();
        s =  s + &format!("{acc},{latdep},{}, {}", total_time+time, 0.0);

        for num in p_prob.iter() {
            s.push_str(",");
            //s.push_str(&num.to_string());
            s= s + &format!("{}+{}j",num.re, num.im );
        }
        s.push_str("\n");

        file
        .write_all( s.as_bytes())
        .unwrap();

       //let oof =  (time/period) as u64;
        /*
        if oof > current_period {
            current_period = oof;
            println!("{} and {}", total_time + time, current_period);
        
        }
        */
        
    }

}