use crate::math::fermat::Pseudoprime;

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
{
    const ZERO: Self;

    const ONE: Self;

    const BYTE_LENGTH : usize;

    fn is_bounded_by(&self, inf: Self, sup: Self) -> bool;

    fn hash_shift(&self, shift: usize, multiplier: u32) -> usize;
    
    fn classify(&self, a: Self) -> Pseudoprime;

    fn is_semiprime_k(&self, a: usize) -> bool;

    fn trial_bound(&self, s: usize) -> bool;

    fn from_u64(k: u64) -> Self;

    fn byte_length() -> usize;

    fn from_bytes(x: &[u8]) -> Self;

    fn to_bytes(&self) -> Vec<u8>;

    fn from_str(x: &str) -> Option<Self>;

    fn comp_gen_k(k: usize) -> Option<Self>;

    fn prime_gen_k(k: usize) -> Option<Self>;
    
    fn gen_k(k: usize) -> Option<Self>;

    fn successor(&mut self);

    fn gcd(&self, other: Self) -> Self;
    
    fn exp_residue(&self, p: Self, n: Self) -> Self;

    fn jacobi(&self, other: Self) -> i8;

    fn fermat(&self, a: Self) -> bool;
    
    fn euler_jacobi(&self, a: Self) -> bool;
    
    fn euler_p(&self) -> bool;

    fn sprp(&self, a: Self) -> bool;

    fn is_prime(&self) -> bool;

    fn is_perfect_power(&self) -> bool;
    
    fn is_power_of(&self, x: usize) -> bool;

    fn isqrt(&self) -> Self;
    
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
