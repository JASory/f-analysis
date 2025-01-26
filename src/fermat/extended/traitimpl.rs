use crate::fermat::FInteger;
use crate::Pseudoprime;


 /// Extended Precision Integer (Zahl)
#[derive(Clone,Copy,Debug,Hash,PartialEq,PartialOrd,Eq,Ord)]
pub struct Epz<const S : usize>{
	pub limbs: [u64;S],
}


const fn zero<const S : usize>() -> Epz<S>{
		Epz{limbs : [0u64;S]}
	}

const fn one<const S : usize>() -> Epz<S>{
	let mut k = zero();
		k.limbs[0] = 1;
		k
}

const fn byte_length<const S : usize>() ->usize{
	S*8usize
}
 

impl<const S : usize> Default for Epz<S>{
	fn default() -> Self{
		Epz{limbs : [0u64;S]}
	}
}

impl<const S :  usize> std::fmt::Display for Epz<S>{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		unimplemented!()
	}
}

impl<const S : usize> FInteger for Epz<S>{
	/// Additive Identity
	const ZERO: Self = zero::<S>();
    /// Multiplicative Identity; 1
    const ONE: Self = one::<S>();
    /// Number of bytes used in representation
    const BYTE_LENGTH : usize = byte_length::<S>();
	
    /// 
    fn product_residue(&self, other: &Self, n: &Self) -> Self{
		unimplemented!()
	}
	
	
	fn wrapping_sub(&self, otra: Self) -> Self{
	   *self-otra
	}
	
    fn is_even(&self) -> bool{
       self.limbs[0]&1==0
    }
    
    fn is_multiple_of(&self, factor: u64) -> bool{
        unimplemented!()
     }
    
    fn small_factor(&self) -> Vec<u64>{
       unimplemented!()
    }
    
    fn div_vector(&self, div: &[u64]) -> bool{
      unimplemented!()
    }
    
    fn inc_by(&mut self, inc: u64){
      unimplemented!()
    }
    
    /// Evaluates if integer is between inf and sup exclusive
    fn is_bounded_by(&self, inf: Self, sup: Self) -> bool{
		unimplemented!()
	}
	
	/// Returns the Minimum and Maximum of a 2-tuple
	fn min_max(&self,otra: Self) -> (Self,Self){
		unimplemented!()
	}
	
    /// (x*multiplier / 2^shift) mod 2^32
    fn hash_shift(&self, shift: usize, multiplier: u32) -> usize{
		((self.limbs[0] as u32).wrapping_mul(multiplier) as usize)>>shift
	}
	
    /// Classifies the integer into the result of 
    fn classify(&self, a: Self) -> Pseudoprime{
		unimplemented!()
	}

    fn is_semiprime_k(&self, a: usize) -> bool{
		unimplemented!()
	}
	
	fn semi_k_complement(&self, k: usize) -> Self{
	    unimplemented!()
	}
    
    // 4x+1 component to the number of the form (x+1)(4x+1)
     fn even_complement(&self, k: Self) -> Self{
		unimplemented!()
	}
	
    fn overflowing_mul(&self, otra: Self) -> (Self,bool){
		unimplemented!()
	}
    
    /// Evaluates if integer is coprime to all primes under the provided bound (exclusive)
    fn trial_bound(&self, s: usize) -> bool{
		unimplemented!()
	}
	
	
	fn euclidean(&self, otra: Self) -> (Self,Self){
	  unimplemented!();// (*self/otra,*self%otra)
	}

    fn to_u64(&self) -> u64{
		unimplemented!()
	}
	
    fn from_u64(k: u64) -> Self{
		let mut z = Self::ZERO;
		z.limbs[0]=k;
		z
	}
    
    /// Number of bytes used in representation
    fn byte_length() -> usize{
	   Self::BYTE_LENGTH
	}
	
	fn msb(&self) -> usize{
	   match self.limbs.iter().rposition(|x| *x !=0){
	     Some(pos) => {
	       return pos*64+(64-self.limbs[pos].leading_zeros() as usize);
	     },
	     None => return 0usize,
	   }
	}
    
