/*!
   Selection of existing functions to eliminate composites of certain forms. All functions return true if the condition is met

     Example of usage

   ```
   use f_analysis::CompVector;
   use f_analysis::filters::{Base,NQR};

      let ce = CompVector::<u64>::from_vector(vec![52,341,561,2047]);
      // Base 2 fermat test removes 52
      assert_eq!(ce.filter_fermat::<Base<2>>(),CompVector::from_vector(vec![341,561,2047]));
      // Base 2 Strong Fermat test removes all but the strong pseudoprime 2047
      assert_eq!(ce.filter_sprp::<Base<2>>(),CompVector::from_vector(vec![2047]));
      // The Carmichael number 561 remains, even after a special base selection
      assert_eq!(ce.filter_fermat::<NQR>(),CompVector::from_vector(vec![561]));
      // Strength the test with the same selection method, and there are no composites left
      assert_eq!(ce.filter_sprp::<NQR>(),CompVector::from_vector(vec![]));
      // Likewise use the first 2 prime bases 2,3. Carmichael remains in the weak test
      assert_eq!(ce.filter_fermat::<PFB<2>>(),CompVector::from_vector(vec![561]));
      // But is eliminated in the strong version
      assert_eq!(ce.filter_sprp::<PFB<2>>(),CompVector::from_vector(vec![]));

   ```
*/
pub(crate) mod filtertype;
pub(crate) mod form;
pub(crate) mod fermat;
pub(crate) mod ftraits;

pub use filtertype::*;
pub use ftraits::{WeakFermat,EulerFermat,StrongFermat,Coprime,FormCheck};

/*
use crate::fermat::FInteger;

/// Trait implementing the a^(n-1)=1 (mod n) Fermat test
pub trait WeakFermat {
    fn fermat<T: FInteger>(x: T) -> bool;
}

/// Trait implementing the a^(n-1)=1 (mod n) Fermat test
pub trait EulerFermat: WeakFermat {
    fn efermat<T: FInteger>(x: T) -> bool;
}

/// Trait implementing the strong variant
pub trait StrongFermat: EulerFermat {
    fn sprp<T: FInteger>(x: T) -> bool;
}

/// Trait implementing checks for integers coprime to some set of integers
pub trait Coprime {
    fn coprime<T: FInteger>(x: T) -> bool;
}

/// Trait implementing checks for integers of a certain form, true means that the integer is of that form
pub trait FormCheck {
    fn is_form<T: FInteger>(x: T) -> bool;
}

/*

  Structures/Types

*/

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

/// All integers of the form K^n,perfect powers with base K
pub struct Power<const K: usize>;




impl<const S: usize> WeakFermat for Base<S> {
    fn fermat<T: FInteger>(x: T) -> bool {
        x.fermat(T::from_u64(S as u64))
    }
}

impl<const S: usize> EulerFermat for Base<S>{
   fn efermat<T: FInteger>(x: T) -> bool{
      x.euler_jacobi(T::from_u64(S as u64))
   }
}


impl<const S: usize> StrongFermat for Base<S> {
    fn sprp<T: FInteger>(x: T) -> bool {
        x.sprp(T::from_u64(S as u64))
    }
}

impl<const S: usize, const P : usize> WeakFermat for DBase<S,P> {
    fn fermat<T: FInteger>(x: T) -> bool {
        if !x.fermat(T::from_u64(S as u64)){
           return false
        }
        x.fermat(T::from_u64(P as u64))
    }
}

impl<const S: usize, const P: usize> EulerFermat for DBase<S,P>{
   fn efermat<T: FInteger>(x: T) -> bool{
      if !x.euler_jacobi(T::from_u64(S as u64)){
           return false
        }
        x.euler_jacobi(T::from_u64(P as u64))
   }
}


impl<const S: usize, const P : usize> StrongFermat for DBase<S,P> {
    fn sprp<T: FInteger>(x: T) -> bool {
        if !x.sprp(T::from_u64(S as u64)){
           return false
        }
        x.sprp(T::from_u64(P as u64))
    }
}

