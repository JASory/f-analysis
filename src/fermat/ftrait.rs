use crate::{Pseudoprime,result::FResult,CompVector,Interval};
///  
///  Trait implementing necessary functions for generic evaluation
///
pub trait FInteger:
    Clone
    + Copy
    + std::fmt::Display
    + std::fmt::Debug
    + Default
    + std::hash::Hash
    + std::cmp::PartialEq
    + std::cmp::Eq
    + std::cmp::Ord
    + std::marker::Send
    + std::marker::Sync
    + 'static
{   /// Additive Identity; 0
    const ZERO: Self;
    /// Multiplicative Idenity; 1
    const ONE: Self;
    /// Number of bytes used in representation
    const BYTE_LENGTH : usize;
    
    
    fn is_even(&self) -> bool;
    /// 
    fn product_residue(&self, other: &Self, n: &Self) -> Self;
    
    /// Evaluates if integer is between inf and sup exclusive
    fn is_bounded_by(&self, inf: Self, sup: Self) -> bool;
    
	/// Returns the Minimum and Maximum of a 2-tuple
	fn min_max(&self,otra: Self) -> (Self,Self);
	
	fn wrapping_sub(&self, otra: Self) -> Self;
	
    /// (x*multiplier / 2^shift) mod 2^32
    fn hash_shift(&self, shift: usize, multiplier: u32) -> usize;
    
    /// Classifies the integer into the result of 
    fn classify(&self, a: Self) -> Pseudoprime;

    fn is_semiprime_k(&self, a: usize) -> bool;
    
    // 4x+1 component to the number of the form (x+1)(4x+1)
    fn even_complement(&self,k: Self) -> Self;
	
    fn semi_k_complement(&self, k: usize) -> Self;
	 
    fn overflowing_mul(&self, otra: Self) -> (Self,bool);
    
    /// Evaluates if integer is coprime to all primes under the provided bound (exclusive)
    fn trial_bound(&self, s: usize) -> bool;
    
    /// Is divisible by some 64-numbers
    fn div_vector(&self, divisors: &[u64]) -> bool;
    
    /// Factors between 2 and 311 
    fn small_factor(&self) -> Vec<u64>;
    
    fn euclidean(&self, otra: Self) -> (Self,Self);
    
    fn to_u64(&self) -> u64;
	
    fn from_u64(k: u64) -> Self;
    
    /// Number of bytes used in representation
    fn byte_length() -> usize;
    
    /// From Little-Endian bytes
    fn from_bytes(x: &[u8]) -> Self;
    /// To Little-Endian bytes
    fn to_bytes(&self) -> Vec<u8>;
    /// Initialise from string
    fn from_str(x: &str) -> Option<Self>;
    /// Randomly generate guaranteed composite of k-bit length
    fn comp_gen_k(k: usize) -> Option<Self>;
    /// Randomly generate guaranteed prime of k-bit length
    fn prime_gen_k(k: usize) -> Option<Self>;
    /// Randomly generate integer of k-bit length
    fn gen_k(k: usize) -> Option<Self>;
    
    /// Successor function; X+1 
    fn successor(&mut self);
    
    fn inc_by(&mut self, inc: u64);
    
    fn is_multiple_of(&self, factor: u64) -> bool;
    
    ///  Greatest common divisor
    fn gcd(&self, other: Self) -> Self;
    
    fn lcm(&self, otra: Self) -> Option<Self>;
    
    ///  Finite ring gcd
    fn extended_gcd(&self, ring: Self) -> (Self, Self,Self);
    
    /// x^p mod n 
    fn exp_residue(&self, p: Self, n: Self) -> Self;
    
    /// 
    fn semi_fermat(&self,p: Self, q: Self) -> bool;
    
    // Strong-fermat to a semiprime with provided factors p,q
    // fn semi_sprp(&self, p: Self, q: Self) -> bool;
    
    fn sqr_fermat(&self,p: Self) -> bool;
    /// Jacobi symbol
    fn jacobi(&self, other: Self) -> i8;
    /// a^p-1 mod p = 1
    fn fermat(&self, a: Self) -> bool;
    /// a^p-1 mod p = jacobi(a,p)
    fn euler_jacobi(&self, a: Self) -> bool;
    /// Colin Plumb's variant of Fermat test
    fn euler_p(&self) -> bool;
    /// Strong Fermat
    fn sprp(&self, a: Self) -> bool;
    /// Deterministic primality, using a combination of machine-prime (J.A Sory's parameters), 
    /// Sorensen and Webster's parameters and Miller test assuming GRH
    /// This function is used to generate guaranteed composites or primes
    fn is_prime(&self) -> bool;

    fn is_perfect_power(&self) -> bool;
    
    fn is_power_of(&self, x: usize) -> bool;
    /// Integer sqrt
    fn isqrt(&self) -> Self;
    /// Integer nth root
    fn nth_root(&self, n: usize) -> Self;
    
    fn max_exp(&self) -> (Self,Self);

    fn is_square(&self) -> bool;
}

pub trait FIterator<T: FInteger>: Iterator {
    fn new(start: Option<T>, len: usize) -> Option<Self>
    where
        Self: Sized;

    fn state(&self) -> T;

    fn inc(&mut self);

    fn to_vector(&self) -> Vec<T>
    where
        T: Sized;
}

pub trait PrimalityTest {
  fn primality<T: FInteger>(&self, cmp: T) -> Self;
  // Prove correct over interval
  fn prove_interval<T: FInteger>(&self, cmp: Interval<T>) -> FResult<T>;
  // Prove correct over some composite set
  fn prove_set<T: FInteger>(&self, cmp: CompVector<T>) -> FResult<T>; 
}

pub trait NTCore {

   fn mont_sub(&self, y: Self, n: Self) -> Self;
   
   fn mont_add(&self, y: Self) -> Self;
   
   fn inv_2(&self) -> Self;
   
   fn inv_2_neg(&self) -> Self;
   
   fn mont_prod(&self, y: Self, n: Self, npi: Self) -> Self;
   
   fn mont_sqr(&self, n: Self, npi:  Self) -> Self;
   
   fn to_z(&self, n: Self, npi: Self) ->  Self;
   
   fn n_identity(&self) -> Self;
   
   fn to_mont(&self,n: Self) -> Self;
   
   fn mont_pow(&self, one: Self, p: Self, n: Self, npi: Self) -> Self;
   
   fn sprp(&self, base: Self) -> bool;
   
   // Odd only fermat 
   fn odd_fermat(&self, base: Self) -> bool;
   // Prime square
   fn p_sq_fermat(&self, base: Self) -> bool;
   
   fn fermat(&self, base: Self) -> bool;
   /// Odd only 
   fn odd_exp_residue(&self, p: Self, n: Self) -> Self;
   
   /// Power of two ring
   fn even_exp_residue(&self, p: Self, n: Self) -> Self;
   
   fn expr(&self, p: Self, n: Self) -> Self;
   
   fn mul_ord(&self,n:Self) -> Self;
   
   fn bounded_ord(&self, n: Self, sup: Self) -> Self;
}

