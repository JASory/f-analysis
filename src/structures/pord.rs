use crate::Primes;
use crate::fermat::FInteger;
use crate::fermat::NTCore;



pub struct PrimeOrd{
	elements : Vec<(u64,u64)>,
}

impl PrimeOrd{
	
	pub fn new(base: u64, ord_bound: u64, p_bound: u64) -> Self{
		
	       	let plist = Primes::generate_or_restore(p_bound as usize);
			let mut elements = vec![];
			
			for i in plist.iter(){
			  if i.gcd(base) == 1{
				elements.push((i,base.bounded_ord(i,ord_bound)));
				}
			}
			
		   Self{elements}			
	}
	
	/*
	fn sp_eval(&self) -> Vec<u64>{
		
		for (idx,el) in elements.iter().enumerate(){
			for j in elements[i..].iter(){
				idx
			}
		}
		
	}
	*/
	}
