## Mach Zehnder CFI
Internal notes on CFI references and other values, using notation from internal gravimetry
- $$I_{MZ} = \frac{(2p_0 T_\pi^2)^2}{\hbar^2}$$ where $p_0$ is the momentum of each arm, $2p_0$ therefore is the splitting, and $T_\pi$ is the time till mirror pulse and also time after mirror pulse to end of sequence.
- In code units, this becomes $$I^{code}_{MZ} = \left[4 n \left(\frac{N_\phi\pi}{2\omega_0}\right)^2\right]^2 = \frac{n^2N_\phi^4 \pi^4}{\omega_0^4}$$ where we obtain another factor of 2 due to conversion into code units , and $n$ denotes which momentum state forms the $\pm$ arms. Usually, for $p_0=4\hbar k_L$, $n=4$. Also, $N_\phi$ is no. of shaking functions in sequence, $\omega_0=11.5\omega_r$ usually. The factor of $2$ in denominator is because $T_\pi$ only contains $N_\phi/2$ shaking functions.
- For a full sequence of $N_\phi=32$, $n=4$, this gives us $I^{code}_{MZ}=93438.966$ in code units.

## Periodicity in acceleration
The relative phase due to acceleration accrues as $\phi_a = 2ap_0 T_\pi^2 /\hbar$ . Factor of 2 since splitting is $2p_0$. Then in code units
-  $$\phi_a = 2 *\tilde{a} v_r \omega_r*n \hbar k_L * \tilde{T_\pi}^2 \omega_r^{-2} /\hbar $$ or 
- $$\tilde{\phi_a} = 2 *\tilde{a}n \tilde{T_\pi}^2 \left( v_r \omega_r* \hbar k_L *  \omega_r^{-2} /\hbar \right) $$
- The units yield
 $$v_r \omega_r^{-1} k_L = \left( \frac{\hbar k_L}{m} \right)\left(\frac{2m}{\hbar k_L^2}\right)\frac{1}{\hbar} =2 $$
 - So $$\tilde{\phi_a} = 2 \tilde{a} n \tilde{T_\pi}^2 *2= \tilde{a}n \tilde{\mathcal{T}}^2 $$where $\mathcal{T}$ is total time of sequence, or $\mathcal{T}= \frac{\pi N_\phi}{\omega_0}$ .
 - In terms of periodicity, we therefore expect this phase to wrap around every $2\pi s$, where $s\in \mathbb{N}$ is a natural no. This yields the periodicity as $$\tilde{a}_s = \frac{2\pi}{n \mathcal{T}^2}=0.020554923$$ or $a = 0.11520051769673768 g$ .


## Differentiating Probability distributions

$$\partial_a P(p) = \partial_a |\psi(p)|^2 =\partial_a \left(\braket{\psi|p}\braket{p|\psi} \right)= (\partial_a \braket{\psi|p}) \braket{p|\psi} + \braket{\psi|p} (\partial_a \braket{p|\psi} )= 2 \mathrm{Re}\left[\braket{\psi|p} \partial_a\braket{p|\psi}\right]$$
$$I_{aa} = \sum_p \frac{1}{P(p)} 4 \left(\mathrm{Re}\left[\braket{\psi|p} \partial_a\braket{p|\psi}\right]\right)^2$$