impl<const S: usize, const P : usize, const Q: usize> WeakFermat for TBase<S,P,Q> {
    fn fermat<T: FInteger>(x: T) -> bool {
        if !x.fermat(T::from_u64(S as u64)){
           return false
        }
        if !x.fermat(T::from_u64(P as u64)){
           return false
        }
        x.fermat(T::from_u64(Q as u64))
    }
}

impl<const S: usize, const P : usize, const Q: usize> EulerFermat for TBase<S,P,Q> {
    fn efermat<T: FInteger>(x: T) -> bool {
        if !x.euler_jacobi(T::from_u64(S as u64)){
           return false
        }
        if !x.euler_jacobi(T::from_u64(P as u64)){
           return false
        }
        x.euler_jacobi(T::from_u64(Q as u64))
    }
}

impl<const S: usize, const P : usize, const Q: usize> StrongFermat for TBase<S,P,Q> {
    fn sprp<T: FInteger>(x: T) -> bool {
        if !x.sprp(T::from_u64(S as u64)){
           return false
        }
        if !x.sprp(T::from_u64(P as u64)){
           return false
        }
        x.sprp(T::from_u64(Q as u64))
    }
}

impl<const S: usize> Coprime for Trial<S> {
    fn coprime<T: FInteger>(x: T) -> bool {
        if S > 128 {
            panic!("Cannot support trial division that high")
        }
        x.trial_bound(S)
    }
}

impl<const S: usize> Coprime for GCD<S> {
    fn coprime<T: FInteger>(x: T) -> bool {
        x.gcd(T::from_u64(S as u64)) == T::ONE
    }
}

impl<const A: usize> FormCheck for SPK<A> {
    fn is_form<T: FInteger>(x: T) -> bool {
        x.is_semiprime_k(A)
    }
}

impl<const A: usize> FormCheck for Power<A> {
    fn is_form<T: FInteger>(x: T) -> bool {
        x.is_power_of(A)
    }
}

impl<const A: usize> FormCheck for SPKA<A> {
    fn is_form<T: FInteger>(x: T) -> bool {
        for i in 2..A {
            if x.is_semiprime_k(i) {
                return true;
            }
        }
        return false;
    }
}

impl<const S: usize> WeakFermat for PFB<S> {
    fn fermat<T: FInteger>(x: T) -> bool {
        const BASES: [u64; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

        for i in BASES[..S].iter() {
            if !x.fermat(T::from_u64(*i)) {
                return false;
            }
        }
        true
    }
}

impl<const S: usize> EulerFermat for PFB<S> {
    fn efermat<T: FInteger>(x: T) -> bool {
        const BASES: [u64; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

        for i in BASES[..S].iter() {
            if !x.euler_jacobi(T::from_u64(*i)) {
                return false;
            }
        }
        true
    }
}

impl<const S: usize> StrongFermat for PFB<S> {
    fn sprp<T: FInteger>(x: T) -> bool {
        const BASES: [u64; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

        for i in BASES[..S].iter() {
            if !x.sprp(T::from_u64(*i)) {
                return false;
            }
        }
        true
    }
}
/*
impl WeakFermat for Miller{

  fn fermat<T: FInteger>(x: T) -> bool{
     let bound = x.ln().powi(2)*2;
  }

}

impl StrongFermat for Miller{

  fn sprp<T: FInteger>(x: T) -> bool{

  }

}
*/

impl WeakFermat for NQR {
    fn fermat<T: FInteger>(x: T) -> bool {
        if x.is_square() {
            return false;
        }

        let mut witness = T::from_u64(3);
        loop {
            if x.jacobi(witness) == -1 {
                break;
            }
            witness.successor();
        }

        x.fermat(witness)
    }
}

impl EulerFermat for NQR {
    fn efermat<T: FInteger>(x: T) -> bool {
        if x.is_square() {
            return false;
        }

        let mut witness = T::from_u64(3);
        loop {
            if x.jacobi(witness) == -1 {
                break;
            }
            witness.successor();
        }

        x.euler_jacobi(witness)
    }
}

impl StrongFermat for NQR {
    fn sprp<T: FInteger>(x: T) -> bool {
        if x.is_square() {
            return false;
        }

        let mut witness = T::from_u64(3);
        loop {
            if x.jacobi(witness) == -1 {
                break;
            }
            witness.successor();
        }

        x.sprp(witness)
    }
}
*/
