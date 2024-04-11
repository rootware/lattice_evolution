## Testing Bayesian Tilt
Our goal in this run is to see if the tilt we see in `Rust_Runs/SPvMPBayesian/` can be replicated or no. 


### Current SP vs MP data
We note that for the previous runs, we had:
1. 501 acceleration grid points, 51 lattice points
2. Our acceleration values range $a \in \{ -0.0225, 0.0225\} \omega_r v_r$ or $a \in \{-0.12610174406014532, + 0.12610174406014532\}g$. This meant a grid spacing of $0.0005044069762405801g$ or $8.99999999999998e-05 \omega_r v_r$.
3. Our lattice depth values range from $V \in \{ 9, 11\}E_R$
4. The tilt plots are plotted from $a \in \{ -0.02g,0.02g \}$ ( $\pm 0.003568546996347399 \omega_r v_r$) for $N=40$ and then $a\in \{-0.005g, 0.005g\}$ ($0.0008921367490868497 \omega_r v_r$ ).
5. We need to really zoom in on these plots

### Setting up new run
1. We use the same outcomes that were used in previous runs. We currently have `500.txt` and `2000.txt` for both MP and SP.
2. I need to install Python lol
3. Try halving grid spacing

Attempted 1001x101 $(a,V)$ run: was aborted, and took too long. See `interrupted_test.txt`.

Try now run with 501x101 but in $a \in \pm 0.01 \omega_r v_r$ 


