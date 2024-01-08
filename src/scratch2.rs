/*

impl Default for Lattice {
    fn default() -> Self {

        let max_p: f64 = (N_STATES as f64)-1.0 ; 

        let diagonal = DVector::<Complex64>::from_vec( (0..N_STATES).map(|i| {
            Complex64::new((i as f64 * 2.0 -max_p).powi(2), 0.0)
        }).collect() );
        // Is there an easier way to convert the f64 diagonal into a Complex valued array?

        let h0 = DMatrix::<Complex64>::from_diagonal( &diagonal);


        let mut h2 = DMatrix::from_element(N_STATES,N_STATES, Complex64::new(0.0,0.0)); 

        let mut  h1 = DMatrix::from_element(N_STATES,N_STATES, Complex64::new(0.0,0.0)); 
        let depth: f64 = 10.0;
        for i in 0..(N_STATES -1){
            h2[(i,i+1)] += Complex64::new(depth/4.0,0.0);
            h2[(i+1,i)] += Complex64::new(depth/4.0,0.0);

            h1[(i,i+1)] += Complex64::new(0.0, depth/4.0);
            h1[(i+1,i)] += Complex64::new(0.0, -depth/4.0);
        }
        let hamiltonian = &h0 - &h2;

        let eigvec = hamiltonian.symmetric_eigen().eigenvectors;

        Self {
            g: 0.0,
            depth: 10.0,
            h0 ,
            h1 ,
            h2 ,
            psi: eigvec.column(3).into()
        }
    }
}
*/

/*
fn main() {
    let mut latt = Lattice::new(0.0, 10.0);
    let shakingfunctions2p2e_new: Vec<f64>= vec![1.83259571, 0., 1.83259571, 2.87979327, 1.83259571, 1.83259571, 1.83259571, 3.40339204, 3.66519143,
    3.40339204, 3.40339204, 3.14159265, 3.92699082, 3.92699082, 2.35619449, 2.35619449, 3.92699082, 3.92699082,
    3.92699082, 3.66519143, 3.66519143, 3.66519143, 2.61799388, 3.66519143, 1.57079633, 1.57079633, 1.57079633,
    1.04719755, 1.04719755, 1.04719755, 1.04719755, 1.57079633]; 
 
    for ampl in &shakingfunctions2p2e_new{
       latt.step( *ampl, FREQ);
    };
 
    let out = latt.get_psi();
    let momentum_i: Vec<Complex64> = (out.conjugate().component_mul(&out)).data.into();
    let momentum : Vec<f64> = momentum_i.iter().map(|&m| m.re).collect();
 
    println!("{:#?}", momentum);
 }
 */

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