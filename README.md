# Shaken Lattice Statistical Analysis (Rust version)
This project began as a learning experience: to code the C++ version (see Github: End2End_SLI) of the SLI in Rust. However, this has since become the main code for generating momentum probabilities and doing the statistical analysis section of the paper. Main differences from C++ code:
- Implemented multithreading using Rayon crate for iterators, so faster runs
- Was able to do more precise runs, and hence do faster comparisons between JSD plots.
- The C++ code library includes the `Environment` class that I used for training with the Libtorch DQNN implementation in the Holland Group. Here, we have a simpler struct `Lattice` that differs slightly from the `Wavepacket` class in C++.
- The Rust style docs are available as .html under docs/ folder.
- The Obsidian folder contains notes for project in Markdown.

### Planned Updates:
- Adding augmented state evolution to code.
- Moving Bayesian updating to Rust from Python as well.

## Using Code
The code is pretty simple.
### Generating Bayesian priors
The Rust code takes in a vector of shaking function amplitudes for the lattice, and generates final momentum probabilities for a range of acceleration and lattice depth values. To build the rust code, just use `cargo build --release` within this directory, and to run `cargo run --release`.
The code will generate a `test.txt` file that contains the following info:
```
[acceleration index, lattice depth index, acceleration value, lattice depth value, P(p)]
```
The no. of values in $P(p)$ is determined by `N_STATES` constant.

### Running Bayesian Updating and Plotting
There are three example notebooks for this:
- 1param Bayesian
- JSD plots for multiparameter
- Comparing Bayesian between two sequences

While the notebooks may illustrate multiple different ways of plotting, recommend using them only as illustrative examples. Use the `Sequence2param.py` and `Units.py` files when doing your own plots for consistency.