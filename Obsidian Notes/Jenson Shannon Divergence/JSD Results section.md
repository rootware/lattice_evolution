List of remaining modifications to plots:
- [ ] Change to .pdfs and change $V_0$ to $V_L$
- [ ] Re-run the Bayesian plots with finer-grained acceleration
	- [ ] Zoom in on certain ranges of acceleration for Bayesian plots
	- [ ] Change the JSD plots to be tiled together using subplot

## JS Results scratch

**Plots**
The plots of the different JSDs  are given in Fig.\ref{fig:Jenson-Shannon} for the same sequence used in Fig.\ref{fig:2d_fringes}. 
The JSD for acceleration vs acceleration contains a lot of information. For small $\delta_a$ around zero acceleration, the JSD increases sharply, indicating high sensitivity. JSD is also only exactly zero when the parameters for the distributions are identical. However, we see the presence of off-diagonals with low JSD, resulting from a spatial inversion symmetry. Furthermore, there's a clear repeating grid line pattern of low JSD values.  This periodic pattern arises because the presence of acceleration is (under a rotating frame transformation, see \cite{Catie} equivalent to a phase shift, and therefore, for fixed time $t=\mathcal{T}$, wraps around every for every $(a',a)$ with $- k_L (a'-a)\mathcal{T}^2   = 2\pi n, n\in \mathbb{Z}$ . The periodic structure decays for large $a'-a$ since the interferometer is trained for maximum sensitivity near $a=0.0$.  

We can further verify this by looking at $D_{JS}(a',a=0.0)$, shown in Fig.\ref{fig:Jenson-Shannon}, and doing a quadratic fit to extract the effective $I_{aa}$ value. Our fit yields an effective $I^{JSD}_{aa}=1.68F^{MZ}_{aa}$, with $I^{JSD}_{aa}=0.9878I^{RL}_{aa}$, where $I^{RL}_{aa}=1.701 F^{MZ}_{aa}$ is the CFI predicted by reinforcement learning code for this sequence.

For lattice depth, we see a very broad region of low JSD: exactly what we would hope from a sensor designed to be insensitive to lattice depth ($I_{V_L V_L}= 6.86*10^{-6}F^{MZ}_{aa}$ ). We also  specifically aimed in this sequence to reduce the sensor's sensitivity's cross correlation between acceleration and lattice depth ($I_{aV_L}=1.31*10^{-5}F^{MZ}_{aa}$). Looking at Fig.\ref{fig:JS_cross}, we observe a horizontal band of small JSD values showing a broad insensitivity range for $V_L$ for accelerations close to zero.


