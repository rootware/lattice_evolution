use std::f64::consts::PI;

pub const LAMBDA : f64 = 1064.0e-9;
pub const K_L : f64 = 2.0*PI/ LAMBDA;
pub const HBAR : f64 = 1.054571817e-34;
pub const ATOMICMASS : f64 = 86.90918;
pub const MASS_DALTON : f64= 1.660539e-27;
pub const MASS_SI : f64 = ATOMICMASS*MASS_DALTON;
pub const E_R : f64 = (HBAR * K_L)* (HBAR * K_L)/(2.0*MASS_SI);
pub const OMEGA_R : f64 = E_R/HBAR;
pub const G : f64 = 9.81;
pub const ACC_UNIT : f64 = 2.0*(E_R*E_R)/(HBAR*HBAR*K_L);
pub const TIME_UNIT : f64 = 1.0/OMEGA_R;

// K_L = 2*np.pi / Lambda;
// HBAR = 1.054571817*10**(-34);
// ATOMICMASS = 86.90918;
// MASS_DALTON = 1.660539*10**(-27);
// mass = ATOMICMASS * MASS_DALTON;
// E_R = (HBAR * K_L)**2 /(2*mass);
// omega_R = E_R/HBAR;
// g=9.81;
// accUnit= 2*(E_R)**2/(HBAR**2*K_L);

// n_p =4; T= 32*np.pi/(2*11.5); print(T);
// F_MZ = (4*n_p *T**2)**2;
// Iaa=1.701*F_MZ;