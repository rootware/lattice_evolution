use num_complex::{Complex, Complex64};
use nalgebra::{   DMatrix, DVector};
use itertools::izip;

use std::f64::consts::PI;
const N_STATES : usize = 9; // Size of matrices
const MASS: f64 = 1.0;
const N_STEPS : i128 = 100000;
const N_FINE_MOMENTUM: usize = 96*8;

const SIGMA_P : f64 = 0.1;
/// Lattice struct represents an instance of the Shaken Optical Lattice
#[derive(Debug)]
pub struct Realistic_Lattice {
    /// acceleration/gravity
    g : f64, 
    /// Correction to pure momentum, $(p+q)$ is kinematic momentum
    q: f64, 
    /// lattice depth
    depth: f64, 
    /// time
    time: f64,
    ///The Kinetic part of the Hamiltonian
    h0: DMatrix<Complex64>, 
    ///The $sin(\phi)$ part of the lattice coupling
    h1: DMatrix<Complex64>, 
    /// the $\cos(\phi)$ part of the lattice coupling
    h2: DMatrix<Complex64>, 
    /// current wavefunction of wavepacket moving through lattice
    psi: DVector<Complex64> ,
    dpsi_a: DVector<Complex64>,
    dpsi_v: DVector<Complex64>,
    /// spacing for grid
    delta_p : f64
}

impl Realistic_Lattice {


    /// Get Delta_p
    pub fn get_delta_p(&self) -> f64 {
        self.delta_p
    }

    /// Returns $a$ of this Lattice
    pub fn get_acceleration(&self) -> f64 {
        self.g
    }
    /// Returns $V_0$ of this Lattice
    pub fn get_depth(&self) -> f64 {
        self.depth
    }
    /// Sets $V_0$ of this Lattice
    pub fn set_depth(&mut self, depth: f64) {
        self.depth = depth;
    }

    /// Sets time of this lattice.
    /// Used only when we're manually doing RK4 to track CFI and QFI metrics.
    pub fn set_time(&mut self, time: f64){
        self.time = time;
    }

    /// Returns full Hamiltonian with $\phi=0$ 
    pub fn get_hamiltonian(&self) -> DMatrix<Complex64> {
        let hamiltonian = &self.h0 - &self.h2;
        hamiltonian
    }

    /// Current Wavefunction of the wavepacket
    pub fn get_psi(&self) -> DVector<Complex64> {
        self.psi.clone()
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }
    pub fn get_dpsi_a(&self) -> DVector<Complex64>{
        self.dpsi_a.clone()
    }

    pub fn get_momentum(&self) -> DVector<f64> {
        let out = self.get_psi();
        let momentum_i = out.conjugate().component_mul(&out);
        let momentum  = DVector::from_vec(momentum_i.into_iter().map(|&m| m.re).collect());
        momentum
    }

    pub fn get_dmomentum_da(&self) -> DVector<f64> {
        let out = self.get_dpsi_a();
        let dmomentum_i = out.conjugate().component_mul(&out);
        let dmomentum  = DVector::from_vec(dmomentum_i.into_iter().map(|&m| m.re).collect());
        dmomentum
    }



    /// Accelerate the wavepacket by adding an _impulse_ to $q$ and updating kinematic momentum
    pub fn accelerate(&mut self, impulse: f64) {
        self.q += impulse;
        let max_p: f64 = (N_STATES as f64)-1.0 ; 

        let diagonal = DVector::<Complex64>::from_vec( (0..N_STATES).map(|i| {
            Complex64::new((i as f64 * 2.0 -max_p + self.q).powi(2), 0.0)
        }).collect() );
        // Is there an easier way to convert the f64 diagonal into a Complex valued array?

        let h0new = DMatrix::<Complex64>::from_diagonal( &diagonal);
        self.h0 = h0new;
    }

