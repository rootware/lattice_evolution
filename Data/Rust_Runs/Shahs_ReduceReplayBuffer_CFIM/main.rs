 pub mod lattice;// use std::f64::consts::PI;
// use ndarray::Zip;
use num_complex::Complex64;
use lattice::Lattice;
use rayon::prelude::*;


use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

const FREQ: f64 = 11.5;
const TOGGLE_INIT : f64 = 1.0;



fn main() {
  /*/ let shakingfunctions2p2e: Vec<f64>= vec![1.83259571, 0., 1.83259571, 2.87979327, 1.83259571, 1.83259571, 1.83259571, 3.40339204, 3.66519143,
   3.40339204, 3.40339204, 3.14159265, 3.92699082, 3.92699082, 2.35619449, 2.35619449, 3.92699082, 3.92699082,
   3.92699082, 3.66519143, 3.66519143, 3.66519143, 2.61799388, 3.66519143, 1.57079633, 1.57079633, 1.57079633,
   1.04719755, 1.04719755, 1.04719755, 1.04719755, 1.57079633]; */

   let shahs_shaking : Vec<f64> = vec![ 0.39269908 ,1.43989663, 0.78539816, 3.01069296, 3.27249235 ,1.96349541,
   0.65449847, 1.96349541 ,1.43989663, 4.05789051, 2.61799388, 1.43989663,
   0.    ,     1.43989663 ,1.43989663 ,0.91629786 ,2.74889357, 1.17809725,
   1.43989663 ,3.01069296, 1.43989663 ,0.26179939 ,1.04719755, 0.26179939,
   1.43989663 ,3.14159265 ,1.04719755, 1.43989663, 0.13089969, 1.43989663];
   
   let _file2 = File::create("test.txt").unwrap();
   //file2.write_all(b"Create\n").unwrap();

   let file = OpenOptions::new()
      .write(true)
      .append(true)
      .open("test.txt").unwrap();
   //file.write_all(b"Metadata\n").unwrap();

   let file = Mutex::new(file);

   let _sum : Vec<f64> = (0..101).into_par_iter().map(|x| {
      let acc = -0.0225 + (0.0225*2.0 * x as f64)/(100 as f64);
      
      for y in 0..51 {
         let latdep : f64 =  9.0 + (2.0* y as f64)/(50 as f64);
         let mut latt = Lattice::new(acc, latdep);

         let mut sign = TOGGLE_INIT;
         for ampl in &shahs_shaking{
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
      
      acc}).collect();


   
}
