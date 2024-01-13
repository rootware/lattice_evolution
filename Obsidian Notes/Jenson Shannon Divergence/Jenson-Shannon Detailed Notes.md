## List of Sequences
We have the following sequences for comparison
- 1param training on $I_{aa}$ 
- Multiparameter training on the CFIM of $(a,V_0)$. All these were trained on $(I^{-1})_{aa}$. For these, we have three sequences:
	- _Option#1_ : a sequence with moderate CFIM element but high $I_{aa}$.
	- _Option#2_ : a sequence with very small $|I_{aV_0}|$.
	- One I found using hyperparameter searches (reduced replay buffer size, higher epsilon) with $1/((I^{-1})_{aa}F_{MZ})=1.92$
## Jenson Shannon: Theory

### What the JSD captures
The JSD allows, in the limit that we have perfectly measured a underlying $P(p|a,V)$, how best we can distinguish this momentum distribution from those for other parameter values $(a',V_0')$. Our ability to distinguish the momentum distributions in parameter space reflects the sensitivity of our sensor to the parameters.

We do expect certain behaviours from our JSD's from theory. For acceleration sensitive sequences,
- We expect high JSD divergence for small changes in acceleration, since this is what we optimized the training.
- We expect that the JSD for lattice depth to have a broad band of indistinguishable sequences with very low JSD, since we would like our sensors to be insensitive to the lattice depth.
- We expect that the acceleration JSD should have  a repeating structure. The presence of acceleration in the Hamiltonian amounts to a phase shift, and given inversion symmetry, we expect that accelerations that are modulus $2\pi$ of the phase shift should have the same JSD. This should effectively give rise to a _grid like pattern_ near $(a',a)=(0, 0)$.

For lattice sensitive sequences, we expect high JSD variance near the origin of the lattice auto-correlation plot, and a broad insensitive band of low JSD values in the acceleration auto-correlation plot.

### Short parameter expansion

We define
1. $JSD(a'a,)$ as the JSD between momentum probability distributions with same $V_0$, usually $V_0=10.0$. 
2. $JSD(V_0',V_0)$ as JSD for probability distributions with same $a$, usually $a=0.0$.
3. $JSD(a',V_0')$ as JSD between probability distributions with $(a', V_0=10.0)$ and $(a=0.0, V_0')$ with $(a',V_0')$ varying.
To bound $JSD\in [0,1]$, we use $\log_2$ .

For these, we can expand in $\delta_a, \delta_V$ and obtain the following relations:
1. $$ JSD(a',a)= \frac{1}{8\ln 2} \delta_a^2 I_{aa}$$
2. $$ JSD(V_0',V_0)= \frac{1}{8\ln 2} \delta_V^2 I_{VV}$$
3. $$JSD(a',V_0') =  JSD(a',a)= \frac{1}{8\ln 2} \delta_a^2 I_{aa} + \frac{1}{8\ln 2} \delta_V^2 I_{VV} - \frac{\delta_a \delta_V}{\ln 2} I_{aV}$$
Our outstanding questions are the following:
- Can we extract $I_{aa}$ from JSD, and does it match what we expect from the CFI the RL claims for the sequences?
- Over what range of $\delta_a,\delta_V$ are these expansions valid? Can we use this to obtain a _confidence interval_ for our sequences.
- How quickly do the $JSD$ deviate from these predictions?

One caveat:
This expansion assumes that the probability distributions $P(p|a,V)$ maintain the same functional form as $(a,V)$ are varied. In reality, it is quite likely that varying $(a,V)$ can give rise to differences in the functional form of $P$, which is why in some literature, the connection between $JSD$ and the Fisher information metric is written independently of $P$. Also, for full accuracy for larger $\delta_a$, one should also consider functional variation $\delta P$.


## Results and Discussion: Text

<div style="page-break-after: always;"></div>

## Results and Discussion: Plots

### Single Param


![[Data/Rust_Runs/1param_acc_fig2/JS_acc.png]]

![[Data/Rust_Runs/1param_acc_fig2/JS_cross.png]]


![[Data/Rust_Runs/1param_acc_fig2/JS_latt.png]]

![[1param_Bayesian.pdf]]

### Option 1

![[Data/Rust_Runs/Option1_Rust/JS_acc.png]]

![[Data/Rust_Runs/Option1_Rust/JS_cross.png]]
![[Data/Rust_Runs/Option1_Rust/JS_latt.png]]



### Option 2
![[Data/Rust_Runs/Option2_Rust_Run2/JS_acc.png]]
![[Data/Rust_Runs/Option2_Rust_Run2/JS_cross.png]]
![[Data/Rust_Runs/Option2_Rust_Run2/JS_latt.png]]![[JS_acc_finerfit_CFI.png]]
![[JS_acc_fit_CFI.png]]
### Shah's CFIM seq with $1.92F_{MZ}$
![[Data/Rust_Runs/Shahs_ReduceReplayBuffer_CFIM/JS_acc.png]]![[Data/Rust_Runs/Shahs_ReduceReplayBuffer_CFIM/JS_cross.png]]
![[Data/Rust_Runs/Shahs_ReduceReplayBuffer_CFIM/JS_latt.png]]
