// Search functions
use crate::structures::{Primes,Point,BaseVector,CompVector};
use crate::fermat::{FInteger,IntSeq,FIterator};

use crate::{HashTable};
use crate::filter::{StrongFermat,Coprime,GenericFilter};
use std::io::{Read,BufRead,Write};
use crate::compeval::{vector::*,file::*};
use crate::search::{hash_search,unary_ht_par,strip_pseudo_par,strip_pseudo_st,binary_evo_st,binary_det_iter_st,binary_evo_par,binary_evo_st_rand_partial, unary_strongest_st,unary_strongest_par,unary_strongest_rand_par,exhaustive_par,exhaustive_rand_par};
use crate::io::write::format_block;
use crate::result::FResult;
use crate::compconfig::{Search,AUTO_FLAG,UTF8_FLAG,MEMORY_MAX};

impl<T: FInteger> CompVector<T>{

  
//
  pub(crate) fn filter_generic_internal<F : GenericFilter>(&self, fileout: Option<&str>, filter_flag: bool) -> FResult<Self>{
  
    let mut pseudos : Vec<T> = Vec::new();
    
    match fileout{
     Some(x) =>{
        match std::fs::File::create_new(x){
          Ok(mut output) => {
             if self.elements.len() > 0{
              let pseudos = filter_generic_v::<T,F>(&self.elements);
              // FIXME catch write error
             for i in pseudos{
                 output.write(&i.to_bytes()[..]).unwrap();
             }
           }
           
          else{
             filter_generic_file::<T,F>(self.file.as_ref().unwrap().try_clone().unwrap(),output.try_clone().unwrap(),self.utf8_flag,true);
          }
          return FResult::Value(Self::from_file_internal(output.try_clone().unwrap(),self.memory_max,self.utf8_flag,self.auto_flag))
          }
          
          Err(file_error) => FResult::IOError(file_error),
        }
     }
     None => {
             let mut pseudos : Vec<T> = Vec::new();
             if self.elements.len() > 0{
                 pseudos = filter_generic_v::<T,F>(&self.elements);
             }
             else{
                let infile = self.file.as_ref().unwrap().try_clone().unwrap();
                pseudos = filter_generic::<T,F>(infile,self.utf8_flag,true);
             }
           return FResult::Value(Self::from_vector_internal(pseudos,self.memory_max,self.utf8_flag,self.auto_flag));
          }
  }
 }
 
  /// Applies a filter that implements the GenericFilter trait
  pub fn filter_generic<F: GenericFilter>(&self, filename: Option<&str>) -> FResult<Self> {
      self.filter_generic_internal::<F>(filename,true)
  }
  
  pub fn nfilter_generic<F: GenericFilter>(&self, filename: Option<&str>) -> FResult<Self> {
      self.filter_generic_internal::<F>(filename,false)
  }
  
  /// Filters all composites that fail a compile-time defined check that implements GenericFilter and StrongFermat
  pub fn filter_sprp<F: StrongFermat>(&self, filename: Option<&str>) -> FResult<Self> {
      self.filter_generic_internal::<F>(filename,true)
  }
  
  pub fn nfilter_sprp<F: StrongFermat>(&self, filename: Option<&str>) -> FResult<Self> {
      self.filter_generic_internal::<F>(filename,false)
  }
  
  pub fn filter_coprime<F: Coprime>(&self, filename: Option<&str>) -> FResult<Self> {
      self.filter_generic_internal::<F>(filename,true)
  }
  
  pub fn nfilter_coprime<F: Coprime>(&self, filename: Option<&str>) -> FResult<Self> {
      self.filter_generic_internal::<F>(filename,false)
  }
  
  /// Attempts to construct a hashtable of fermat bases with the provided arguments (size, hash multiplier, and fermat base maximum) or use defaults.  
    /// Note that providing the same integer parameters for the same set results in identical tables being produced, 
    /// allowing reproducibility. 
    /// Variation is primarily determined by the multiplier value which is pseudorandomly generated if not provided. 
    /// For instance to_hashtable(None,Some(3411698987), None) will always produce the same table for the same composite set
    /// as the dimension is computed as a ratio of the length and the base maximum is 65535 by default
    pub fn to_hashtable(&self, dimen: Option<usize>, multiplier: Option<u32>,bound: Option<u64>) -> FResult<HashTable> {
        // If dimension of hashtable defined use it, otherwise calculate it 
        let dim = if let Some(dm) = dimen {
              dm
            } else {
            (self.elements.len()/150).next_power_of_two()
        };
       
        // If multiplier defined use it, otherwise calculate it
        let mul = if let Some(mx) = multiplier {
              mx
            } else {
            let iterations = 262144000/self.elements.len();
            hash_search(&self.elements[..], dim, iterations)
        };
        
        // If multiplier defined use it, otherwise set it as 65535
        let bnd = if let Some(bd) = bound {
              bd
            } else {
            65535
        };
        
        match unary_ht_par::<T,1>(self.elements.clone(), dim, mul, bnd) {
            FResult::Exhaustive(x) => FResult::Exhaustive(HashTable::new(x, dim, mul)),
            FResult::InsufficientCandidates(x) => FResult::InsufficientCandidates(x),
                                           _=> FResult::InsufficientCandidates(0),
        }
    }
    
    
    