    /// Evolve wavepacket in this lattice given $(A,\omega)$ for $\phi=A\sin(\omega t)$
    /// for a time period $period = \pi/(11.5 \omega)$.
    /// Usually, we set $\omega=11.5\omega_r$ to be constant, given by FREQ in Lattice module
    /// 
    /// the step() method calls the rk4step() solver, which uses Runge-Kutta 4 to update the system. 
    /// To do so it calls update(), the method for calculating derivatives
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
        self.time+= time;
        
    }

    /// update is essentially the derivative to the wavepacket at time t$
    /// It's given by $-i*H*|\psi>$
    pub fn update(&self, wavefunction: DVector<Complex64>, amplitude:f64, omega: f64, t: f64 ) -> DVector<Complex64> {
       // let phi = amplitude*f64::sin(omega*t);
        let phi = amplitude;
        let hamiltonian = &self.h0 + &self.h1*Complex64::from(f64::sin(phi)) - &self.h2*Complex64::from(f64::cos(phi));
        hamiltonian*wavefunction*Complex64::new(0.0,-1.0)
    }

    /// update_d is essentially the derivative to the state derivative at time t$
    /// It's given by $-i*H*|\psi> +i*\hat{p}t$
    pub fn update_da(&self, state_deriv: DVector<Complex64>, state: DVector<Complex64>,amplitude:f64, omega: f64, t: f64 ) -> DVector<Complex64> {
        //let phi = amplitude*f64::sin(omega*t);
        let phi = amplitude;


        let hamiltonian = &self.h0 + &self.h1*Complex64::from(f64::sin(phi)) - &self.h2*Complex64::from(f64::cos(phi));


        let max_p: f64 = (N_STATES as f64)-1.0 ; 

        let diagonal = DVector::<Complex64>::from_vec( (0..N_FINE_MOMENTUM+1).map(|i| {
            Complex64::new(-max_p+ i as f64*self.delta_p, 0.0)//remove self.q
        }).collect() );
        // Is there an easier way to convert the f64 diagonal into a Complex valued array?


        hamiltonian*state_deriv*Complex64::new(0.0,-1.0) + diagonal.component_mul(&state)*Complex64::new( 0.0, 2.0*(self.time +t) )
    }

    pub fn update_d_v(&self, state_deriv: DVector<Complex64>, state: DVector<Complex64>,amplitude:f64, omega: f64, t: f64 ) -> DVector<Complex64> {
        // let phi = amplitude*f64::sin(omega*t);
        let phi = amplitude;


        let ham_shaking = &self.h1*Complex64::from(f64::sin(phi)) - &self.h2*Complex64::from(f64::cos(phi));
        let hamiltonian = &self.h0 + &ham_shaking;



        &hamiltonian*state_deriv*Complex64::new(0.0,-1.0) + &ham_shaking*state*Complex::new(0.0, -1.0/self.depth)
    }

    
    /// The RK4 step.
    pub fn rk4step(&mut self, dt:f64, amplitude: f64, omega: f64, t:f64){
        let k1 = self.update(self.psi.clone(), amplitude, omega, t);
        let k2 = self.update(self.psi.clone() + &k1*Complex64::from(dt/2.0) , amplitude, omega, t+dt/2.0);
        let k3 = self.update(self.psi.clone() + &k2*Complex64::from(dt/2.0) , amplitude, omega, t+dt/2.0);
        let k4 = self.update(self.psi.clone() + &k3*Complex64::from(dt) , amplitude, omega, t+dt);
/* 
        let k1_da = self.update_da(self.dpsi_a.clone(), self.psi.clone(), amplitude, omega, t);
        let k2_da= self.update_da(self.dpsi_a.clone() + &k1_da*Complex64::from(dt/2.0) ,self.psi.clone()+ &k1*Complex64::from(dt/2.0), amplitude, omega, t+dt/2.0);
        let k3_da = self.update_da(self.dpsi_a.clone() + &k2_da*Complex64::from(dt/2.0) , self.psi.clone()+&k2*Complex64::from(dt/2.0) ,amplitude, omega, t+dt/2.0);
        let k4_da = self.update_da(self.dpsi_a.clone() + &k3_da*Complex64::from(dt) , self.psi.clone()+&k3*Complex64::from(dt),amplitude, omega, t+dt);

        let k1_d_v = self.update_d_v(self.dpsi_v.clone(), self.psi.clone(), amplitude, omega, t);
        let k2_d_v= self.update_d_v(self.dpsi_v.clone() + &k1_d_v*Complex64::from(dt/2.0) ,self.psi.clone()+ &k1*Complex64::from(dt/2.0), amplitude, omega, t+dt/2.0);
        let k3_d_v = self.update_d_v(self.dpsi_v.clone() + &k2_d_v*Complex64::from(dt/2.0) , self.psi.clone()+&k2*Complex64::from(dt/2.0) ,amplitude, omega, t+dt/2.0);
        let k4_d_v = self.update_d_v(self.dpsi_v.clone() + &k3_d_v*Complex64::from(dt) , self.psi.clone()+&k3*Complex64::from(dt),amplitude, omega, t+dt);
*/
        self.psi = &self.psi + ( k1 + k2*Complex64::from(2.0) + k3*Complex64::from(2.0) + k4)*Complex64::from(dt/6.0);
//        self.dpsi_a = &self.dpsi_a + ( k1_da + k2_da*Complex64::from(2.0) + k3_da*Complex64::from(2.0) + k4_da)*Complex64::from(dt/6.0);
 //       self.dpsi_v = &self.dpsi_v + ( k1_d_v + k2_d_v*Complex64::from(2.0) + k3_d_v*Complex64::from(2.0) + k4_d_v)*Complex64::from(dt/6.0);

        //self.accelerate(-MASS*self.g*dt);
    }
}

