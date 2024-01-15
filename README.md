# Shaken Lattice Statistical Analysis (Rust version)
This project began as a learning experience: to code the C++ version (see Github: End2End_SLI) of the SLI in Rust. However, this has since become the main code for generating momentum probabilities and doing the statistical analysis section of the paper. Main differences from C++ code:
- Implemented multithreading using Rayon crate for iterators, so faster runs
- Was able to do more precise runs, and hence do faster comparisons between JSD plots.
- The C++ code library includes the _Environment_ class that I used for training with the Libtorch DQNN implementation in the Holland Group. Here, we have a simpler struct _Lattice_ that differs slightly from the _Wavepacket_ class in C++.
- The Rust style docs are available as .html under docs/ folder.
- The Obsidian folder contains notes for project in Markdown.

### Planned Updates:
- Adding augmented state evolution to code.
- Moving Bayesian updating to Rust from Python as well.