    /// From Little-Endian bytes
    fn from_bytes(x: &[u8]) -> Self{
		unimplemented!()
	}
	
    /// To Little-Endian bytes
    fn to_bytes(&self) -> Vec<u8>{
		unimplemented!()
	}
	
    /// Initialise from string
    fn from_str(x: &str) -> Option<Self>{
		unimplemented!()
	}
	
    /// Randomly generate guaranteed composite of k-bit length
    fn comp_gen_k(k: usize) -> Option<Self>{
		unimplemented!()
	}
	
    /// Randomly generate guaranteed prime of k-bit length
    fn prime_gen_k(k: usize) -> Option<Self>{
		unimplemented!()
	}
	
    /// Randomly generate integer of k-bit length
    fn gen_k(k: usize) -> Option<Self>{
		unimplemented!()
	}
    
    /// Successor function; X+1 
    fn successor(&mut self){
		unimplemented!()
	}
    
    ///  Greatest common divisor
    fn gcd(&self, other: Self) -> Self{
		unimplemented!()
	}
    
    fn lcm(&self, otra: Self) -> Option<Self>{
       unimplemented!()
    }
    
    ///  Finite ring gcd
    fn extended_gcd(&self, ring: Self) -> (Self, Self,Self){
		unimplemented!()
	}
    
    /// x^p mod n 
    fn exp_residue(&self, p: Self, n: Self) -> Self{
		unimplemented!()
	}
    
    /// 
    fn semi_fermat(&self,p: Self, q: Self) -> bool{
		unimplemented!()
	}
    
    // Strong-fermat to a semiprime with provided factors p,q
    // fn semi_sprp(&self, p: Self, q: Self) -> bool;
    
    fn sqr_fermat(&self,p: Self) -> bool{
		unimplemented!()
	}
	
    /// Jacobi symbol
    fn jacobi(&self, other: Self) -> i8{
		unimplemented!()
	}
	
    /// a^p-1 mod p = 1
    fn fermat(&self, a: Self) -> bool{
		unimplemented!()
	}
    /// a^p-1 mod p = jacobi(a,p)
    fn euler_jacobi(&self, a: Self) -> bool{
		unimplemented!()
	}
	
    /// Colin Plumb's variant of Fermat test
    fn euler_p(&self) -> bool{
		unimplemented!()
	}
	
    /// Strong Fermat
    fn sprp(&self, a: Self) -> bool{
		unimplemented!()
	}
	
    /// Deterministic primality, using a combination of machine-prime (J.A Sory's parameters), 
    /// Sorensen and Webster's parameters and Miller test assuming GRH
    /// This function is used to generate guaranteed composites or primes
    fn is_prime(&self) -> bool{
		unimplemented!()
	}

    fn is_perfect_power(&self) -> bool{
		unimplemented!()
	}
    
    fn is_power_of(&self, x: usize) -> bool{
		unimplemented!()
	}
    /// Integer sqrt
    fn isqrt(&self) -> Self{
		unimplemented!()
	}
    /// Integer nth root
    fn nth_root(&self, n: usize) -> Self{
		unimplemented!()
	}
    
    fn max_exp(&self) -> (Self,Self){
		unimplemented!()
	}

    fn is_square(&self) -> bool{
		unimplemented!()
	}
}


/*
  Algorithm 
  
  Check if Length will fit in binary array of length S
  check if String is all ASCII digits
  
  Map string to radix-10^19 vector
  
  Convert to radix-2^64, fill the resultant array
  
  
*/

/*
impl std::str::FromStr for EPZ<S>{
     type Err = &str;
     
 fn from_str(input: &str) -> Result<Self,Self::Err>{
    let  minimumlen = (19.265919722*(S as f64)).floor() as usize;
    let inputlen = input.len()/19;
    // Check 
    if input.len()*19 > minimumlen{
     return Err("String too large")
    }
    
    let b = input.bytes().all(|c| c.is_ascii_digit());

    if !b {
        return Err("Non-digit characters in string");
    }
    
    let mut digit_vector = Vec::with_capacity()
    for i in 
    
 }
}
*/
