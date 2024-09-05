use crate::io::write::format_block;
use crate::fermat::FInteger;
use std::sync::{Arc,atomic::{AtomicU64,AtomicUsize,Ordering}};
use machine_prime::is_prime_wc;
use crate::{CompVector,cvec};
use crate::{result::FResult,search::thread_count};

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
    
  pub fn set_idx(&mut self, val: u64, idx: usize){
      self.table[idx]=val;
  }
  
  // Returns the index and the base that gets selected
  pub fn lut_values<T: FInteger>(&self, x: T) -> (usize,u64){
      let idx = x.hash_shift((32-self.dimen.trailing_zeros()) as usize, self.multiplier);
      (idx,self.table[idx])
  }

    pub fn to_file(&self, locale: &str) -> Option<()> {
        use std::fs::File;
        use std::io::Write;

        match File::create(locale) {
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
      // let hash = x.hash_shift((32-self.dimen.trailing_zeros()) as usize, self.multiplier);
       x.sprp(T::from_u64(self.lut_values(x).1))
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

pub fn corrector_set(&mut self, cvec: CompVector<u64>, integer_max: u64, indices: Vec<usize>){

       let ce : Arc<CompVector<u64>> = Arc::new(cvec);
       let step = (integer_max>>32).wrapping_add(1);
       let indices : Arc<Vec<usize>> = Arc::new(indices);
       let mut v : Vec<AtomicU64> = Vec::new();
       let initial : AtomicU64 = AtomicU64::new(0);
       let mut thread_vec : Vec<std::thread::JoinHandle<()>> = Vec::new();
       let idx : Arc<AtomicUsize> = Arc::new(AtomicUsize::new(usize::MAX));
       let len = indices.len();
       // Initialise Base vector with zeros
       for i in 0..len{
           v.push(AtomicU64::new(0u64));
       }
       
       // increment index, access index 
       
       let values : Arc<Vec<AtomicU64>> = Arc::new(v);
       // include correct path
       let tc = thread_count();
       
       for i in 0..tc{
       
         let ce_i = Arc::clone(&ce);
         let ind_i = Arc::clone(&indices);
         let v_i = Arc::clone(&values);
         let idx_i = Arc::clone(&idx);
         let shift = (32-self.dimen.trailing_zeros()) as usize;
         let multiplier = self.multiplier;
         

         
         thread_vec.push(std::thread::spawn(  move || {
              'search : loop {
              
               let mut c_idx = idx_i.load(Ordering::SeqCst);
                
                 if c_idx != len{
                    c_idx = c_idx.wrapping_add(1);
                 }
                 
                 if c_idx == usize::MAX{
                    c_idx = 0usize
                 }
                
                // Store the current index for other threads to access
                idx_i.store(c_idx, Ordering::SeqCst); 
                
                if c_idx == len{
                //println!("Search break");
                   break 'search;
                }
                
                // Access 
                
                let idx = unsafe{*ind_i.get_unchecked(c_idx)};
                
                //let idx = arc_idx.load(Ordering::SeqCst);
              
             // Calculation
              let mut veccy : CompVector<u64> = cvec![];
   
            for i in ce_i.iter().unwrap(){
                 if i.hash_shift(shift,multiplier)==idx{
                    veccy.push(*i)
                 }
            }

            let mut bases = veccy.terminating_list_st(2,2000).unwrap();
            bases.sort();
            // Residue Class elements under 2^32   
            let mut residues = vec![];
            // Calculate initial odd residue class element
            for i in 0u64..0x100000000{
  
            if i.hash_shift(shift,multiplier)==idx && i%2==1{
               residues.push(i);
            }
     
           }// end residue loop
  
           'bsearch : for b in bases{
             
             'rsearch : for (res_idx,j) in  residues.iter().enumerate(){
                  // Initialise with initial residue class
                 let mut val = *j;
                 
                for _ in 0..step{
                
                 if val.sprp(b){
                 
                   if !is_prime_wc(val){
                        break 'rsearch;
                   }
                   
                }
                // Increment to the next element of the residue class 
                val+=0x100000000;
              } // end residue class loop section
              
            if res_idx == residues.len()-1{
                //println!("{}",b);
                let interim = unsafe{v_i.get_unchecked(c_idx)};
                interim.store(b,Ordering::SeqCst);
                break 'bsearch;
            }
         } // end all residue class loops

       }// end base search
       } // end loop
       }// closure bracket
         ));
     }// end thread loop  
       
       
     // Execute all threads
    for handle in thread_vec {
        handle.join().unwrap();
    }
	
     
    let interim = Arc::try_unwrap(values).unwrap();
	// Convert the vector of Arc bases to 64-bit bases and return
    let output =  interim
            .iter()
            .map(|q| q.load(Ordering::SeqCst))
            .collect::<Vec<u64>>();
            
    // Update hashtable with correct values        
    for (idx,val) in indices.iter().zip(output.iter()){
       self.set_idx(*val,*idx);
    }
       
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
