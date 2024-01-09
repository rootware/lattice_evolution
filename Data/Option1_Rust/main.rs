 pub mod lattice;// use std::f64::consts::PI;
// use ndarray::Zip;
use num_complex::Complex64;
use lattice::Lattice;
use rayon::prelude::*;
const FREQ: f64 = 11.5;

use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

/*
fn main() {
   let mut latt = Lattice::new(-0.01, 10.0);


   let shakingfunctions = vec![1.83259571, 0., 1.83259571, 2.87979327, 1.83259571, 1.83259571, 1.83259571, 3.40339204, 3.66519143,
   3.40339204, 3.40339204, 3.14159265, 3.92699082, 3.92699082, 2.35619449, 2.35619449, 3.92699082, 3.92699082,
   3.92699082, 3.66519143, 3.66519143, 3.66519143, 2.61799388, 3.66519143, 1.57079633, 1.57079633, 1.57079633,
   1.04719755, 1.04719755, 1.04719755, 1.04719755, 1.57079633];
   let mut sign = 1.0;
   println!("{}", latt.get_psi());
   for ampl in shakingfunctions{
      latt.step( sign*ampl, FREQ);
      sign*= -1.0;
   };

   let out = latt.get_psi();
   let momentum_i: Vec<Complex64> = (out.conjugate().component_mul(&out)).data.into();
   let momentum : Vec<f64> = momentum_i.iter().map(|&m| m.re).collect();
   println!("{:#?}", momentum);


}
*/



fn main() {
   let shakingfunctions2p2e: Vec<f64>= vec![3.92699082, 3.66519143, 2.35619449, 1.30899694, 1.30899694, 2.35619449, 3.40339204, 3.14159265, 3.40339204,
   3.40339204, 3.40339204, 3.40339204, 2.87979327, 2.35619449, 2.35619449, 2.35619449, 2.35619449, 2.35619449,
   2.35619449, 2.61799388, 3.66519143, 3.14159265, 3.66519143, 3.66519143, 3.14159265, 2.87979327, 3.14159265,
   3.40339204, 1.04719755, 1.04719755, 0.78539816, 2.61799388]; 
   
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

         let mut sign = 1.0;
         for ampl in &shakingfunctions2p2e{
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
