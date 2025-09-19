use crate::car::MRC_18;
use crate::enums::{Search, AUTO_FLAG, MEMORY_MAX, UTF8_FLAG};
use crate::structures::store::Persistent;
use crate::structures::{Primes,residue::ResidueClass,monier::MonierSemiprime};
use crate::{Natural,FResult,Epz};
use crate::primes::{PARTIAL_WHEEL,WHEEL};
use crate::{CompVector, HashTable};
use std::fs::File;
use std::io::{Read, Write};
use crate::search::thread_count;
use std::sync::{atomic::{Ordering,AtomicU64,AtomicBool},Arc};


/// Vector of Fermat bases to be evaluated sequentially as a full primality test
#[derive(Clone, Debug)]
pub struct BaseSeq<T: Natural> {
    bases: Vec<T>,
    mode: Search,
}

/// Macro initialising a BaseSeq
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

impl<T: Natural> Persistent for BaseSeq<T> {
    fn to_persistent(&self, locale: &str) -> FResult<()> {
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

    fn from_persistent(filename: &str) -> FResult<Self> {
    
        match std::fs::File::open(filename) {
            Ok(mut x) => {
                let mut buffer = String::new();
                match x.read_to_string(&mut buffer) {
                    Ok(_) => {
                        // FIXME Handle unwrap correctly
                        let interim = buffer
                            .split(",").collect::<Vec<&str>>();
                        let mut res = vec![];
                        
                        for i in  interim{
                           match T::from_str(i){
                             Ok(x) => res.push(x),
                             Err(message) => return FResult::Err("Parse error"), 
                           }
                        }   
                           // .map(|z| T::from_str(z).unwrap())
                           // .collect::<Vec<T>>();

                        FResult::Value(Self::new(res))
                    }
                    Err(read_error) => FResult::IOError(read_error),
                }
            }
            Err(file_error) => FResult::IOError(file_error),
        }
    }
}

impl<T: Natural> std::ops::Index<usize> for BaseSeq<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.bases[index]
    }
}

impl<T: Natural + std::str::FromStr> std::str::FromStr for BaseSeq<T>{
  type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
       let sections = input.split(",").collect::<Vec<&str>>();
       
       let mut res = bseq![];
       
       for i in sections{
           match i.parse::<T>(){
            Ok(x) => res.append(x),
            Err(mess) => return Err("Invalid integer in sequence"),
           }
    }
    Ok(res)
  }
}

impl<T: Natural> BaseSeq<T> {

    pub fn new(bases: Vec<T>) -> Self {
        Self {
            bases,
            mode: Search::WeakHeuristic,
        }
    }
    
    pub fn first_primes(k: u64) -> Self{
        let mut p = T::ONE;
        let mut b = bseq![];
        let mut counter = 0u64;
        loop{
          p.successor();
          if p.is_prime(){
            counter+=1;
            b.append(p);
          }
          if counter == k{
             break;
          }
        }
        b
    }   

    pub fn append(&mut self, el: T) {
        self.bases.push(el)
    }

    pub fn set_weak_heuristic(&mut self) {
        self.mode = Search::WeakHeuristic;
    }

    pub fn len(&self) -> usize {
        self.bases.len()
    }

    pub fn set_strong_heuristic(&mut self) {
        self.mode = Search::StrongHeuristic;
    }

    pub fn set_deterministic(&mut self) {
        self.mode = Search::Deterministic;
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.bases.iter()
    }

    pub fn swap(&mut self, new_value: T, idx: usize) -> Option<T> {
        if idx >= self.len() {
            return None;
        }
        let interim = self.bases[idx];
        self.bases[idx] = new_value;
        Some(interim)
    }

    pub fn rand_initialise(len: usize) -> Self {
        let mut veccy = vec![];
        for i in 0..len {
            veccy.push(T::gen_k(T::BYTE_LENGTH * 8).unwrap())
        }
        Self {
            bases: veccy,
            mode: Search::WeakHeuristic,
        }
    }

    pub fn primality(&self, c: T) -> bool {
        for i in self.bases.iter() {
            if !c.sprp(*i) {
                return false;
            }
        }
        return true;
    }

