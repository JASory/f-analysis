use crate::structures::Primes;
use crate::Natural;
use crate::structures::store::Storage;
use crate::{HashTable,CompVector};
use crate::car::MRC_18;
use crate::FResult;
use std::fs::File;
use std::io::{Write,Read};
use crate::enums::{Search,MEMORY_MAX,UTF8_FLAG,AUTO_FLAG};


/// Vector of Fermat bases to be evaluated as a full primality test
#[derive(Clone,Debug)]
pub struct BaseSeq<T: Natural>{
   bases: Vec<T>,
   mode : Search,
}

#[macro_export]
 macro_rules! bseq{
    () => {
       BaseSeq::new(vec![])
    };
   ( $( $x:expr ),* ) => {
      {
        let mut tmpvec = BaseSeq::new(vec![]);
        $(
          tmpvec.append($x);
        )*
        tmpvec
      }
   };
   
   }

impl<T: Natural> Storage for BaseSeq<T>{
    
     fn to_persistent(&self, locale: &str) -> FResult<()>{
        use std::fs::File;
        use std::io::Write;

        match File::create(locale) {
            Ok(mut out) => {
                let res = self.to_string();
                match out.write_all(res.as_bytes()) {
                    Ok(_) => FResult::Success,
                    Err(message) => FResult::IOError(message),
                }
            }
            Err(message) => FResult::IOError(message),
        }
    }
    
    fn from_persistent(filename: &str) -> FResult<Self>{
    
       match std::fs::File::open(filename){
         Ok(mut x) => {
            let mut buffer = String::new();
            match x.read_to_string(&mut buffer){
              Ok(_) => { 
                 // FIXME Handle unwrap correctly   
                let res = buffer.split(",")
                          .map(|z| T::from_str(z).unwrap() )
                          .collect::<Vec<T>>();
                          
                FResult::Value(Self::new(res))
            }
            Err(read_error) => FResult::IOError(read_error),
         }
       }
        Err(file_error) => FResult::IOError(file_error),
     } 
    }
}

impl<T: Natural> BaseSeq<T>{
	   
	
   pub fn new(bases : Vec<T>) -> Self{
      Self {bases, mode : Search::WeakHeuristic}	   
   }
   
   pub fn append(&mut self, el: T) {
       self.bases.push(el)
   }
   
   pub fn set_weak_heuristic(&mut self){
      self.mode=Search::WeakHeuristic;
   }
   
   pub fn len(&self) -> usize{
      self.bases.len()
   }
   
   pub fn set_strong_heuristic(&mut self){
	   self.mode = Search::StrongHeuristic;
   }
   
   pub fn set_deterministic(&mut self){
	  self.mode = Search::Deterministic;
   }
   
   pub fn iter(&self) -> std::slice::Iter<T>{
      self.bases.iter()
   }
   
   
   
   pub fn swap(&mut self, new_value: T, idx: usize) -> Option<T>{
      if idx >= self.len(){
         return None
      }
      let interim = self.bases[idx];
      self.bases[idx] = new_value;
      Some(interim)
   }
   
   pub fn rand_initialise(len: usize) -> Self{
      let mut veccy = vec![];
       for i in 0..len{
         veccy.push(T::gen_k(T::BYTE_LENGTH*8).unwrap())
       }
      Self{bases: veccy, mode: Search::WeakHeuristic} 
   } 
   
   pub fn primality(&self, c: T) -> bool{
       for i in self.bases.iter(){
          if !c.sprp(*i){
            return false
          }
       }
       return true
   }
   