impl Realistic_Lattice {

    /// Initializes the Lattice and makes a new instnce
    /// Uses given values of $(a,V_0)$ (see paper) to construct the Hamiltonian.
    /// It further initializes the wavepacket to be the ground state.
    /// 
    /// By default, the const N_STATES defines how many momentum states are included in the
    /// simulation basis. So N_STATES=11 means $p\in \{-10\hbark_L, -8\hbar k_L, ..., 10\hbar k_L\}$
    pub fn new(acceleration: f64, latticedepth: f64) -> Realistic_Lattice {
        let max_p: f64 = (N_STATES as f64)-1.0 ; 
        let delta_p_index = (N_FINE_MOMENTUM as i128/(N_STATES as i128 - 1)) as usize;
        let delta_p = (N_STATES-1) as f64 *2.0/ (N_FINE_MOMENTUM  as f64);
        println!("{delta_p}, {delta_p_index}");
        let diagonal = DVector::<Complex64>::from_vec( (0..N_FINE_MOMENTUM+1).map(|i| {
            Complex64::new((-max_p+ i as f64*delta_p).powi(2), 0.0)
        }).collect() );
        // Is there an easier way to convert the f64 diagonal into a Complex valued array?

        let h0 = DMatrix::<Complex64>::from_diagonal( &diagonal);

        let offset_vec  = DVector::<Complex64>::from_vec( (0..N_FINE_MOMENTUM+1).map(|i| {
            Complex64::new(0.0, 0.0)
        }).collect() );
        let offset =DMatrix::<Complex64>::from_diagonal( &offset_vec);


        let mut h2 = DMatrix::from_element(N_FINE_MOMENTUM+1 ,N_FINE_MOMENTUM +1 , Complex64::new(0.0,0.0)); 

        let mut  h1 = DMatrix::from_element(N_FINE_MOMENTUM+1,N_FINE_MOMENTUM+1, Complex64::new(0.0,0.0)); 
        let depth =  latticedepth;
        for i in 0..(N_FINE_MOMENTUM+1){

            if i+delta_p_index < N_FINE_MOMENTUM+1 {
                h2[(i,i+delta_p_index)] += Complex64::new(depth/4.0,0.0);
                h2[(i+delta_p_index,i)] += Complex64::new(depth/4.0,0.0);

                h1[(i,i+delta_p_index)] += Complex64::new(0.0, depth/4.0);
                h1[(i+delta_p_index,i)] += Complex64::new(0.0, -depth/4.0);
            }
        }


        let mut psitemp : DVector<Complex64>= DVector::from_element( N_FINE_MOMENTUM+1,Complex64::new(0.0,0.0) );
        

     //   println!("{}", (&h0-&h2+&offset).symmetric_eigen().eigenvalues);
     //   println!("{}", (&h0-&h2+&offset).symmetric_eigen().eigenvectors.column(1));
     //   let eigv=(&h0-&h2).symmetric_eigen().eigenvectors;
        let ground_state = vec![ 0.0001265523919516028 ,
         0.0033487321195564265,
           0.05097922316699938,
            0.3668232006272171,
           0.8518575330001764,
           0.3668232006272265,
          0.05097922316700132,
         0.0033487321195565883,
         0.0001265523919514923];



        for i in 0..N_STATES {
            let myindex = i *N_FINE_MOMENTUM /(N_STATES -1);
          //  psitemp[myindex]=Complex64::new(ground_state[i], 0.0) ;
           // psitemp[i] *=  Complex64::new(f64::exp( - ((-max_p + delta_p*(i as f64))/SIGMA_P  ).powi(2) ), 0.0)  ;
            let current_height = Complex64::new(ground_state[i], 0.0) ;
            for n in 0..=N_FINE_MOMENTUM {
               psitemp[n]+= current_height*Complex64::new(f64::exp( - ((delta_p*(n as f64 -myindex as f64))/SIGMA_P  ).powi(2) ), 0.0) ;
            }
        }

        psitemp /= Complex64::new(psitemp.norm(), 0.0);

        
       // println!("{}", &h0);
        Self {
            g: acceleration,
            q: 0.0,
            depth: latticedepth,
            time: 0.0,
            h0 ,
            h1 ,
            h2 ,
            psi: psitemp,
            dpsi_a: DVector::from_element( N_STATES,Complex64::new(0.0,0.0)),
            dpsi_v: DVector::from_element( N_STATES,Complex64::new(0.0,0.0)),
            delta_p    }
            // So for 9 states, we start with 
            // -8hbar kL, -8hKL+d_p, ... -8hbar k_L + d_p* N_FINE_MOMENTUM= -8+ 16/N_FINE_MOMENTUM*N_FINE_MOMENTUM = 8hkL
            // Our Hamiltonian matrices are now N_FINE_MOMENTUM +1 x N_FINE_MOMENTUM sized
            // N_FINE_MOMENTUM consistutes a 16hk_L addition
            // 
            // This also means that if p_index = 0 when -8hk_L, we get -6hk_L when 
            // delta_p_index = N_FINE_MOMENTUM/(N_STATES - 1)
            // Since then -8 + delta_p_index*delta_p= -8 + 2
    }
}

