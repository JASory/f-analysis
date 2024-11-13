# f-analysis
Number-theoretic library for analysis of Fermat base selection. 

Highly experimental. Api has zero stability, and functions may give erroneous values. 

Currently only supports up to 128-bit. Uses cpu threads for computation. The more cores available the faster. 

The goal of this library is to provide algorithms for statistical analysis of fermat bases and testing of the reasonableness of conjectures. It is not currently able to replicate the hardest computations, however it provides a greater range of analysis than the current published tables. 

Current capability

- Computing fermat pseudoprimes to any base within 2;2^64. Note that computing pseudoprimes in intervals greater than 10^12 is impractical
- Filtering by Euler-Jacobi, Strong Fermat, and first non-quadratic base.
- Filtering by coprimality, and certain forms of semiprimes
- Construction of fermat base hashtables, as used in [machine-prime](https://github.com/JASory/machine-prime)
- Heuristic strong pseudoprime generation.
- Iterative selection of bases up to a bound. (e.g picking the strongest base up to a bound, then picking the strongest base against the previous set). Combined with the heuristic prime generation this results in frequently deterministic base sets, although impractical to prove. 

Future capability 
- Evolutionary search of bases (probably not advantageous)
- Generating Carmichael numbers
- Faster generation of pseudoprimes
- Computing bounds for pseudoprimes for a set of bases (a la Jaeschke)
- Possibly implement Euler-Plumb, and modified fermat tests
