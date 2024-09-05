//! F-Analysis (short for Fermat-Analysis) is intended to provide a reliable and reproducible method of research into
//! selection of fermat bases
//!
//! Intended use
//!
//! How to select or even optimise selecting a base for the fast fermat test with few false positives is
//! an open problem in number theory. The purpose of this library is to provide simple to use tools for
//! evaluating the reasonableness of conjectures concerning this problem.  For instance one observation made
//! with this library is that using a particular base selection method with a certain composite generator can
//! fairly reliably produce deterministic tests with less than 1% of the computational cost.
//!
//! Preferred Specifications
//!
//! - 64-bit cpu
//! - As many cpu threads as possible (GPU is not currently supported)
//! - As much RAM as possible. Most operations are limited by persistent harddrive memory not RAM, however RAM is much faster
//! - RDRAND supported, (no operations require randomness without selecting for it)

pub(crate) mod structures;
pub(crate) mod io;
pub(crate) mod math;
pub(crate) mod primes;
pub(crate) mod car;
pub(crate) mod compeval;
pub(crate) mod search;
pub(crate) mod compconfig;

pub mod fermat;
pub mod filter;
 mod result;

pub use math::Pseudoprime;
pub use crate::result::FResult;
pub use crate::fermat::extended::traitimpl::Epz;
pub use crate::structures::{CounterExamples,DataVector,HashTable,Primes,PrimeOrd,CompVector,BaseSeq,Interval,WieferichPrime,Constructor,PCGenerator};

// https://www.reddit.com/r/cryptography/comments/zgal8n/pollard_rho_easy_explanation/

/*
   Research Capability

   Produce Weak Fermat bases, Euler Jacobi, and Strong Fermat pseudoprimes, and Super-Pseudoprimes (composites whose factors are also base-k pseudoprimes)

   Optimize Search for Fermat, Euler and Jacobi pseudoprimes

   Hashtable construction for arbitrary size values

   Search for strongest fermat base, euler pseudoprime, or strong fermat given an interval or composite vector

   Search Modes Random and sequential, Sequential_Probable, Random_Probable {Probable meaning filters the (ak+1)(k+1)}

   Generate composites,

   Things to investigate, do different bases take different numbers of steps?

   Implement R.E Pinch's fermat algorithm, implement Sorenson & Websters algorithm


   Conjectures to test

     Is strength over small intervals a predictor of strength over large intervals?
     Is there a constant by which each strength can be predicted? narayanan's conjecture


     Evaluate if composing bases by multiplication is useful

       A pseudoprime to both base X and base Y is likely to be a pseudoprime to X*Y (possibly true)

       A pseudoprime to either base X and Y is likely to be a pseudoprime to X*Y (possibly false)




 (102, 0.4086139815669888, 0.030853870138539068) Composites
 (102, 0.3979569381069357, 0.02849157713997394) Primes

 (90, 0.48179825424144757, 0.024115337286465123) First 256
 (102, 0.4559031878562129, 0.033900096962721525) Composites
 (102, 0.44407595208509093, 0.03152252321535374) Primes


 2, 9375, 1300


 2,9375,43732, 58860, 55829, 27429, 48694, 3




// [60, 96, 13, 37, 83, 11, 17, 94]
// [15, 96, 13, 37, 34, 59, 67, 7, 11]






[15, 13, 37, 29, 23, 31, 38, 3] 0
[60, 52, 37, 79, 29, 41, 55]  0
[240, 52, 37, 129, 102, 159, 46] 0  256    2,3,5,13,17,23,37,43,53
[240, 52, 37, 258, 386, 167, 17] 0  512    2,3,5,13,17,37,43,193,167,
[1215, 468, 148, 1387, 698, 1769, 7]0 2048 2,3,5,7,17,19,29,37,61,73,349,
[9375, 22932, 17908, 9217, 25265, 9978, 17] 32768

[60, 96, 15, 13, 37, 83, 11, 31, 17] 1 100  130 million Zero filter base

[60, 13, 14, 37, 58, 76, 67, 17]

75, 37, 65, 95, 71, 5


*/
