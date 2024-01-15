## Past Bayesian Analysis
In the past, we used 1pe2e_acc from our C++ runs to do a Bayesian analysis. We obtained 
![[1param_Bayesian.pdf]]
There are a few issues with this plot:
- The standard deviation was calculated with respect to the _mean_ of $P(a,V_0=10.0|\{m_i\})$ for 2000 runs. However, this may not mean that $0.0 \neq \langle a \rangle \equiv  \int a P(a,V_0|\{m_i\})$. In fact, calculating standard deviation with respect to the true mean $\mu = 0.0$ may be quite noisy.
- The plot is labelled wrong: $a$ is in code units, I accidentally multiplied the probability by `accUnit/g` , where $accUnit$ in code gives `realacc = codeacc*accUnit `, and `g=9.81$ms^{-2}$` 
- Our extract CFI was $I^{B}_{aa} \sim 0.94 I^{RL}_{aa}$ .

## New Run_1 : Jan 14
We now rerun using our Rust codebase. 
- Same as for the runs in other Rust settings, we use $a\in {-0.0025, 0.0025}$ in code units. 
- Use 1001 points, instead of 1000, for fine grained resolution.
- For now, we'll fix $V_0 = 10.0$ throughout. Later, we'll also do this for $V_0$.
- Need to change $V_0$ to $V_L$
- 

Questions we would like to answer:
1. How noisy is $\sigma_a$ when we calculate w.r.t $\langle a \rangle$ vs $\mu$
2. Does CFI get better when we fit it for higher no. of runs
3. How much does $\langle a \rangle$ deviate from $\mu$ ? This isn't a question of Bayesian convergence as much as it is a possible issue about our _dynamic range_ .
4. Check that $a=0.0$ is included as a datapoint, at least within Python's floating point error. Since I'm iterating over acceleration values using a formula, occasionally the code records data for $a=5.0*10^{-20}$ or something instead of $a=0.0$. This shouldn't ideally matter much: theoretically, $D_{JS}(a'=\delta_a, a=0.0)= \frac{1}{8\ln 2} \delta_a^2 I_{aa}$.
### Results:
Sanity checks:
- $a=0.0$ is included as a datapoint

 We first look at $\sigma_a$ evaluated from $\langle a \rangle$ . We do Runs = 10,000. Probability plots are recorded every 50 measurements, $\sigma_a$ and $\langle a\rangle$ are recorded every measurement.
- Plot of standard deviation with measurements:
 ![[Pasted image 20240114155927.png]]
- Plot of probability distributions:![[Pasted image 20240114160052.png]]
- Zoomed in probability plot: ![[Pasted image 20240114161047.png]]
- Comparison of $\langle a \rangle$ to $\mu$ where $a$ is in units of $g$: ![[Pasted image 20240114160151.png]]
- Residual:
![[Pasted image 20240114160936.png]]
- Our expected CFI: $I^{RL}_{aa}=1.94F^{MZ}_{aa}$ , we get $I=2.3197900841675843F^{MZ}_{aa}$

## Run_2 with Option 2
Do 2000 outcomes and 25,000 outcomes.

For 25,000 outcomes:
- CFI extracted with prob mean = 1.622253965762357 F_MZ
- Plots are in `Option2_acconly_Bayesian` in Rust runs
  