   pub fn generate_pseudoprimes(&self, inf: T, sup: T, locale: Option<&str>) -> FResult<CompVector<T>>{
 
 	if self.mode == Search::Deterministic{
		   return FResult::NotSupported;
		}
		
		let p_bound = sup.isqrt().to_u64() as usize;
	   	let plist = Primes::generate_or_restore(p_bound);
	   	
	   	match locale {
	   	   // Write all composites to file
	   	   Some(x) => {
	   	       let mut outfile = File::create(x).unwrap();
	   	       let mut out = std::io::BufWriter::new(outfile.try_clone().unwrap());
	   	       // Monier-Rabin Heuristic
	   	       for i in plist.iter(){
	   	       
	   	          let lhs = T::from(i);
	   	          
	   	          for j in [3,4,6].iter(){
                  let rhs = lhs.even_complement(T::from(*j));
       
                  if rhs.is_prime(){
                     let (prod,flag) = lhs.overflowing_mul(rhs);
                     // If multiplication overflowed or the prod exceeds the bound break loop
                     if flag  || !prod.is_bounded_by(T::ZERO,sup){
                       break;
                     }
                     
                     if prod.is_bounded_by(inf,sup) && self.primality(prod){
                        out.write(&prod.to_bytes()[..]).unwrap();
                     }
                  }
                }
            }
           
               for i in MRC_18{
                 let carmichael = T::from(i);
                 if carmichael.is_bounded_by(inf,sup) && self.primality(carmichael){
                  out.write(&carmichael.to_bytes()[..]).unwrap();
                 }
              }
                out.flush().unwrap();
              
              if self.mode == Search::StrongHeuristic{
              
                for i in plist.iter(){
                
                   let lhs = T::from(i);
                   
	   	           for j in 2..64{
	   	           
                     let rhs = lhs.semi_k_complement(j);
       
                      if rhs.is_prime(){
                         let (prod,flag) = lhs.overflowing_mul(rhs);
                         
                          if flag || !prod.is_bounded_by(T::ZERO,sup){
                             break;
                          }
                          
                         if prod.is_bounded_by(inf,sup) && self.primality(prod){
                        
                           out.write(&prod.to_bytes()[..]).unwrap();
                         }
                       }
                  }
                }
                out.flush().unwrap();
              }
              return FResult::Value(CompVector::from_file_internal(outfile.try_clone().unwrap(),MEMORY_MAX,UTF8_FLAG,AUTO_FLAG));
    
	   	   } // End file write
	   	   
	   	   // Write all composites to volatile memory (i.e vector)
	   	   None => {
	   	      let mut ce = Vec::<T>::new();
	   	      
	   	      for i in plist.iter(){
	   	      
	   	          let lhs = T::from(i);
	   	        for j in [3,4,6].iter(){
                  let rhs = lhs.even_complement(T::from(*j));
       
                  if rhs.is_prime(){
                     let (prod,flag) = lhs.overflowing_mul(rhs);
                     
                      if flag || !prod.is_bounded_by(T::ZERO,sup){
                       break;
                      }
                     
                     if prod.is_bounded_by(inf,sup)&self.primality(prod){
                        ce.push(prod);
                     }
                  }
                }
              }
              
               for i in MRC_18{
                 if T::from(i).is_bounded_by(inf,sup)&self.primality(T::from(i)){
                   ce.push(T::from(i))
                 }
              }
              
               if self.mode == Search::StrongHeuristic{
                
                for i in plist.iter(){
                
                   let lhs = T::from(i);
                   
	   	           for j in 2..64{
	   	           
                     let rhs = lhs.semi_k_complement(j);
       
                      if rhs.is_prime(){
                         let (prod,flag) = lhs.overflowing_mul(rhs);
                         
                          if flag || !prod.is_bounded_by(T::ZERO,sup){
                             break;
                          }
                         
                         if prod.is_bounded_by(inf,sup) && !flag && self.primality(prod){
                           ce.push(prod);
                         }
                       }
                  }
                }
              }
              return FResult::Value(CompVector::from_vector(ce));
	   	   }
	} // end match
   }
   
   }
   
   impl BaseSeq<u64>{
	   
	   /// Returns the first pseudoprime  to the bases, as generated by the Search 
	   pub fn upper_bound(&self) -> FResult<u128>{
	       // default bound, this covers all 64-bit composites
		   let mut bound = 1usize<<32;
		   
		   // Modifies bound to be number of bases 
		   if self.bases.len() > 6{
			   bound = 1usize<<(32 + (self.bases.len()-6));
		   }
		    // restore or generate list of primes
		   let plist = Primes::generate_or_restore(bound);
		   // Counterexample
		   let mut ce : u128 = u128::MAX;
		   let mut p_stop = u64::MAX;
		   
    // (2x+1)(4x+1) where 2x+1 and 4x+1 are semiprimes 
            for i in plist.iter(){
               let lhs = i;
               let rhs = i.even_complement(4);
       
             if rhs.is_prime(){
               let (prod,flag) = lhs.overflowing_mul(rhs);
              // If product is greater than 2^64 promote to 128-bit arithmetic
             if flag{
                 let prod_128 = lhs as u128 * rhs as u128;
                 let mut inner_flag = false;
           
			   for j in self.bases.iter(){
                 if  prod_128.sprp(u128::from(*j)) == false{
                      inner_flag = true;
				     break;
                 }
                }
                // Set ce to the found product
              if inner_flag == false{
                  ce = prod_128;
                  p_stop = lhs;
                  break;
              }
            }
            else {
               let mut inner_flag = false;
               
               for j in self.bases.iter(){
               
                if  prod.sprp(*j) == false{
                     inner_flag = true;
				   break;
                }
               }
               // Set ce to the found product
           if inner_flag == false{
             ce = prod as u128;
             p_stop = lhs;
             break;
           }
           
          }
       }
    }
    
    if 	self.mode == Search::StrongHeuristic || self.mode == Search::Deterministic{
        
        for i in plist.iter(){
               let lhs = i;
               
               if lhs > p_stop{
                  break;
               }
            
               for k in 2..256{
               
               let rhs = i.semi_k_complement(k);
       
             if rhs.is_prime(){
               let (secprod,flag) = lhs.overflowing_mul(rhs);
               
              
              // If product is greater than 2^64 promote to 128-bit arithmetic
             if flag{
                 let secprod_128 = lhs as u128 * rhs as u128;
                 let mut secinner_flag = false;
           
			   for j in self.bases.iter(){
                 if  secprod_128.sprp(u128::from(*j)) == false{
                      secinner_flag = true;
				     break;
                 }
                }
                // Set ce to the found product
              if secinner_flag == false && secprod_128 < ce {
                  ce = secprod_128;
                  break;
              }
            }
            else {
               let mut secinner_flag = false;
               
               for j in self.bases.iter(){
               
                if  secprod.sprp(*j) == false{
                     secinner_flag = true;
				   break;
                }
               }
               // Set ce to the found product
             if secinner_flag == false && (secprod as u128) < ce{
                ce = (secprod as u128);
                break;
             }
           }
       } // conditional prime check
     }  // end k loop
    }   // primeloop  
   }   // end branch
    if self.mode == Search::Deterministic{
       return FResult::NotSupported;
    }
    if ce == u128::MAX{
       return FResult::NoCandidate;
    }
    // 
    return FResult::Value(ce)
	}
	
	// If Strong heuristic 
	// If Deterministic block


   // generate_pseudoprimes(&self,inf: T, sup: T ) -> CompVector<T>	
   }
   
   
   impl<T: Natural> std::fmt::Display for BaseSeq<T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       let  zepto = self.bases.iter().map(|x| x.to_string()).collect::<Vec<String>>();
       let quokka = zepto.join(",");
       write!(f,"{}",quokka)
    }
    
   } 