impl Realistic_Lattice {

    pub fn acc_qfi(&self)-> f64 {
        let dpsi_a_c = self.dpsi_a.clone();
        let psi_c = self.psi.clone();

        4.0 * ( dpsi_a_c.conjugate().dot(&dpsi_a_c) - (dpsi_a_c.conjugate().dot(&psi_c) ).norm().powi(2)    ).re
    }

    pub fn acc_cfi(&self) -> f64 {
        let dpsi_a_c = self.dpsi_a.clone();
        let psi_c = self.psi.clone();

        let p_a : f64 = dpsi_a_c.iter().zip(psi_c.iter()).map(|(&dp, &p)|
                    { (2.0*(dp*p.conj()).re ).powi(2)/( p.norm_sqr() )}
                    ).collect::<Vec<f64>>().iter().sum();

        p_a
    }

    pub fn depth_qfi(&self)-> f64 {
        let dpsi_v_c = self.dpsi_v.clone();
        let psi_c = self.psi.clone();

        4.0 * ( dpsi_v_c.conjugate().dot(&dpsi_v_c) - (dpsi_v_c.conjugate().dot(&psi_c) ).norm().powi(2)    ).re
    }

    pub fn depth_cfi(&self) -> f64 {
        let dpsi_v_c = self.dpsi_v.clone();
        let psi_c = self.psi.clone();

        let p_v : f64 = dpsi_v_c.iter().zip(psi_c.iter()).map(|(&dp, &p)|
                    { (2.0*(dp*p.conj()).re ).powi(2)/( p.norm_sqr() )}
                    ).collect::<Vec<f64>>().iter().sum();

        p_v


    }

    pub fn acc_depth_cfi(&self) -> f64 {
        let dpsi_v_c = self.dpsi_v.clone();
        let psi_c = self.psi.clone();

        let dpsi_a_c = self.dpsi_a.clone();

        let dp_v = dpsi_v_c.iter().zip(psi_c.iter()).map(|(&dp, &p)|
        { 2.0*(dp*p.conj()).re  }).collect::<Vec<f64>>();

        let dp_a = dpsi_a_c.iter().zip(psi_c.iter()).map(|(&dp, &p)|
        { 2.0*(dp*p.conj()).re  }).collect::<Vec<f64>>();

        /*let i_av : f64 = dp_a.iter().zip( dp_v.iter()).zip(psi_c.iter()).map( |(&dpa , &dpv, &p)| {
            dpa*dpv/p.norm_sqr()
        }).collect::<Vec<f64>>().iter().sum();*/

        let mut i_av = 0.0;

        for (dpa, dpv, p) in izip!( dp_a.iter(), dp_v.iter(), psi_c.iter()) {
            i_av += dpa*dpv/p.norm_sqr();
        }
        
        i_av

        
    }
}