    pub fn generate_pseudoprimes(
        &self,
        inf: T,
        sup: T,
        locale: Option<&str>,
    ) -> FResult<CompVector<T>> {
        if self.mode == Search::Deterministic {
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
                for i in plist.iter() {
                    let lhs = T::from(i);

                    for j in [3, 4, 6].iter() {
                        let rhs = lhs.even_complement(T::from(*j));

                        if rhs.is_prime() {
                            let (prod, flag) = lhs.overflowing_mul(rhs);
                            // If multiplication overflowed or the prod exceeds the bound break loop
                            if flag || !prod.is_bounded_by(T::ZERO, sup) {
                                break;
                            }

                            if prod.is_bounded_by(inf, sup) && self.primality(prod) {
                                out.write(&prod.to_bytes()[..]).unwrap();
                            }
                        }
                    }
                }

                for i in MRC_18 {
                    let carmichael = T::from(i);
                    if carmichael.is_bounded_by(inf, sup) && self.primality(carmichael) {
                        out.write(&carmichael.to_bytes()[..]).unwrap();
                    }
                }
                out.flush().unwrap();

                if self.mode == Search::StrongHeuristic {
                    for i in plist.iter() {
                        let lhs = T::from(i);

                        for j in 2..64 {
                            let rhs = lhs.semi_k_complement(j);

                            if rhs.is_prime() {
                                let (prod, flag) = lhs.overflowing_mul(rhs);

                                if flag || !prod.is_bounded_by(T::ZERO, sup) {
                                    break;
                                }

                                if prod.is_bounded_by(inf, sup) && self.primality(prod) {
                                    out.write(&prod.to_bytes()[..]).unwrap();
                                }
                            }
                        }
                    }
                    out.flush().unwrap();
                }
                return FResult::Value(CompVector::from_file_internal(
                    outfile.try_clone().unwrap(),
                    MEMORY_MAX,
                    UTF8_FLAG,
                    AUTO_FLAG,
                ));
            } // End file write

            // Write all composites to volatile memory (i.e vector)
            None => {
                let mut ce = Vec::<T>::new();

                for i in plist.iter() {
                    let lhs = T::from(i);
                    for j in [3, 4, 6].iter() {
                        let rhs = lhs.even_complement(T::from(*j));

                        if rhs.is_prime() {
                            let (prod, flag) = lhs.overflowing_mul(rhs);

                            if flag || !prod.is_bounded_by(T::ZERO, sup) {
                                break;
                            }

                            if prod.is_bounded_by(inf, sup) & self.primality(prod) {
                                ce.push(prod);
                            }
                        }
                    }
                }

                for i in MRC_18 {
                    if T::from(i).is_bounded_by(inf, sup) & self.primality(T::from(i)) {
                        ce.push(T::from(i))
                    }
                }

                if self.mode == Search::StrongHeuristic {
                    for i in plist.iter() {
                        let lhs = T::from(i);

                        for j in 2..64 {
                            let rhs = lhs.semi_k_complement(j);

                            if rhs.is_prime() {
                                let (prod, flag) = lhs.overflowing_mul(rhs);

                                if flag || !prod.is_bounded_by(T::ZERO, sup) {
                                    break;
                                }

                                if prod.is_bounded_by(inf, sup) && !flag && self.primality(prod) {
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

impl BaseSeq<u64> {
    // Fast check for 128-bit semiprimes, keeps the majority of computation in 64-bit arithmetic
    pub fn check(&self, lhs: u64,rhs: u64) -> bool{
        // Perform a modified fermat check, most composites will fail this
           if !self.bases[0].exp_unit(lhs-1,rhs){
              return false;
           }
        
        let prod = (lhs as u128*rhs as u128);
        
        for i in self.bases.iter(){
          if !prod.sprp(*i as u128){
            return false;
          }
        }
        return true
    }
    
    // Check for semiprimes that are greater than 2^128
    pub fn check_epz(&self, lhs: u128, rhs: u128) -> bool{
        if rhs < 1<<64{
           return self.check(lhs as u64,rhs as u64);
        }
        
        if rhs < 1<<65{
           if (self.bases[0] as u128).exp_unit(lhs-1,rhs){
              return false;
           }
           let prod = lhs*rhs;
           
           for i in self.bases.iter(){
              if !prod.sprp((*i).into()){
                 return false;
              }
           }
           return true;
        } else {
         
        for i in self.bases.iter(){
         if !(*i as u128).exp_unit(lhs-1,rhs){
           return false;
         }
        }
        
        let prod = Epz::<3>::from(lhs)*Epz::<3>::from(rhs);
        
        for i in self.bases.iter(){
           if !prod.sprp((*i).into()){
             return false;
           }
        }
        return true;
    }
   } 
    // Searches for the first composite by successively checking integers 
    pub fn exhaustive_bound(&self) -> FResult<u128>{
           let mut start = 4;
           
           loop{
             if !start.is_prime(){
                if self.primality(start){
                   return FResult::Value(start as u128);
                }
             }
             start+=1;
           }
           FResult::NoCandidate
    }
    
    pub fn k_candidate(&self,k: u64) -> FResult<u128>{
       let mut minbound = u128::MAX;
       
       for i in MRC_18{
          if self.primality(i){
             minbound=i as u128;
             break;
          }
       }
     
     let step = ((minbound).isqrt() as u64/2310u64)+1;
     
     let mut flag = false;
     
     let mut finalstep = step;
     
     for s in 0..step{
      
        if s == finalstep{
           break;
        } 
        
        let x = s*2310;
        
        for j in WHEEL{
    
            let lhs = x+j;
            let xcore = (lhs-1)/2;
            // 23,67,90,43,901,2345678
            if lhs.is_prime(){ // 20845
               for i in 3..k{
               let rhs = i*xcore+1;
               let fermatflag = self.check(lhs,rhs);

              if fermatflag && !flag{

                 let prod = lhs as u128*rhs as u128;
                 if minbound > prod{
                    minbound=prod;
                 }

                 flag = true;
                 finalstep=s*i;
              }
              if fermatflag && flag{
                 let prod = lhs as u128*rhs as u128;
                 if minbound > prod{
                    minbound=prod;
                 }
              }
            }  
       }
     }
   }
     if minbound != u128::MAX{
       return FResult::Value(minbound)
       }
       FResult::NoCandidate
    }
    
    /*
       Searches for semiprimes pq of the form (2x+1)(4x+1)
     
       Necessary conditions for such semiprimes to be pseudoprimes are that 
       
       q be a quadratic residue to all bases, such a condition does not hold for p
       therefore we search for legible q instead
    */
    
    pub fn mr_bound(&self) -> FResult<u128>{
       let mut minbound = u128::MAX;
       
       for i in MRC_18{
          if self.primality(i){
             minbound=i as u128;
             break;
          }
       }
       let stime = std::time::Instant::now();
       // Calculate the residue classes 
       // Partial Wheel is the residue classes of q such that p is prime
       let mut residues = self.mr_residues(1u64<<32);
       residues.sort();
       
       let ring = residues.ring;

       // Multiply by 2 as q is twice as large as p
       // In reality this is a galatic bound that will never be achieved
       let step = (u64::MAX/ring)+1;//((minbound).isqrt() as u64/ring)*2+1;
     
     for s in 0..step{ 
        
        let x = s*ring;
        
        for j in residues.iter(){
    
            let rhs = x+*j;
             let xcore = (rhs-1)/2;
           // let xcore = (rhs-1)/2;
            // 23,67,90,43,901,2345678
            if rhs.is_prime(){ // 20845
                let lhs = xcore+1;
               if lhs.is_prime(){
               if self.check(lhs,rhs){
               return FResult::Value(lhs as u128 * rhs as u128);
               }
               }
            }  
       }
     }
     if minbound != u128::MAX{
       return FResult::Value(minbound)
       }
       FResult::NoCandidate
    }
    
    // Parallelised MR search using a provided residue class and 
    // In: Witness list, Residue Classes, Initial
    pub fn mr_bound_par(&self, residue: ResidueClass, inf: u64, sup: u64) -> FResult<u128>{
         debug_assert!(sup < u64::MAX);
         debug_assert!(inf < sup);
         //Algorithm 
         let residues : Arc<ResidueClass> = Arc::new(residue);
         // 2^64-1 is not a prime therefore it is clearly invalid
         let rhs_candidate : Arc<AtomicU64> = Arc::new(AtomicU64::new(u64::MAX));
         // If inf == 0 this wraps to 2^64-1, but this is fine since we immediately 
         // add 1 before we iterate so it returns to 0
         let index : Arc<AtomicU64> = Arc::new(AtomicU64::new(inf.wrapping_sub(1)));
         let witness : Self = self.clone();
         
         let tc = thread_count();
         
         let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();
        // split interval into (sup-inf)/thread_count sections
         
        // if a pseudoprime is found then set (Integer, scalar)
         
         for i in 0..tc{
           let r_i : Arc<ResidueClass> = Arc::clone(&residues);
           let c_i : Arc<AtomicU64> = Arc::clone(&rhs_candidate);
           let b_i : Self = witness.clone();
           let idx : Arc<AtomicU64> = Arc::clone(&index);
           
           
           thread_vec.push(std::thread::spawn(  move ||{
           
           'search: loop {
                // Get current scalar index and increment by 1
                let mut s_idx = idx.load(Ordering::SeqCst);
                let pseudo = c_i.load(Ordering::SeqCst);
                // if pseudo has been found then halt search
                if pseudo != u64::MAX{
                   break 'search;
                }
                
                if s_idx == sup{
                   break 'search;
                }
                // increment
                s_idx = s_idx.wrapping_add(1);
                
                

                // Store the current index for other threads to access
                idx.store(s_idx, Ordering::SeqCst);
                
                let scalar = s_idx*r_i.ring;
                
                for i in r_i.iter(){
                   let rhs = scalar.wrapping_add(*i);
                   // Overflow detection as the residue classes are always less than the scalar
                   // except when scalar = 0, and overflowing would require the residue classes to exceed
                   // 2^64, impossible to store on any existing machine
                   if rhs < scalar{
                      continue;
                   }
                   
                   if rhs.is_prime(){
                      let lhs = (rhs>>1)+1;
                      if lhs.is_prime(){
                        if b_i.check(lhs,rhs){
                          let existing = c_i.load(Ordering::SeqCst);
                          if rhs < existing{
                             c_i.store(rhs,Ordering::SeqCst);
                          }
                        }
                      }
                   }
                }
           
           } }));
         }
         
         for i in thread_vec{
            i.join().unwrap();
         }
         
         let res = rhs_candidate.load(Ordering::SeqCst);
         if res == u64::MAX{
            return FResult::NoCandidate;
         }
         let rhs = res as u128;
         let lhs = ((rhs-1)/2)+1;
         FResult::Value(lhs*rhs)
    }
    
    pub fn mr_bound_epz_par(&self, residue: ResidueClass, inf: u64, sup: u64) -> FResult<Epz<3>>{
         debug_assert!(sup < u64::MAX);
         debug_assert!(inf < sup);
         let residues : Arc<ResidueClass> = Arc::new(residue);
         let rhs_candidate_hi : Arc<AtomicU64> = Arc::new(AtomicU64::new(u64::MAX));
         let rhs_candidate_lo : Arc<AtomicU64> = Arc::new(AtomicU64::new(u64::MAX));
         let candidate_flag : Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
         let index : Arc<AtomicU64> = Arc::new(AtomicU64::new(inf.wrapping_sub(1)));
         let witness : Self = self.clone();
         
         let tc = thread_count();
         
         let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();
         
         for i in 0..tc{
           let r_i : Arc<ResidueClass> = Arc::clone(&residues);
           let cf_i : Arc<AtomicBool> = Arc::clone(&candidate_flag);
           let clo_i : Arc<AtomicU64> = Arc::clone(&rhs_candidate_lo);
           let chi_i : Arc<AtomicU64> = Arc::clone(&rhs_candidate_hi);
           let b_i : Self = witness.clone();
           let idx : Arc<AtomicU64> = Arc::clone(&index);
           
        thread_vec.push(std::thread::spawn(  move ||{
           
           'search: loop {
                // Get current scalar index and increment by 1
                let mut s_idx = idx.load(Ordering::SeqCst);
                // Load the candidate flag
                let c_exists = cf_i.load(Ordering::SeqCst);
                // if pseudo has been found then halt search
                if c_exists == true{
                   break 'search;
                }
                // If the supremum has been reached then halt
                if s_idx == sup{
                   break 'search;
                }
                
                // increment
                s_idx = s_idx.wrapping_add(1);
                
                

                // Store the current index for other threads to access
                idx.store(s_idx, Ordering::SeqCst);
                
                let scalar = (s_idx as u128)*(r_i.ring as u128);
                
                for i in r_i.iter(){
                // Overflow is ignored as 
                   let rhs = scalar.wrapping_add(*i as u128);
                   // This uses machine-prime's BPSW, even if rhs is actually composite 
                   // it is of no consequence as primality testing is just an optimisation here 
                   if rhs.is_prime(){
                      let lhs = (rhs>>1)+1;
                      if lhs.is_prime(){
                        if b_i.check_epz(lhs,rhs){
                          let existing = cf_i.load(Ordering::SeqCst);
                          
                          if existing{
                             let c_lo = clo_i.load(Ordering::SeqCst);
                             let c_hi = chi_i.load(Ordering::SeqCst);
                             let cfull = (c_lo as u128)+((c_hi as u128)<<64);
                             if rhs < cfull{
                             clo_i.store(rhs as u64,Ordering::SeqCst);
                             chi_i.store((rhs>>64) as u64,Ordering::SeqCst);
                             }
                          }
                          else{
                             clo_i.store(rhs as u64,Ordering::SeqCst);
                             chi_i.store((rhs>>64) as u64,Ordering::SeqCst);
                             cf_i.store(true,Ordering::SeqCst);  
                          }
                          
                          
                        }
                      }
                   }
                }
           
           } }));
         }
         
         for i in thread_vec{
            i.join().unwrap();
         }
         
         let res = candidate_flag.load(Ordering::SeqCst);
         if !res{
            return FResult::NoCandidate;
         }
         let rhs_hi = rhs_candidate_hi.load(Ordering::SeqCst);//res as u128;
         let rhs_lo = rhs_candidate_lo.load(Ordering::SeqCst);
         let rhs_128 = ((rhs_hi as u128)<<64) + rhs_lo as u128;
         let lhs_128 = (rhs_128>>1)+1;
         let prod = Epz::<3>::from(rhs_128)*Epz::from(lhs_128);
         FResult::Value(prod)
         
    }
    
    // Monier Rabin residues
    pub fn mr_residues(&self, memory_max: u64) -> ResidueClass{
       // Modify to branch for the case of base 2 and set the residue class to 1 mod 8
        let mut residues = ResidueClass::new(PARTIAL_WHEEL.to_vec(),2310);
       for b in self.bases.iter(){
           // If base is less than 10000 calculate the quadratic residues
           if *b < 10000 && b.is_prime(){
            // println!("{}",b);
             if *b == 2{
                 // Replace with in-place promotion
                 residues = residues.unify(&ResidueClass::new(vec![1],8));
             } else {
              let quadratic_residues = ResidueClass::from_qr(*b);
              // Limit RAM consumption
              if quadratic_residues.cardinality()*residues.cardinality() > (memory_max>>3) as usize{
                 break;
              }
              match residues.checked_unify(&quadratic_residues,memory_max){
                FResult::MemoryExceeded(_) => break,
                FResult::Value(x) => residues = x.filter_monier_rabin(),
                _=> break,
              }
           }
          }
       }
       residues
    }
    
    // Quadratic Classes such that 
  pub fn quadratic_residues_prime(&self, start: usize, stop: usize, memory_max: u64) -> ResidueClass{
         debug_assert!(stop > start);
      // Modify to branch for the case of base 2 and set the residue class to 1 mod 8
        let mut residues = ResidueClass::new(PARTIAL_WHEEL.to_vec(),2310);
        
       for b in self.bases[start..stop].iter(){
           // If base is less than 10000 calculate the quadratic residues
           if *b < 10000 && b.is_prime(){
             if *b == 2{
                 // Replace with in-place promotion
                 residues = residues.unify(&ResidueClass::new(vec![1],8));
             } else {
              let quadratic_residues = ResidueClass::from_qr(*b);
              // Limit RAM consumption
              if quadratic_residues.cardinality()*residues.cardinality() > (memory_max>>3) as usize{
                 break;
              }
              match residues.checked_unify(&quadratic_residues,memory_max){
                FResult::MemoryExceeded(_) => break,
                FResult::Value(x) => residues = x,
                _=> break,
              }
           }
          }
       }
       residues
  }

   // Quadratic Classes such that 
  pub fn quadratic_residues(&self, start: usize, stop: usize, memory_max: u64) -> ResidueClass{
         debug_assert!(stop > start);
      // Modify to branch for the case of base 2 and set the residue class to 1 mod 8
        let mut residues = ResidueClass::new(vec![],0);
        
       for b in self.bases[start..stop].iter(){
           // If base is less than 10000 calculate the quadratic residues
           if *b < 10000 && b.is_prime(){
             if *b == 2{
                 // Replace with in-place promotion
                 residues = residues.unify(&ResidueClass::new(vec![1],8));
             } else {
              let quadratic_residues = ResidueClass::from_qr(*b);
              // Limit RAM consumption
              if quadratic_residues.cardinality()*residues.cardinality() > (memory_max>>3) as usize{
                 break;
              }
              match residues.checked_unify(&quadratic_residues,memory_max){
                FResult::MemoryExceeded(_) => break,
                FResult::Value(x) => residues = x,
                _=> break,
              }
           }
          }
       }
       residues
  }

    
 pub fn mr_semiprimes_st(&self, res: ResidueClass,floor: u64, ceiling: u64) -> MonierSemiprime<u128>{
     
     let mut next = vec![];
     
     let ring = res.ring as u128;
     
     for i in floor..ceiling{
         let scalar = (i as u128)*ring;
         
         for j in res.iter(){
             let rhs = scalar+(*j as u128);
             
             if rhs.is_prime(){
                let lhs = (rhs>>1)+1;
                if lhs.is_prime(){
                   if self.check_epz(lhs,rhs){
                      next.push(rhs);
                   }
                }
             }
         }
     }
     
     MonierSemiprime::new(next)     
 }
 
 pub fn mr_semiprimes_par(&self, res: &ResidueClass, floor: u64, ceiling: u64) -> MonierSemiprime<u128>{
     let tc = thread_count();
     
     let part_res = res.partition(tc);
     
     let mut thread_vec: Vec<std::thread::JoinHandle<MonierSemiprime<u128>>> = Vec::new();
     
     for i in 0..tc{
       
       let bclone = self.clone();
       let partial_residue = part_res[i].clone();
       thread_vec.push(std::thread::spawn( move || {
       
         bclone.mr_semiprimes_st(partial_residue,floor,ceiling)
          
       }
       
       )
       
       );
       
     }
     
     let mut mrsp = MonierSemiprime::<u128>::new(vec![]); 
     
     for i in thread_vec{
      
        mrsp.append(&mut i.join().unwrap())
     
     }
     mrsp
 }
 /*
    pub fn  vertical_search_st(&self, residues: &[u64], ring: u64, height: u64) -> MRSemiprime{
       let mut sp 
        for i in 0..height{
          let scalar = i*ring;
           for j in residues{
             let rhs = scalar+j;
             
             if rhs.is_prime(){
                let lhs = (rhs>>1)+1;
                if lhs.is_prime(){
                   if self.check(lhs,rhs){
                   
                   }
                }
             }
           }
        }
        sp
    }
   */ 
    // Return height 
    pub fn partial_cache(&self, residues: &[u64],ring: u64,height: u64) -> (u64,u64){
        for i in 0..height{
          let scalar = i*ring;
           for j in residues{
              let rhs = scalar+j;
              
              if rhs.is_prime(){
                 let lhs = ((rhs-1)/2)+1;
                 
                 if lhs.is_prime(){
                 
                    if self.check(lhs,rhs){
                       return (i,rhs)
                    }
           }
        }
        
        } // end residue search
       }
       return (height,u64::MAX)
    }
    
    pub fn full_cache(&self,height: u64) -> FResult<u128>{
        let residues = self.mr_residues(1u64<<32);
        let mut res = u64::MAX;
        let stride = 65536usize>>1;
        let mut adjusted_height = height;
        let steps = (residues.cardinality()/stride);
        for i in 0..steps{
           let start = stride*i;
           let stop = stride*(i+1);
           let (interim_height,rhs) = self.partial_cache(&residues.elements[start..stop],residues.ring,adjusted_height);
           
           if interim_height < adjusted_height{
              adjusted_height = interim_height;
              res = rhs;
           }
        }
    
    if res == u64::MAX{
            return FResult::NoCandidate;
         }
         let rhs = res as u128;
         let lhs = ((rhs-1)/2)+1;
         FResult::Value(lhs*rhs)
    }
    
    

    pub fn weak_bound(&self) -> FResult<Epz<3>>{
    /*
       let basecount = self.bases.len();
       // Case where 
       if basecount==1{
         return self.exhaustive_bound();
       }
       
       if basecount < 4{
          return self.k_candidate(32);
       }
       
       if basecount < 6{
          return self.k_candidate(16);
       }
       
       if basecount < 8{
          return self.k_candidate(9);
       }
       else{
         let res = self.mr_residues(1u64<<32);
         return self.mr_bound_epz_par(res,0,100000)
       }
       */
       let res = self.mr_residues(1u64<<32);
         return self.mr_bound_epz_par(res,0,100000)
    }
    
    
    /// Returns the first pseudoprime  to the bases, as generated by the Search
    pub fn upper_bound(&self) -> FResult<u128> {
        // default bound, this covers all 64-bit composites
        let mut bound = 1usize << 32;

        // Modifies bound to be number of bases
        if self.bases.len() > 6 {
            bound = 1usize << (32 + (self.bases.len() - 6));
        }
        // restore or generate list of primes
        let plist = Primes::generate_or_restore(bound);
        // Counterexample
        let mut ce: u128 = u128::MAX;
        let mut p_stop = u64::MAX;

        // (2x+1)(4x+1) where 2x+1 and 4x+1 are semiprimes
        for i in plist.iter() {
            let lhs = i;
            let rhs = i.even_complement(4);

            if rhs.is_prime() {
                let (prod, flag) = lhs.overflowing_mul(rhs);
                // If product is greater than 2^64 promote to 128-bit arithmetic
                if flag {
                    let prod_128 = lhs as u128 * rhs as u128;
                    let mut inner_flag = false;

                    for j in self.bases.iter() {
                        if prod_128.sprp(u128::from(*j)) == false {
                            inner_flag = true;
                            break;
                        }
                    }
                    // Set ce to the found product
                    if inner_flag == false {
                        ce = prod_128;
                        p_stop = lhs;
                        break;
                    }
                } else {
                    let mut inner_flag = false;

                    for j in self.bases.iter() {
                        if prod.sprp(*j) == false {
                            inner_flag = true;
                            break;
                        }
                    }
                    // Set ce to the found product
                    if inner_flag == false {
                        ce = prod as u128;
                        p_stop = lhs;
                        break;
                    }
                }
            }
        }

        if self.mode == Search::StrongHeuristic || self.mode == Search::Deterministic {
            for i in plist.iter() {
                let lhs = i;

                if lhs > p_stop {
                    break;
                }

                for k in 2..256 {
                    let rhs = i.semi_k_complement(k);

                    if rhs.is_prime() {
                        let (secprod, flag) = lhs.overflowing_mul(rhs);

                        // If product is greater than 2^64 promote to 128-bit arithmetic
                        if flag {
                            let secprod_128 = lhs as u128 * rhs as u128;
                            let mut secinner_flag = false;

                            for j in self.bases.iter() {
                                if secprod_128.sprp(u128::from(*j)) == false {
                                    secinner_flag = true;
                                    break;
                                }
                            }
                            // Set ce to the found product
                            if secinner_flag == false && secprod_128 < ce {
                                ce = secprod_128;
                                break;
                            }
                        } else {
                            let mut secinner_flag = false;

                            for j in self.bases.iter() {
                                if secprod.sprp(*j) == false {
                                    secinner_flag = true;
                                    break;
                                }
                            }
                            // Set ce to the found product
                            if secinner_flag == false && (secprod as u128) < ce {
                                ce = (secprod as u128);
                                break;
                            }
                        }
                    } // conditional prime check
                } // end k loop
            } // primeloop
        } // end branch
        if self.mode == Search::Deterministic {
            return FResult::NotSupported;
        }
        if ce == u128::MAX {
            return FResult::NoCandidate;
        }
        //
        return FResult::Value(ce);
    }

    // If Strong heuristic
    // If Deterministic block

    // generate_pseudoprimes(&self,inf: T, sup: T ) -> CompVector<T>
}

impl<T: Natural> std::fmt::Display for BaseSeq<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let zepto = self
            .bases
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let quokka = zepto.join(",");
        write!(f, "{}", quokka)
    }
}
