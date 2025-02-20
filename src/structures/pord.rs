use crate::Primes;
use crate::structures::store::{Storage};
use crate::Natural;
use crate::natural::montcore::NTCore;
use crate::CompVector;


// Set of primes with a specific order
pub struct OrdPrime{
    ord: u64,
    primes: Vec<u64>,
}

pub struct OrdPrimes{
  elements: Vec<OrdPrime>,
}

/// orders of 32-bit primes 
pub struct PrimeOrd{
    a: u64,    // (ord,p) 32-bit 
	elements : Vec<u64>,
}

impl PrimeOrd{
	
	pub fn new(a: u64,p_bound: u64) -> Self{
		
	       	let plist = Primes::generate_or_restore(p_bound as usize);
			let mut elements = vec![];
			
			for i in plist.iter(){
			  if i.gcd(a) == 1{
				elements.push(i+(i.p_ord(a)<<32));
				}
			  else{
			     elements.push(i);
			  }	
			}
		   Self{a,elements}			
	}
	}
	/*
 impl Storage for PrimeOrd{
 
    fn to_persistent(&self, locale: &str) -> FResult<()>{
    
    }
    
    fn from_persistent(locale:&str) -> FResult<Self>{
    
    }
    
 }	
 */
 /*
 fn calculate_large_primes() -> OrdPrimes{
    // 
    let bound = 
    // 
 }
 
 fn calculate_small_primes() -> PrimeOrd{
 
 }
 */
