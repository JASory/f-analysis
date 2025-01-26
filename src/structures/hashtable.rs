use crate::io::write::format_block;
use crate::fermat::FInteger;
use std::sync::{Arc,atomic::{AtomicU64,AtomicUsize,AtomicBool,Ordering}};
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
  
  pub fn get_idx(&self, idx: usize) -> u64{
      self.table[idx]
  }
  
  // Returns the index and the base that gets selected
  pub fn lut_values<T: FInteger>(&self, x: T) -> (usize,u64){
      //println!("{}",self.table.len());
      let idx = x.hash_shift((32-self.dimen.trailing_zeros()) as usize, self.multiplier);
      (idx,self.table[idx])
  }

    pub fn to_file(&self, locale: &str) -> FResult<()> {
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
    
    /// Fix errors
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
            
            if param.len() != div{
               return FResult::Err("Table of incorrect dimensions");
            }
            
            let res = HashTable{dimen: div, multiplier: mul, table: param };
            FResult::Value(res)
          }
          Err(file_error) => FResult::IOError(file_error),
        }
        
    }
    
    /// Evaluates primality for an integer, utilizing the hashtable computed
    pub fn primality<T: FInteger>(&self, x: T) -> bool{
       x.sprp(T::from_u64(self.lut_values(x).1))
    }
    
    // FIXME Allow Composite to be a file
    /// Checks that the hashtable eliminates all composites from the vector
    pub fn prove<T: FInteger>(&self, cvec: &CompVector<T>) -> FResult<T>{
         for i in cvec.to_vector().iter(){
            if self.primality(*i){
              return FResult::Failure;
            }
         }
         return FResult::Success
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
   
   pub fn failure_interval(&self, inf: u64, sup: u64, total: bool) -> CompVector<u64>{
   
       let tc = thread_count();
       let stride = (sup-inf)/(tc as u64);
       let mut thread_vec : Vec<std::thread::JoinHandle::<Vec<u64>>> = Vec::new();
       
       for i in 0..tc{
         let ht = self.clone();
         let start = inf+(stride* (i as u64));
         let stop = start+stride;
         thread_vec.push(std::thread::spawn(move || {
         let mut veccy = vec![];
         if total{
         for i in start..stop{
           let base = ht.lut_values(i).1;
            if i&2 == 0 || i%3 == 0 || i%5==0 || i%7==0{
               if i.sprp(base){
                  veccy.push(i);
               }
            }
            else{
               if i.sprp(base){
                 if !i.sprp(2){
                   veccy.push(i)
                 }
               }
            }
         }
         }
         if !total{
         for i in start..stop{
           if i&1 == 0{
             continue;
           }
           let base = ht.lut_values(i).1;
            if i%3 == 0 || i%5==0 || i%7==0{
               if i.sprp(base){
                  veccy.push(i);
               }
            }
            else{
               if i.sprp(base){
                 if !i.sprp(2){
                   veccy.push(i)
                 }
               }
            }
         }
         }
         veccy
       }
         ));
         
       }
       let mut veccy = vec![];
       for i in thread_vec{
         let interim = i.join().unwrap();
         veccy.extend_from_slice(&interim[..]);
       }
       CompVector::from(veccy)
   }
   
pub fn  update(&self,sup: u64,flag: bool) -> FResult<Self>{
      if !sup.is_power_of_two(){
         return FResult::NotSupported
      }
      
      let errors = self.failure_interval(4,sup,flag);
      
      let mut ht = self.clone();
      let mut idx_set = std::collections::HashSet::new();
      for i in errors.iter().unwrap(){
         idx_set.insert(i.hash_shift((32-self.dimen.trailing_zeros()) as usize,self.multiplier));
      }
      let indices = idx_set.drain().collect::<Vec<usize>>();
      
      match ht.corrector_set(errors,sup,indices,flag){
         FResult::Value(_) => FResult::Value(ht.clone()),
         _=> FResult::NotSupported,
      }
}   
   
pub fn lesser_indices(&self, other: &Self) -> FResult<Vec<usize>>{
    if self.dimen != other.dimen{
       return FResult::Err("Unequal dimensions");
    }
    if self.multiplier != other.multiplier{
       return FResult::Err("Unequal hashes");
    }
    let mut indices = vec![];
    for idx in 0..self.dimen{
       if self.table[idx] < other.table[idx]{
          indices.push(idx)
       }
    }
    FResult::Value(indices)
}   

pub fn promote(&mut self, other: &Self) -> FResult<&str>{
    if self.dimen != other.dimen{
       return FResult::Err("Unequal dimensions");
    }
    if self.multiplier != other.multiplier{
       return FResult::Err("Unequal hashes");
    }
    
    for (i,j) in self.table.iter_mut().zip(other.table.iter()){
        if j > i{
          *i = *j;
        }
    }
    FResult::Success
}

pub fn corrector_set(&mut self, cvec: CompVector<u64>, integer_max: u64, indices: Vec<usize>, total: bool) -> FResult<CompVector<u64>>{

       let ce : Arc<CompVector<u64>> = Arc::new(cvec);
       let step = (integer_max>>32).wrapping_add(1);
       let indices : Arc<Vec<usize>> = Arc::new(indices);
       let mut v : Vec<AtomicU64> = Vec::new();
       let initial : AtomicU64 = AtomicU64::new(0);
       let mut thread_vec : Vec<std::thread::JoinHandle<()>> = Vec::new();
       let idx : Arc<AtomicUsize> = Arc::new(AtomicUsize::new(usize::MAX));
       let error_idx : Arc<AtomicUsize> = Arc::new(AtomicUsize::new(usize::MAX));
       let terminator : Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
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
         let eidx_i = Arc::clone(&error_idx);
         let t_i = Arc::clone(&terminator);
         let shift = (32-self.dimen.trailing_zeros()) as usize;
         let multiplier = self.multiplier;
         

         
         thread_vec.push(std::thread::spawn(  move || {
              'search : loop {
              
               let term_flag = t_i.load(Ordering::SeqCst);
               
               if term_flag{
                   break 'search;
               }
               
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
                //println!("{} {}",c_idx,idx);
                //let idx = arc_idx.load(Ordering::SeqCst);
              
             // Calculation
              let mut veccy : CompVector<u64> = cvec![];
   
            for i in ce_i.iter().unwrap(){
                 if i.hash_shift(shift,multiplier)==idx{
                    veccy.push(*i)
                 }
            }
             // Residue Class elements under 2^32   
            let mut residues = vec![];
            // Calculate initial odd residue class element
            if total{
              for i in 0u64..0x100000000{
  
                if i.hash_shift(shift,multiplier)==idx{
                   residues.push(i);
                }
              }// end residue loop
  
            }
            if !total{
            
             for i in 0u64..0x100000000{
  
               if i.hash_shift(shift,multiplier)==idx && i%2==1{
                 residues.push(i);
               }
     
             }// end residue loop
          }
           // Set break loop if a valid base is found
           // If no base is found then either panic or set error flag
          'incsearch : for i in 0..64{
            //println!("stepped at {}",i);
            let mut  flag : bool = false;
            let mut bases = vec![];
            
            if i == 0{
              bases = veccy.terminating_list_st(2,1024).unwrap();     
            } else{
              bases = veccy.terminating_list_st(i*1024,(i+1)*1024).unwrap();
            }

            bases.sort();
          //  println!("{:?}",bases);
  
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
                flag=true;
                let interim = unsafe{v_i.get_unchecked(c_idx)};
                interim.store(b,Ordering::SeqCst);
                break 'bsearch;
            }
         } // end all residue class loops

       }// end base search
         if flag{
            break 'incsearch;
         }
         // if you reach the end of the incremental search loop without finding a value  then set a flag
         if i == 63{
         // set flag
            t_i.store(true,Ordering::SeqCst);
         // set the error idx   
            eidx_i.store(c_idx, Ordering::SeqCst); 
         }
       } // end incremented base search
       
       } // end loop
       }// closure bracket
         ));
     }// end thread loop  
       
       
     // Execute all threads
    for handle in thread_vec {
        handle.join().unwrap();
    }
	
	if terminator.load(Ordering::SeqCst){
	    let idx = error_idx.load(Ordering::SeqCst);
	   return FResult::InsufficientCandidates(idx);
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
    FResult::Value(CompVector::from(output))    
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
