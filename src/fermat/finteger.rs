use crate::math::fermat::*;
use crate::math::fermat::fsprp;
use crate::math::ntfunc::*;

use crate::fermat::ftrait::FInteger;
use crate::math::rand::{comp_gen_k,prime_gen_k,gen_k};
//use crate::math::rand::prime_gen_k;
use crate::primes::{PRIME_INV_128, PRIME_INV_64};

use machine_prime::is_prime;

impl FInteger for u64 {

    const ONE: u64 = 1;
    const ZERO: u64 = 0;
    const BYTE_LENGTH : usize = 8;

    fn from_u64(x: u64) -> Self {
        x
    }

    fn is_bounded_by(&self, inf: Self, sup: Self) -> bool {
        if *self > inf && *self < sup {
            return true;
        }
        false
    }

    fn byte_length() -> usize {
        8usize
    }

    fn hash_shift(&self, shift: usize, multiplier: u32) -> usize {
        ((*self as u32).wrapping_mul(multiplier) >> shift) as usize
    }

    fn is_semiprime_k(&self, a: usize) -> bool {
        let fctr = a as u64;
        let sq = (*self - 1) / fctr;
        let k = (sq as f64).sqrt() as u64;

        if ((k * k + k) * fctr + k + 1) == *self {
            return true;
        }

        return false;
    }

    fn trial_bound(&self, s: usize) -> bool {
        if *self & 1 == 0 {
            return false;
        }

        if *self < 0x5A2553748E42E8 {
            for i in PRIME_INV_64[..s].iter() {
                if self.wrapping_mul(*i) < *self {
                    return false;
                }
            }
            return true;
        } else {
            for i in PRIME_INV_128[..s].iter() {
                if ((*self as u128).wrapping_mul(*i)) < *self as u128 {
                    return false;
                }
            }
        }
        return true;
    }

    fn from_bytes(x: &[u8]) -> Self {
        Self::from_le_bytes([x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]])
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_str(x: &str) -> Option<Self> {
        match x.parse::<u64>() {
            Ok(z) => Some(z),
            Err(_) => None,
        }
    }

    fn successor(&mut self) {
        *self += 1;
    }

    fn comp_gen_k(k: usize) -> Option<Self> {
        comp_gen_k(k as u64)
    }

    fn prime_gen_k(k: usize) -> Option<Self> {
        prime_gen_k(k as u64)
    }
    
    fn gen_k(k: usize) -> Option<Self>{
        gen_k(k as u64)
    }

    fn gcd(&self, other: Self) -> Self {
        gcd(*self, other)
    }

    fn jacobi(&self, other: Self) -> i8 {
        jacobi(*self, other)
    }

    fn classify(&self, a: Self) -> Pseudoprime{
       fsprp(*self,a)
    }
    
    fn fermat(&self, a: Self) -> bool {
        fermat(*self, a)
    }
    
    fn exp_residue(&self, p: Self, n: Self) -> Self{
       pow_64(*self,p,n)
    }
    
    fn euler_jacobi(&self, a: Self) -> bool{
       let r = a.jacobi(*self);
       if r == -1{
        return a.exp_residue((*self-1)/2,*self) == *self-1 
       }
       a.exp_residue((*self-1)/2,*self)== (Self::from_u64(r as u64))
    }

    fn sprp(&self, a: Self) -> bool {
        sprp(*self, a)
    }

    fn is_prime(&self) -> bool {
        is_prime(*self)
    }
    
    fn euler_p(&self) -> bool{
       let residue = *self&7;
       let mut param = 0;
       
       if residue == 1{ 
          param = 1;
       }
       
       let ap = 2.exp_residue((*self-1)>>(1+param),*self);
       if ap == 1  {
           return (residue == 1 || residue == 7);
       }
       else if ap == *self-1 {
           return (residue == 1 || residue == 3 || residue == 5)
       }  
       return false
    }

    fn is_perfect_power(&self) -> bool {
       if self.max_exp().1 > 1{
         return true
       }
       false
    }
    
    fn is_power_of(&self, x: usize) -> bool{
       if *self == x as u64 {
         return true
       }
       if x == 1 || x == 0{
          return false
       }
       if *self% (x as u64) != 0{
         return false
       }
       let val = x as u64;
       //println!("{}",val);
       for _ in 0..64{
         let (val,flag) = val.overflowing_mul(x as u64);
         
         if flag{ // if overflowed then not perfect power
           return false
         }
         if val == *self as u64{
            return true
         }
       }
       return false
    }

    fn isqrt(&self) -> Self {
        isqrt(*self)
    }
    
    fn nth_root(&self, n: usize) -> Self{
        if n > 63 {
            return 1;
        }

        if n == 1 {
            return *self;
        }

        if n == 0 {
            panic!("No integer is a zeroth factor ")
        }

        let mut est = (*self as f64).powf((n as f64).recip()) as Self + 1;

        loop {
            let s = est;
            let t = (n as u64 - 1) * s + *self / s.pow(n as u32 - 1);
            est = t / (n as u64);
            if est >= s {
                return s;
            }
        }
    }
    
    fn max_exp(&self) -> (Self,Self){
    
      for i in 1..64{
      let p = 64-i;
      let base = self.nth_root(p);
         if base.pow(p as u32) == *self{
           return(base,p as u64)
         }
      }
     return (*self,1)
    }    

    fn is_square(&self) -> bool {
        let sq = isqrt(*self);
        sq * sq == *self
    }
}
