  

/// Fermat Base
pub struct Base<const S: usize>;
/// Double Fermat Base
pub struct DBase<const S: usize, const P: usize>;
/// Triple Fermat Base
pub struct TBase<const S: usize, const P: usize, const Q: usize>;

/// First Non-quadratic residue fermat base selection
pub struct NQR;

/// Euler-Plumb Fermat test (modified 2-fermat)
pub struct EPF;

// First quadratic residue fermat base selection
//pub struct QR;

/// Prime First Base, Fermat test using the first S prime bases
pub struct PFB<const S: usize>;

/// Checks if coprime to a constant integer
pub struct GCD<const S: usize>;

/// Checks if coprime to the first S primes, where S < 128
pub struct Trial<const S: usize>;

/// Selection of bases according to the Miller Deterministic Test reliant on GRH 2*ln(n)^2
///
/// Note that in the case of the weakfermat check Miller bases are not deterministic due to Carmichael numbers
pub struct Miller;

/// Monier-Rabin composites of the form (2x+1)(4x+1)
pub struct MRC;

/// Semiprimes of the form (k+1)(ak+1) where A is a constant
pub struct SPK<const A: usize>;

/// All semiprimes of the form (k+1)(ak+1) where A ranges from 2 to X exclusive
pub struct SPKA<const A: usize>;


/// Strong Fermat test of the form SF(n,Sqrt(n/A)+1)
pub struct SFSqr<const A: usize>;

/// All integers of the form K^n,perfect powers with base K
pub struct Power<const K: usize>;

/// All perfect squares 
pub struct Square;

