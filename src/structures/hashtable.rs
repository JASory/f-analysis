use crate::io::write::format_block;
use crate::fermat::FInteger;

use crate::CompVector;
use crate::result::FResult;

/// Structure for hashtable primality test
#[derive(Clone)]
pub struct HashTable {
    dimen: usize,
    multiplier: u32,
    table: Vec<u64>,
}

impl HashTable {

    pub fn new(table: Vec<u64>, dimen: usize, multiplier: u32) -> Self {
        HashTable {
            table,
            dimen,
            multiplier,
        }
    }
    
     /// Returns dimension, multiplier, and hashvalues
  pub fn values(&self) -> (usize,u32,Vec<u64>){
       (self.dimen,self.multiplier,self.table.clone())
    }


    pub fn to_file(&self, locale: &str) -> Option<()> {
        use std::fs::File;
        use std::io::Write;

        match File::create_new(locale) {
            Ok(mut out) => {
                let res = self.to_string();
                match out.write_all(res.as_bytes()) {
                    Ok(_) => Some(()),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }
    
    
    pub fn from_file(filename: &str) -> FResult<Self>{
        use std::io::BufRead;
        
        //FIXME handle parsing error 
        let parser = |x: &str| -> u64{
          let z = x.chars().filter(|k| k.is_digit(10)).collect::<String>();
          z.parse::<u64>().unwrap()
        };
  
       let splitter = |x: &str| -> Vec<u64>{
          let mut z = x.split(",").collect::<Vec<&str>>();
          z.pop();
          z.iter().map(|y| parser(y)).collect::<Vec<u64>>()
       };
        
        let mut div = 0usize;
        let mut mul = 0u32;
        let mut param = vec![];
        
        match std::fs::File::open(filename){
          Ok(x) => {
            let r = std::io::BufReader::new(x);
            
            for (idx,el) in r.lines().enumerate(){
             let interim = el.unwrap();
              if idx == 0{
                 
                 let q = interim.split(':').collect::<Vec<&str>>();
                 div = (1usize<<32)/(parser(q[1]) as usize);
                 mul = parser(q[2]) as u32;
              }
              if idx > 2{
                
                let mut z = splitter(interim.clone().as_str());
                param.extend_from_slice(&mut z[..]);
              }
            }
            
            let res = HashTable{dimen: div, multiplier: mul, table: param };
            FResult::Value(res)
          }
          Err(file_error) => FResult::IOError(file_error),
        }
        
    }
    
    /// Evaluates primality for an integer, utilizing the hashtable computed
    pub fn primality<T: FInteger>(&self, x: T) -> bool{
       let hash = x.hash_shift((32-self.dimen.trailing_zeros()) as usize, self.multiplier);
       x.sprp(T::from_u64(self.table[hash]))
    }
    
    // FIXME Allow Composite to be a file
    /// Checks that the hashtable eliminates all composites from the vector
    pub fn prove<T: FInteger>(&self, cvec: &CompVector<T>) -> FResult<T>{
         for i in cvec.to_vector().iter(){
            if self.primality(*i){
              return FResult::ProofFailed;
            }
         }
         return FResult::Verified
    }
    
    /// FIXME Allow composite to be file
   pub fn count_failure<T: FInteger>(&self, cvec: &CompVector<T>) -> u64{
       let mut count = 0u64;
       for i in cvec.to_vector().iter(){
         if self.primality(*i){
           count+=1;
         }
       }
       count
   }
   
  pub fn list_failure<T: FInteger>(&self, cvec: &CompVector<T>) -> CompVector<T>{
       let mut veccy = vec![];
       for i in cvec.to_vector().iter(){
         if self.primality(*i){
           veccy.push(*i);
         }
       }
       CompVector::<T>::from_vector(veccy)
   }
}

impl std::fmt::Display for HashTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = (1u64 << (32 - self.dimen.trailing_zeros())).to_string();
        let m = self.multiplier.to_string();
        let q = format_block::<16, u64>(&self.table);

        write!(f, "divisor: {} multiplier: {} \n hash(x)  = (x as u32).wrapping_mul({})/{} \n {}", d, m,m,d, q)
    }
}
