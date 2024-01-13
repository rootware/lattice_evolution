# Shaken Lattice Statistical Analysis (Rust version)
This project began as a learning experience: to code the C++ version (see Github: ) of the SLI in Rust. However, this has since become the main code for generating momentum probabilities and doing the statistical analysis section of the paper. Main differences from C++ code:
- Implemented multithreading using Rayon crate for iterators, so faster runs
- Was able to do more precise runs, and hence do faster comparisons between JSD plots.
- The C++ code library includes the _Environment_ class that I used for training with the Libtorch DQNN implementation in the Holland Group. Here, we have a simpler struct _Lattice_ that differs slightly from the _Wavepacket_ class in C++.
