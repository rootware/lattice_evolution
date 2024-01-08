use num_complex::Complex64;
use nalgebra::{ DMatrix,DVector};
const N_STATES : usize = 11; // Size of matrices
use std::f64::consts::PI;
const MASS: f64 = 1.0;
const N_STEPS : i128 = 10000;

#[derive(Debug)]
pub struct Lattice {
    g : f64,
    q: f64,
    depth: f64,
    h0: DMatrix<Complex64>,
    h1: DMatrix<Complex64>,
    h2: DMatrix<Complex64>,
    psi: DVector<Complex64>
}

impl Lattice {
    pub fn get_depth(&self) -> f64 {
        self.depth
    }

    pub fn set_depth(&mut self, depth: f64) {
        self.depth = depth;
    }

    pub fn get_hamiltonian(&self) -> DMatrix<Complex64> {
        let hamiltonian = &self.h0 - &self.h2;
        hamiltonian
    }

    pub fn get_psi(&self) -> DVector<Complex64> {
        self.psi.clone()
    }

    pub fn accelerate(&mut self, impulse: f64) {
        self.q+= impulse;
        let max_p: f64 = (N_STATES as f64)-1.0 ; 

        let diagonal = DVector::<Complex64>::from_vec( (0..N_STATES).map(|i| {
            Complex64::new((i as f64 * 2.0 -max_p + self.q).powi(2), 0.0)
        }).collect() );
        // Is there an easier way to convert the f64 diagonal into a Complex valued array?

        let h0new = DMatrix::<Complex64>::from_diagonal( &diagonal);
        self.h0 = h0new;
    }

    pub fn step( &mut self, amplitude: f64, omega: f64){
        let mut time : f64 = 0.0;
        let period : f64 = PI/omega;    
        let no_iter = N_STEPS;
        let mut it = 0;
        let dt = period/(no_iter as f64);
        while it < no_iter {
            self.rk4step( dt,  amplitude, omega, time);
            it+=1; time+=dt;
        }
        
    }

    pub fn update(&self, wavefunction: DVector<Complex64>, amplitude:f64, omega: f64, t: f64 ) -> DVector<Complex64> {
        let phi = amplitude*f64::sin(omega*t);
        let hamiltonian = &self.h0 + &self.h1*Complex64::from(f64::sin(phi)) - &self.h2*Complex64::from(f64::cos(phi));
        hamiltonian*wavefunction*Complex64::new(0.0,-1.0)
    }

    pub fn rk4step(&mut self, dt:f64, amplitude: f64, omega: f64, t:f64){
        let k1 = self.update(self.psi.clone(), amplitude, omega, t);
        let k2 = self.update(self.psi.clone() + &k1*Complex64::from(dt/2.0) , amplitude, omega, t+dt/2.0);
        let k3 = self.update(self.psi.clone() + &k2*Complex64::from(dt/2.0) , amplitude, omega, t+dt/2.0);
        let k4 = self.update(self.psi.clone() + &k3*Complex64::from(dt) , amplitude, omega, t+dt);

        self.psi = &self.psi + ( k1 + k2*Complex64::from(2.0) + k3*Complex64::from(2.0) + k4)*Complex64::from(dt/6.0);
        self.accelerate(-MASS*self.g*dt);
    }
}

impl Lattice {

    pub fn new(acceleration: f64, latticedepth: f64) -> Lattice {
        let max_p: f64 = (N_STATES as f64)-1.0 ; 

        let diagonal = DVector::<Complex64>::from_vec( (0..N_STATES).map(|i| {
            Complex64::new((i as f64 * 2.0 -max_p).powi(2), 0.0)
        }).collect() );
        // Is there an easier way to convert the f64 diagonal into a Complex valued array?

        let h0 = DMatrix::<Complex64>::from_diagonal( &diagonal);


        let mut h2 = DMatrix::from_element(N_STATES,N_STATES, Complex64::new(0.0,0.0)); 

        let mut  h1 = DMatrix::from_element(N_STATES,N_STATES, Complex64::new(0.0,0.0)); 
        let depth =  latticedepth;
        for i in 0..(N_STATES -1){
            h2[(i,i+1)] += Complex64::new(depth/4.0,0.0);
            h2[(i+1,i)] += Complex64::new(depth/4.0,0.0);

            h1[(i,i+1)] += Complex64::new(0.0, depth/4.0);
            h1[(i+1,i)] += Complex64::new(0.0, -depth/4.0);
        }
        let hamiltonian = &h0 - &h2;

        let eigvec = -hamiltonian.symmetric_eigen().eigenvectors;
      //  let mut psitemp :DVector<Complex64> = eigvec.column(3).into();
      //  psitemp = &psitemp/Complex64::from((&psitemp).norm_squared());
        Self {
            g: acceleration,
            q: 0.0,
            depth: latticedepth,
            h0 ,
            h1 ,
            h2 ,
            psi: eigvec.column(1).into()
        }
    }
}

