#Lambda = 852*10**(-9);
import numpy as np;
Lambda = 1064*10**(-9);
k_L = 2*np.pi / Lambda;
hbar = 1.054571817*10**(-34);
atomicmass = 86.90918;
massDalton = 1.660539*10**(-27);
mass = atomicmass * massDalton;
E_R = (hbar * k_L)**2 /(2*mass);
omega_R = E_R/hbar;
g=9.81;
accUnit= 2*(E_R)**2/(hbar**2*k_L);

n_p =4; T= 32*np.pi/(2*11.5); print(T);
F_MZ = (4*n_p *T**2)**2;
Iaa=1.701*F_MZ;
IVV=0.641;