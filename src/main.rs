pub mod jittery_lattice;
pub mod lattice;
pub mod statistics;
pub mod read_test;
use rand::prelude::*;
use std::iter::*;
/*use plotly::common::{
   ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};*/
use rayon::prelude::*;

use ndarray::{Array1,Array2, Array3};
// use jittery_lattice::JitteryLattice;
// use statistics::jenson_shannon_divergence;


use std::fs::File;
use ndarray_npy::{read_npy, write_npy};
use ndarray::s;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use std::io::{BufReader, BufRead};
// use random_choice::random_choice;

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

   let alist :  Array1<f64> = read_npy("./SP_Bayesianpriors_FinerRun/acceleration.npy").unwrap();
   let vlist : Array1<f64> = read_npy("./SP_Bayesianpriors_FinerRun/latticedepth.npy").unwrap();

   let avlistindex : Array2<i64> = read_npy("./SP_Bayesianpriors_FinerRun/AVIndex.npy").unwrap();

   let momprob : Array2<f64> = read_npy("./SP_Bayesianpriors_FinerRun/MomProb.npy").unwrap();
   let datamom : Array3<f64> = read_npy("./SP_Bayesianpriors_FinerRun/datamom.npy").unwrap();

   let mut prob_av : Array2<f64> = Array2::<f64>::ones( (alist.len(), vlist.len()));
   prob_av  /= prob_av.sum();

   let prob_actual = datamom.slice(s![alist.len()/2+1,vlist.len()/2+1, ..]).into_owned();

   /* 
   let mut samples = vec![0,1,2,3,4,5,6,7,8,9,10];
   let mut mychoices : Vec<(i32,f64)>= Vec::new();
   
   for i in samples {
      mychoices.push ( (i, prob_actual[i as usize]));

   }
   
   let mut rng = thread_rng();
   // 50% chance to print 'a', 25% chance to print 'b', 25% chance to print 'c'
   let outcomes = mychoices.choose_weighted(&mut rng, |item| item.1).unwrap().0;
   println!("{:#?}",outcomes);
   */

   // Read same file as the Bayesian code

   let file =  File::open("./Data/Rust_Runs/SPvMPBayesian/SP_acc_multiparamBayesianpriors/500.txt").unwrap();
   let reader = BufReader::new(file);

   let outcomes: Vec<i32> = reader
   .lines()
   .map(|line| line.unwrap().parse::<f64>().unwrap() as i32)
   .collect();

   let bar = ProgressBar::new(outcomes.len() as u64);
   bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {wide_bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap()
    .progress_chars("##-"));



   let mut counter = 0;

   for outcome in outcomes {
      // let _dummy :Vec<usize>= (0..alist.len() ).into_par_iter().map(|a_index| {

      /* 
      let accbar = ProgressBar::new(alist.len() as u64);
      accbar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {wide_bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
         .unwrap()
         .progress_chars("##-"));

      
      for a_index in 0..alist.len() {
         for v_index in 0..vlist.len() {
            prob_av[[ a_index, v_index]] *= datamom[[ a_index, v_index, outcome as usize ]];
            prob_av /= prob_av.sum();
         }
        accbar.inc(1);
      };

      */

      prob_av = &prob_av* datamom.slice( s![..,..,outcome as usize]).into_owned();
      prob_av/= prob_av.sum();

      bar.inc(1);
      counter +=1;
      if counter == 50 {
         write_npy("./SP_Bayesianpriors_FinerRun/N50.npy", &prob_av).unwrap();
      }

   }
}
