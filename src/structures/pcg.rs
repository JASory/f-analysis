use std::sync::Arc;
use crate::filter::filtertype::{Prime,Composite};

pub trait PrimeValuation{
  const BOOLEAN : bool;
}

impl PrimeValuation for Prime{
   const BOOLEAN : bool = true;
}


impl PrimeValuation for Composite{
   const BOOLEAN : bool = false;
}

fn identity(x: &u64) -> bool{
   true
} 

// Generates either primes or odd composites
pub struct PCGenerator<T: PrimeValuation>{
  sup: u64,
  c_flag: std::marker::PhantomData<T>, 
}

impl<T: PrimeValuation> PCGenerator<T>{

pub fn init(sup: u64) -> Self{
    Self{sup: sup, c_flag: std::marker::PhantomData::<T>::default() }
}
 
pub fn filter(&self,func : &(dyn Fn(&u64) -> bool + 'static) ) -> Vec<u64>{
    let isqrt = (self.sup as f64).sqrt() as u64;
   
   let segment_size = std::cmp::max(isqrt,32768);
   let mut count = 0u64;
   let mut values = vec![];
   let mut sieve = Vec::<bool>::with_capacity(segment_size as usize);
   let mut is_prime = vec![true;isqrt as usize+1];
   let mut primes = vec![];
   let mut multiples = vec![];
   let mut low = 0u64;
   
   let mut i : u64 = 3;
   let mut n : u64 = 3;
   let mut s : u64 = 3; 
   
   loop { // full loop
     
    
     if low > self.sup{
       break;
     }
     
     
     sieve.truncate(0);
     sieve.resize(segment_size as usize,true); // allocate with true values
     
     let mut high : u64 = low + segment_size -1;
     high = std::cmp::min(high,self.sup);
     
     let inner_sup = (high as f64).sqrt() as u64;
    
     loop{ // Generate sieving primes
     
       if i*i > high{
          break;
       }
       
       if is_prime[i as usize]{
                      let mut j = i*i;
         loop {

            if j > isqrt{
               break;
            }
            is_prime[j as usize] = false;
                        j+=i;
         }
         
       }
              i+=2;
     } // End prime generation
    
     loop{// prime initialisation
       if s*s > high{
         break;
       }
       
       if is_prime[s as usize]{
          primes.push(s);
          multiples.push(s*s -low);
       }
              s+=2;
     }// end prime initialisation
   
     for i in 0..primes.len(){// sieve current segment
     
         let mut j = multiples[i as usize];
         
         let k = primes[i as usize]*2;
         
         loop {
           if j >= segment_size{
             break;
           }
           sieve[j as usize] = false;
           j+=k;
         }
         multiples[i as usize] = j -segment_size;
     }// end current sieve

     loop{
       if n > high{
         break;
       }
       if sieve[(n - low) as usize]==T::BOOLEAN{
         count+=1;
         if func(&n){
         values.push(n);
         }
       }
       n+=2;
     }
          low+=segment_size;
   }
   values
}
 
 pub fn to_vector(&self) -> Vec<u64>{
       self.filter(&identity)
 }
 
 
}
