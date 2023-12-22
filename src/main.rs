pub mod lattice;// use std::f64::consts::PI;
// use ndarray::Zip;

use lattice::Lattice;

fn main() {
   let mut latt = Lattice::default();
  
   latt.set_depth(10.0);
   println!("{}", latt.depth());
   dbg!(&latt);
}