use crate::{natural::factor::Factorization, CompVector, FResult, Interval, Pseudoprime};

///  
///  Trait implementing necessary functions for generic evaluation
///
pub trait Natural:
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
    + std::convert::From<u64>
    + std::str::FromStr
    + 'static
{
    /// Additive Identity; 0
    const ZERO: Self;
    /// Multiplicative Idenity; 1
    const ONE: Self;
    /// Number of bytes used in representation
    const BYTE_LENGTH: usize;

    fn is_even(&self) -> bool;
    ///
    fn product_residue(&self, other: &Self, n: &Self) -> Self;

    /// Evaluates if integer is between inf and sup exclusive
    fn is_bounded_by(&self, inf: Self, sup: Self) -> bool;

    /// Returns the Minimum and Maximum of a 2-tuple
    fn min_max(&self, otra: Self) -> (Self, Self);

    fn wrapping_sub(&self, otra: Self) -> Self;

    /// (x*multiplier / 2^shift) mod 2^32
    fn hash_shift(&self, shift: usize, multiplier: u32) -> usize;

    fn is_semiprime_k(&self, a: usize) -> bool;

    // 4x+1 component to the number of the form (x+1)(4x+1)
    fn even_complement(&self, k: Self) -> Self;

    fn semi_k_complement(&self, k: usize) -> Self;

    fn overflowing_mul(&self, otra: Self) -> (Self, bool);

    /// Evaluates if integer is coprime to all primes under the provided bound provided that  (exclusive)
    fn trial_bound(&self, s: usize) -> bool;

    /// Is divisible by some 64-numbers
    fn div_vector(&self, divisors: &[u64]) -> bool;

    /// Factors between 3 and 727
    fn small_factor(&self) -> Vec<u64>;

    fn euclidean(&self, otra: Self) -> (Self, Self);

    fn to_u64(&self) -> u64;

    /// Number of bytes used in representation
    fn byte_length() -> usize;
    /// Position of highest set bit
    fn msb(&self) -> usize;
    /// From Little-Endian bytes
    fn from_bytes(x: &[u8]) -> Self;
    /// To Little-Endian bytes
    fn to_bytes(&self) -> Vec<u8>;
    /// Randomly generate guaranteed composite of k-bit length
    fn comp_gen_k(k: usize) -> Option<Self>;
    /// Randomly generate guaranteed prime of k-bit length
    fn prime_gen_k(k: usize) -> Option<Self>;
    /// Randomly generate integer of k-bit length
    fn gen_k(k: usize) -> Option<Self>;

    /// Successor function; X+1
    fn successor(&mut self);
    ///
    fn inc_by(&mut self, inc: u64);

    fn is_multiple_of(&self, factor: u64) -> bool;

    ///  Greatest common divisor
    fn gcd(&self, other: Self) -> Self;

    fn lcm(&self, otra: Self) -> Option<Self>;

    ///  Finite ring gcd
    fn extended_gcd(&self, ring: Self) -> (Self, Self, Self);
    /// GCD and left hand Bezout coefficient
    fn gcd_bz(&self, otra: Self) -> (Self,Self);
    /// Multiplicative inverse
    fn mul_inverse(&self, ring: Self) -> Option<Self>;

    /// x^p mod n
    fn exp_residue(&self, p: Self, n: Self) -> Self;

    /// x^p mod n = 1. where gcd(p,n) = 1
    /// This function is often faster than exp_residue,
    /// except in the case where p \in 2Z+1 AND n \in 4Z
    fn exp_unit(&self, p: Self, n: Self) -> bool;

    /// Count of the fermat pseudoprimes,strong pseudoprimes
    fn pseudoprime_count(&self) -> (Self, Self);

    // FIXME Remove this?
    fn semi_fermat(&self, p: Self, q: Self) -> bool;

    // Strong-fermat to a semiprime with provided factors p,q
    // fn semi_sprp(&self, p: Self, q: Self) -> bool;

    fn sqr_fermat(&self, p: Self) -> bool;
    /// Jacobi symbol
    fn jacobi(&self, other: Self) -> i8;

    /// Classifies the integer into Composite or some type of Fermat Pseudoprime
    /// This functions runs in the same amount of time as a single fermat test
    fn fast_classify(&self, a: Self) -> Pseudoprime;

    /// Classifies the integer into Composite, a type of Pseudoprime or a Prime
    /// This functions runs in the same time as a single fermat test plus is-prime
    fn classify(&self, a: Self) -> Pseudoprime;
    /// a^p-1 mod p = 1
    fn fermat(&self, a: Self) -> bool;
    /// a^p-1 mod p = jacobi(a,p)
    fn euler_jacobi(&self, a: Self) -> bool;
    /// Colin Plumb's variant of Fermat test
    fn euler_p(&self) -> bool;
    /// Strong Fermat
    fn sprp(&self, a: Self) -> bool;
    // Performs a strong fermat test to a certain power, Analog to exp_unit
    // N must be \in 2Z+1
    fn special_sf(&self, p: Self,n: Self) -> bool;
    /// Deterministic primality, using a combination of machine-prime (J.A Sory's parameters),
    /// and a modified BPSW test
    /// This function is used to generate probable primes or composites (guaranteed for n < 2^64)
    fn is_prime(&self) -> bool;

    fn is_perfect_power(&self) -> bool;

    fn is_power_of(&self, x: usize) -> bool;
    /// Integer sqrt
    fn isqrt(&self) -> Self;
    /// Integer nth root
    fn nth_root(&self, n: usize) -> Self;

    fn max_exp(&self) -> (Self, Self);

    fn is_square(&self) -> bool;
    // Factorisation of N
    fn factor(&self) -> Option<Factorization<Self>>;

    // General multiplicative order
    fn ord(&self, a: Self) -> Option<Self>;

    // Multiplicative order for P and some A guaranteed to be coprime to P
    // P= 2 is not supported. This is to be handled separately
    fn p_ord(&self, a: Self) -> Self;
    // Returns order and the signature (the largest factor of 2 dividing the order)
    fn signature(&self, a: Self) -> Option<(Self, u32)>;
    // P=2 is not supported
    fn signature_v(&self, base: &[Self]) -> Option<(Self, Vec<u32>)>;
}

pub trait PrimalityTest {
    fn primality<T: Natural>(&self, cmp: T) -> Self;
    // Prove correct over interval
    fn prove_interval<T: Natural>(&self, cmp: Interval<T>) -> FResult<T>;
    // Prove correct over some composite set
    fn prove_set<T: Natural>(&self, cmp: CompVector<T>) -> FResult<T>;
}
