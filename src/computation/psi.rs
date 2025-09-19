use crate::{structures::{BaseSeq,ResidueClass,store::Persistent},search::thread_count,Epz,FResult};

/*

  Estimation of Jaeschke's psi_k function 

  the minimum integer such that it is a strong pseudoprime to the first k primes

   

*/
/*
// MAX Ring 2459559130353965640
// Return the height 
fn bound_est(k: u64, ring: u64) -> u64{
   
}
*/
enum Execution{
   // If a known bound is given return K
   KnownBound,
   // If a known psi is given find the bound
   KnownPsi,
   // If both the psi and bound are given 
   // then enumerate the pseudoprimes 
   List,
}

fn appender(data: &str, file: &str){
   use std::io::{Write,Seek,SeekFrom};
   let mut file = std::fs::OpenOptions::new().write(true).open(file).unwrap();
   file.seek(SeekFrom::End(0));
   file.write(&data.as_bytes());
}

// 
fn load_bounds(input: &str) -> Option<(u64,u64)>{
   let strin = std::fs::read_to_string(input).unwrap();
   
   let args = strin.split_whitespace().collect::<Vec<&str>>();
   match args.len(){
    1 => {
      match args[0].parse::<u64>(){
        Ok(x) => return Some((0,x)),
        Err(_) => return None,
      }
    }
    2 => {
     match (args[0].parse::<u64>(),args[1].parse::<u64>()){
      (Ok(x),Ok(y)) => Some((x,y)),
      _=> None,
     }
    }
    _ => return None,
   }
}

/**
Large scale estimation of Jaeschke's Ψ function. Ψ(n) is defined as the minimum composite that passes the
strong fermat test to the first n primes.

The current implementation has hardcoded initialisation for pseudoprimes to the first 15 primes. 
It requires approximately 4GB of free memory. 

The maximum bound is 2^192, however it is currently impractical beyond 2^128. 
*/

#[derive(Clone)]
pub struct PsiEval{
     res1: ResidueClass,
     res2: ResidueClass,
     witness: BaseSeq<u64>,
     height: u64,
     index: u64,
     // Execution Mode 
   //  exec: Execution,
     folder: String,
}



impl PsiEval{

  pub fn initialise(k: Option<u64>, memory_bound: u64, bound: Option<Epz<3>>, folder: &str, bound_search: bool){
      let psi = BaseSeq::<u64>::first_primes(k.unwrap());
      psi.to_persistent(&(folder.to_owned()+"/witness"));
      let res1 = BaseSeq::<u64>::first_primes(12).mr_residues(1u64<<32);  
      res1.to_persistent(&(folder.to_owned()+"/primaryresidue"));
       
      let res2 = ResidueClass::from_qr(41).unify(&ResidueClass::from_qr(43)).unify(&ResidueClass::from_qr(47));
      
      res2.to_persistent(&(folder.to_owned()+"/secondaryresidue"));
      
      let ring = Epz::<3>::from(2459559130353965640u64);
      
      let mut height = 1;
      let mut tc : u64 = 1;
      
      if bound_search{
         tc = thread_count() as u64;
      }

      let bnd = bound.unwrap();
      
      while Epz::<3>::from(height*tc)*ring < bnd{
         height+=1;
      } 
    
      height*=tc;
     
      
      std::fs::write(folder.to_owned()+"/height",height.to_string().as_bytes());
      std::fs::File::create(folder.to_owned()+"/pseudoprimes");
  }
  
  
  
  pub fn recover(folder : &str) -> FResult<Self>{
     match std::fs::exists(folder){
        Ok(existence) => {
           if existence{
              let witness = BaseSeq::from_persistent(&(folder.to_owned()+"/witness")).unwrap();
              let primary = ResidueClass::from_persistent(&(folder.to_owned()+"/primaryresidue")).unwrap();
              let secondary = ResidueClass::from_persistent(&(folder.to_owned()+"/secondaryresidue")).unwrap();
              let indexfile = folder.to_owned()+"/index";
              let mut idx = 0u64;
              let height = std::fs::read_to_string(folder.to_owned()+"/height").unwrap().trim().parse::<u64>().unwrap();
              if std::fs::exists(indexfile.clone()).unwrap(){
               idx= std::fs::read_to_string(indexfile).unwrap().trim().parse::<u64>().unwrap();
              }
              return FResult::Value(PsiEval{
               res1: primary,
               res2: secondary,
               witness: witness,
               index : idx,
               height: height,
               folder: folder.to_string(),
              });
           }
           else{
             return FResult::NoCandidate;
           }
        }
        Err(mess) => FResult::IOError(mess), 
     }
  }
  
  /// Enumerate possible bounds
  pub fn bound_search(&mut self){

     
     for (idx,el) in self.res2.iter().enumerate(){
     
       if (idx as u64) < self.index{
          continue;
       }
       
            let mut residues = self.res1.clone();
            
       residues.coprime_promote(*el,82861);
       
       match self.witness.mr_bound_epz_par(residues.clone(),0,self.height){
         FResult::NoCandidate => {std::fs::write(self.folder.clone()+"/index",idx.to_string().as_bytes());},
         FResult::Value(p) => {
               appender(&p.to_string(),&(self.folder.clone()+"/pseudoprimes"));               
         },
         _=> panic!("Unknown error"),
       }
     }
  }
  
   // Calculate height as bound/ring during initialisation
  pub fn enumerate_pseudo(&mut self){

     
     for (idx,el) in self.res2.iter().enumerate(){
     
       if (idx as u64) < self.index{
         continue;
       }

            let mut residues = self.res1.clone();
            
       residues.coprime_promote(*el,82861);
       
       let p = self.witness.mr_semiprimes_par(&residues,0,self.height).to_string();
       
       if p.len() != 0{
       appender(&p,&(self.folder.clone()+"/pseudoprimes"));  
       }  
       std::fs::write(self.folder.clone()+"/index",idx.to_string().as_bytes()); 
       
  }
  
}
  
}
