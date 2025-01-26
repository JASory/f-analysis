use crate::fermat::{FInteger,NTCore};
use crate::primes::{PRIME_INV_128,SMALL_PRIMES};
//use crate::math::mont_core::lucas;
use machine_prime::{PRIME_INV_64,is_prime_128};
use crate::Pseudoprime;


/*
    128-bit FInteger check
*/

impl FInteger for u128{

    const ONE: u128 = 1;
    const ZERO: u128 = 0;
    const BYTE_LENGTH : usize = 16;
    
    
    fn is_even(&self) -> bool{
       *self&1 == 0
    }
    
     fn is_multiple_of(&self, factor: u64) -> bool{
        *self%(factor as u128) == 0
     }
  
    fn inc_by(&mut self, inc: u64){
       *self += inc as u128;
    }
    
    fn from_u64(x: u64) -> Self {
        x as u128
    }
	
	fn to_u64(&self) -> u64{
	  if *self > 1u128<<64{
		panic!("to_u64 function panicked due to value being greater than 2^64")
		}
		*self as u64
	}

    fn is_bounded_by(&self, inf: Self, sup: Self) -> bool {
        if *self > inf && *self < sup {
            return true;
        }
        false
    }
	
	fn min_max(&self, otra: Self) -> (Self,Self){
		if otra < *self{
			return (otra,*self)
		}
		(*self,otra)
		
	}
	
	
	fn wrapping_sub(&self, otra: Self) -> Self{
	   u128::wrapping_sub(*self,otra)
	}

    fn byte_length() -> usize {
        16usize
    }

    fn msb(&self) -> usize{
        128usize-self.leading_zeros() as usize
    }
    
    fn hash_shift(&self, shift: usize, multiplier: u32) -> usize {
        ((*self as u32).wrapping_mul(multiplier) >> shift) as usize
    }

    fn is_semiprime_k(&self, a: usize) -> bool {
        let fctr = a as u128;
        let sq = (*self - 1) / fctr;
        let k = sq.isqrt();

        if ((k * k + k) * fctr + k + 1) == *self {
            return true;
        }

        return false;
    }
	
	
	fn even_complement(&self, k: Self) -> Self{
		((*self-1)/2)*k + 1
	}
	
	fn semi_k_complement(&self, k: usize) -> Self{
       (k as u128)*(*self -1)+1
    }
	
	fn overflowing_mul(&self,otra: Self) -> (Self,bool){
		u128::overflowing_mul(*self,otra)
		
	}

    fn trial_bound(&self, s: usize) -> bool {
	      unimplemented!()  
	}
	
	
    fn small_factor(&self) -> Vec<u64>{
       let mut veccy = vec![];
       
       for i in SMALL_PRIMES.iter(){
          if *self% (*i as u128) == 0{
            veccy.push(*i);
          }
       }
       veccy
    }
    
    fn div_vector(&self, f: &[u64]) -> bool{
        for i in f.iter(){
          if *self% (*i as u128) == 0{
            return true
          }
        }
        return false
    }
	 
	
	fn euclidean(&self, otra: Self) -> (Self,Self){
	   (*self/otra,*self%otra)
	} 
	   
	fn from_bytes(x: &[u8]) -> Self {
        Self::from_le_bytes(
		[x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7],
		x[8], x[9], x[10], x[11], x[12], x[13], x[14], x[15]])
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_str(x: &str) -> Option<Self> {
        match x.parse::<u128>() {
            Ok(z) => Some(z),
            Err(_) => None,
        }
    }

    fn successor(&mut self) {
        *self += 1;
    }
   
    
    fn comp_gen_k(k: usize) -> Option<Self> {
       if k > 128{
           return None;
        }
        loop{
          let p = Self::gen_k(k).unwrap();
          if !p.is_prime(){
             return Some(p);
          }
        }
		None
    }

    fn prime_gen_k(k: usize) -> Option<Self> {
        if k > 128{
           return None;
        }
        loop{
          let p = Self::gen_k(k).unwrap();
          if p.is_prime(){
             return Some(p);
          }
        }
		None
    }
    
    fn gen_k(k: usize) -> Option<Self>{
      
      if k < 65{
		return u64::gen_k(k).map(|x| x as u128);
		}
		let lhs = u64::gen_k(64).unwrap();
		let rhs = u64::gen_k(64).unwrap();
	    let hi_digit = 1u128<<k;
	    let mask = (hi_digit-1)|hi_digit;
	    let res = (((lhs as u128)<<64) + (rhs as u128))&mask;
	    Some(res)
    }

    fn gcd(&self, other: Self) -> Self {
       let mut a = *self;
       let mut b = other;
       
       if b == 0 {
          return a;
      } else if a == 0 {
        return b;
      }

      let self_two_factor = a.trailing_zeros();
      let other_two_factor = b.trailing_zeros();
      let min_two_factor = std::cmp::min(self_two_factor, other_two_factor);
      a >>= self_two_factor;
      b >>= other_two_factor;
     loop {
        if b > a {
            std::mem::swap(&mut b, &mut a);
        }
        a -= b;

        if a == 0 {
            return b << min_two_factor;
        }
        a >>= a.trailing_zeros();
     }
    }