    pub fn compute_hashtable(&self, dimen: Option<usize>, multiplier: Option<u32>,bound: Option<u64>) -> FResult<HashTable> {
       
       match dimen{
         Some(x) => {
           loop {
             match self.to_hashtable(Some(x),multiplier,bound){
               FResult::Exhaustive(res) => return FResult::Exhaustive(res),
               _=> {()},
             }
           }
         }
         
         None => {
         
            let mut dm = (self.elements.len()/600).next_power_of_two();
            
            loop {
              match self.to_hashtable(Some(dm),multiplier,bound){
                FResult::Exhaustive(res) => return FResult::Exhaustive(res),
                FResult::InsufficientCandidates(c_count) => {
                   if c_count == 0{
                     dm*=2;
                   }
                   else if dm/c_count > 2{
                      dm*=2;
                   }
               }
               _=> (),
            } // end inner match
         } // end loop 
       } // end middle match
    } // end dimen match
    
    }
    
    
    /// Filters the composites using a BaseVector
    pub fn filter_bvector(&self, fil: &BaseVector<T>) -> Self{
     
        let mut ce = self.clone();
    
        for i in fil.iter(){
           ce = ce.sprp_ce(*i);
        }
      ce
    }
    
    /// Filters the composites using a Hashtable
    pub fn filter_hashtable(&self, ht: &HashTable) -> Self{
        let mut veccy = vec![];
        for i in self.iter().unwrap(){
           if ht.primality(*i){
              veccy.push(*i);
           }
        }
        Self::from_vector_internal(veccy,self.memory_max,self.utf8_flag,self.auto_flag)
    }


    /// Filter by a selected base, collecting all composites that pass
    /// # Usage
    /// This is the run-time equivalent to filter_sprp
    pub fn sprp_ce(&self, base: T) -> Self{
    
       match self.file{
         Some(_) => {
           let ce = filter_strong::<T>(self.file.as_ref().unwrap().try_clone().unwrap(),self.utf8_flag,true,base);
           Self::from_vector_internal(ce,self.memory_max,self.utf8_flag,self.auto_flag)
         }
         None => {
          let mut ce = self.elements.clone();
           if ce.len() > 1000{
            ce= strip_pseudo_st::<T>(&ce[..],base);
                }
           else{     
         ce= strip_pseudo_par::<T>(ce,base);
         }
        Self::from_vector_internal(ce,self.memory_max,self.utf8_flag,self.auto_flag)
         }
       } // match 
        
    }
    
    pub fn k_iterative(&self, k: usize) -> BaseVector<T>{
      let mut ce = self.clone();
      let mut bv = BaseVector::<T>::new(vec![]);
      for i in 0..(k-1){
       let bound = 1_000;
         let (c,_) = unary_strongest_par::<T>(ce.elements.clone(),3,bound*(k as u64+1));
         ce = ce.sprp_ce(T::from_u64(c));
         bv.append(T::from_u64(c));
      }
      let x = exhaustive_par(ce.elements);
      bv.append(T::from_u64(x));
      bv
  }
  
  /// Infinite search, this is short-circuiting and therefore much faster than a strongest search. However it has an unpredictable run time
  pub fn terminating_search(&self) -> FResult<BaseVector<T>>{
     self.load_eval(&|x: Self| FResult::Value(BaseVector::new(vec![T::from_u64(exhaustive_par(x.elements))])))
  }
  
  pub fn iterative_search(&self) -> BaseVector<T>{
      let mut ce = self.clone();
      let mut bv = BaseVector::<T>::new(vec![]);
      
      while ce.len() > 100{
        let bound = 10_000_000_000u64/(ce.len() as u64);
        let (c,_) = unary_strongest_par::<T>(ce.elements.clone(),3,bound);
         ce = ce.sprp_ce(T::from_u64(c));
         bv.append(T::from_u64(c));
      }
      
      let x = exhaustive_par(ce.elements);
      bv.append(T::from_u64(x));
      bv
  }
  
  /// Searches for the strongest base within an interval
  pub fn strongest_search(&self, inf: u64,sup: u64) -> FResult<Point<u64>>{
     self.load_eval(&|x: Self| {
         let (base, count) = unary_strongest_par::<T>(x.elements,inf,sup);
         FResult::Value(Point::<u64>::new(base,count))
     })
        }
  
  pub fn bs_rand(&self) -> BaseVector<T>{
      let b = T::gen_k(64).unwrap();
      let mut bv = BaseVector::new(vec![b]);
      let mut ce =  self.sprp_ce(b);
      
      while ce.len() > 0{
        let b = T::gen_k(64).unwrap();
        ce = ce.sprp_ce(b);
        bv.append(b);
      }
      bv
  }  

}