   fn lcm(&self, otra: Self) -> Option<Self>{
       let g = self.gcd(otra);
       let (res,flag) = self.overflowing_mul(otra/g);
       if flag{
         return None
       }
       Some(res)
    }
    
   // FIXME
   fn product_residue(&self, other: &Self, n: &Self) -> Self {
        if n == &0 {
            return self.wrapping_mul(*other)
        }
        ((*self as u128 * *other as u128) % *n as u128) as Self
    }
    
    fn extended_gcd(&self, other: Self)->(Self,Self,Self){
        let mut gcd: u128 = *self;
        let mut new_r: u128 = other;
        let mut bezout_1: u128 = 1;
        let mut new_s: u128 = 0;
        let mut bezout_2: u128 = 0;
        let mut new_t: u128 = 1;

        while new_r != 0 {
            let quotient = gcd / new_r;
            let mut temp: u128 = new_r;
            new_r = gcd - quotient * temp;
            gcd = temp;

            temp = new_s;
            if bezout_1 < quotient.product_residue(&temp, &other) {
                // First bezout coefficient is computed over Z[q]
                new_s = other - (quotient.product_residue(&temp, &other) - bezout_1)
            } else {
                new_s = bezout_1.wrapping_sub(quotient * temp);
            }

            bezout_1 = temp;

            temp = new_t;
            if bezout_2 < quotient.product_residue(&temp, self) {
                // Second bezout coefficient is computed over Z[p]
                new_t = *self - (quotient.product_residue(&temp, self) - bezout_2)
            } else {
                new_t = bezout_2.wrapping_sub(quotient.product_residue(&temp, self));
            }
            bezout_2 = temp
        }
        (gcd, bezout_1, bezout_2)
    }
    

    fn jacobi(&self, other: Self) -> i8 {
    let mut n = *self;
    let mut p = other;
    let mut t = 1i8;
    n %= p;

    while n != 0 {
        let zeros = n.trailing_zeros();
        n >>= zeros;

        if (p % 8 == 3 || p % 8 == 5) && (zeros % 2 == 1) {
            t = -t
        }

        std::mem::swap(&mut n, &mut p);
        if n % 4 == 3 && p % 4 == 3 {
            t = -t;
        }
        n %= p;
    }

    if p == 1 {
        t
    } else {
        0
    }
    }
    

    fn classify(&self, a: Self) -> Pseudoprime{
       	      unimplemented!()
    }
    
    fn fermat(&self, a: Self) -> bool {
        NTCore::fermat(self, a)
    }
    
    // Odd-only semifermat
    fn semi_fermat(&self, x: Self, y: Self) -> bool{
       unimplemented!()
    }
	
	fn sqr_fermat(&self, base: Self) -> bool{
		if *self > 1u128<<32{
			return (*self as u128).p_sq_fermat(base as u128);
		}
		else{
			return self.p_sq_fermat(base);
		}
		
	}
    
    fn exp_residue(&self, p: Self, n: Self) -> Self{
       NTCore::expr(self,p,n)
    }
    
    fn euler_jacobi(&self, a: Self) -> bool{
       let r = a.jacobi(*self);
       if r == -1{
        return a.exp_residue((*self-1)/2,*self) == *self-1 
       }
       a.exp_residue((*self-1)/2,*self)== (Self::from_u64(r as u64))
    }

    fn sprp(&self, a: Self) -> bool {
     if *self&1==0{
       return NTCore::fermat(self,a);
     }
        NTCore::sprp(self,a)
    }

    fn is_prime(&self) -> bool {
       machine_prime::is_prime_128(*self)      
    }
    
    fn euler_p(&self) -> bool{
       let residue = *self&7;
       let mut param = 0;
       
       if residue == 1{ 
          param = 1;
       }
       
       let ap = 2.exp_residue((*self-1)>>(1+param),*self);
       if ap == 1  {
           return residue == 1 || residue == 7;
       }
       else if ap == *self-1 {
           return residue == 1 || residue == 3 || residue == 5
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
       if *self == x as u128 {
         return true
       }
       if x == 1 || x == 0{
          return false
       }
       if *self% (x as u128) != 0{
         return false
       }
       let mut val = x as u128;
       
       for _ in 0..128{
         let (val_interim,flag) = val.overflowing_mul(x as u128);
         
         val = val_interim;
         
         if flag{ // if overflowed then not perfect power
           return false
         }
         if val == *self as u128{
            return true
         }
       }
       return false
    }

    fn isqrt(&self) -> Self {
            let mut est = (*self as f64).sqrt() as u128 + 1;

    loop {
        let s = est;
        let t = s + *self / s;
        est = t >> 1;
        if est >= s {
            return s;
        }
    }
    
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
            let t = (n as u128 - 1) * s + *self / s.pow(n as u32 - 1);
            est = t / (n as u128);
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
           return(base,p as u128)
         }
      }
     return (*self,1u128)
    }    

    fn is_square(&self) -> bool {
        let sq = self.isqrt();
        sq * sq == *self
    }

}
