use crate::natural::ftrait::Natural;
use crate::{FResult,Persistent,search::thread_count};

#[derive(Clone)]
pub struct ResidueClass{
  pub(crate) elements: Vec<u64>,
  pub ring: u64,
}

struct Signature{
  sig: Vec<u64>,
}

 //  
fn nc_crt(res_1: u64, ring_1: u64, res_2: u64, ring_2: u64) -> Option<(u64,u64)>{

    let (g,inv1,inv2) = ring_1.extended_gcd(ring_2);
    if (res_2.abs_diff(res_1))%g != 0{
       return None;
    }
    //let delta = (res_2.abs_diff(res_1))/g;
    let n_ring = (ring_1/g)*ring_2;
    let r_r2 = ring_2/g;
    let r_r1 = ring_1/g;
    let n_ring128 = n_ring as u128;
    let lhs = (res_1 as u128*((inv2 as u128*r_r2 as u128)%n_ring128))%n_ring128;
    let rhs = (res_2 as u128*((inv1 as u128*r_r1 as u128)%n_ring128))%n_ring128;
    Some( (((lhs + rhs)%n_ring128) as u64,n_ring))
    }
    
// Coprime CRT
fn c_crt(res_1: u64, ring_1: u64, res_2: u64, ring_2: u64) -> u64{

   let (g,inv1,inv2) = ring_1.extended_gcd(ring_2);
   
    let n_ring = ring_1*ring_2;
    
    let n_ring128 = n_ring as u128;
    let lhs = (res_1 as u128*((inv2 as u128*ring_2 as u128)%n_ring128))%n_ring128;
    let rhs = (res_2 as u128*((inv1 as u128*ring_1 as u128)%n_ring128))%n_ring128;
    (((lhs + rhs)%n_ring128) as u64)
}

impl ResidueClass{

   pub fn new(elements: Vec<u64>,ring: u64) -> Self{
       Self{elements,ring}
   }
   
   pub fn cardinality(&self) -> usize{
       self.elements.len()
   }
   
   pub fn sort(&mut self){
      self.elements.sort();
   }
   
   pub fn iter(&self) -> std::slice::Iter<u64>{
       self.elements.iter()
   }
   
   pub fn split_at(&mut self,idx: usize) -> Self{
       let high = self.elements[idx..].to_vec();
       self.elements = self.elements[..idx].to_vec();
       Self::new(high,self.ring)
   }
   
   pub fn append(&mut self, otra: &mut Self){
       debug_assert!(self.ring==otra.ring);
       self.elements.append(&mut otra.elements);
   }
   
   // Splits into N partitions 
   pub fn partition(&self,n: usize) -> Vec<Self>{
       let sublen = self.cardinality()/n;
       let mut res = vec![];
       for i in 0..n-1{
           res.push(Self::new(self.elements[sublen*i..sublen*(i+1)].to_vec(),self.ring));
       }
       res.push(Self::new(self.elements[sublen*(n-1)..].to_vec(),self.ring));
       res
   }
   
   // The quadratic residues of N
   pub fn from_qr(n: u64) -> Self{
     let mut r = std::collections::HashSet::new();
     for i in 1..n{
        r.insert((i*i)%n);
     }
     let mut s =r.drain().collect::<Vec<u64>>();
     s.sort();
     
     Self::new(s,n)
   }
   
   // The nonquadratic residues of N
   pub fn from_nqr(n: u64) -> Self{
     let mut r = vec![];
     for i in 2..n{
        if i.jacobi(n)==-1{
          r.push(i);
        }
     }
     Self::new(r,n)
   }

   //fn from_3_4(b: BaseSeq<u64>, Signature) -> Self{
   
   //}
   
   //fn from_1_4() -> Self{
   
   //}
   
   pub fn promote(&self, residue: u64, ring: u64) -> Self{
      let n_ring = self.ring.lcm(ring).unwrap();
      let mut res = vec![];
      
      for i in self.elements.iter(){
         match nc_crt(*i,self.ring,residue,ring){
           Some(x) => res.push(x.0),
           None => (),
         }
      }
      Self::new(res,n_ring)
   }
   
   /*
     let (g,inv1,inv2) = ring_1.extended_gcd(ring_2);
   
    let n_ring = ring_1*ring_2;
    
    let n_ring128 = n_ring as u128;
    let lhs = (res_1 as u128*((inv2 as u128*ring_2 as u128)%n_ring128))%n_ring128;
    let rhs = (res_2 as u128*((inv1 as u128*ring_1 as u128)%n_ring128))%n_ring128;
    (((lhs + rhs)%n_ring128) as u64)
   */
   
   // in-place promotion
   pub fn coprime_promote(&mut self, residue: u64, ring: u64){
     /*
        New algorithm 
        
        (g,inv1,inv2) <- EEA (ring_1,ring_2)
        New_ring <- ring_1*ring_2 
        rhs <- res_2 * inv1*ring_1
        lcofactor <- inv2*ring2 
        
        loop{
          residue <- residue*lcofactor + rhs
        }
     */
       debug_assert!(self.ring.gcd(ring)==1);
       let (g,inv1,inv2) = self.ring.extended_gcd(ring);
       let new_ring = (self.ring*ring) as u128;
       let rhs = ((residue as u128)*((inv1 as u128)*(self.ring as u128)%new_ring))%new_ring;
       let lcofactor = ((inv2 as u128)*(ring as u128))%new_ring;
       
       for i in self.elements.iter_mut(){
           *i= ((((*i as u128)*lcofactor)%new_ring + rhs)%new_ring) as u64;
       }
       self.ring = new_ring as u64;
       /*
       Old algorithm
       
       for i in self.elements.iter_mut(){
          *i=c_crt(*i,self.ring,residue,ring);
       }
       self.ring *=ring;
       */
   }
   
   pub fn coprime_promote_par(&mut self, residue: u64, ring: u64){
       let tc = thread_count();
       let nring = ring*self.ring;
       let mut thread_vec = vec![];
       for mut i in self.partition(tc){
           thread_vec.push(std::thread::spawn(move||{
            i.coprime_promote(residue,ring);
            i
           }))
       }
       let mut res = ResidueClass::new(vec![],nring);
       for i in thread_vec{
           res.append(&mut i.join().unwrap())
       }
       *self=res;
   }
   // List of units to a prime [1;p-1]
   pub fn unit_prime(p: u64) -> Self{
       let mut res = vec![];
       
       for i in 1..p{
          res.push(i);
       }
      Self::new(res,p)
   }
   
   //  Combine two residue class systems
   pub fn unify(&self, otra: &Self) -> Self{
   
     let n_ring = self.ring.lcm(otra.ring).unwrap();
     let mut res = vec![];
     
     for i in self.elements.iter(){
       for j in otra.elements.iter(){
          match nc_crt(*i,self.ring,*j,otra.ring){
            Some(x) => res.push(x.0),
            None => (),
          }
       }
     }
     Self::new(res,n_ring)   
   }
   
   /*
   let n_ring = (self.ring/g)*ring_2;
    let r_r2 = otra.ring/g;
    let r_r1 = self.ring/g;
    let n_ring128 = n_ring as u128;
    let lhs = (res_1 as u128*((inv2 as u128*r_r2 as u128)%n_ring128))%n_ring128;
    let rhs = (res_2 as u128*((inv1 as u128*r_r1 as u128)%n_ring128))%n_ring128;
    Some( (((lhs + rhs)%n_ring128) as u64,n_ring))
    
   */
   
   pub fn checked_unify(&self, otra: &Self, mem_max: u64) -> FResult<Self>{
     //let n_ring = self.ring.lcm(otra.ring).unwrap();
     let mut res = vec![];
     let mut mem = 0u64;
     let (g,inv1,inv2) = self.ring.extended_gcd(otra.ring);
     let n_ring = (self.ring/g)*otra.ring;
     for i in self.elements.iter(){
     
      let lhs = ((*i as u128)*((inv2 as u128)*((otra.ring/g) as u128)%(n_ring as u128) ))%(n_ring as u128);
      let rcofactor = (inv1 as u128)*((self.ring/g) as u128)%(n_ring as u128);
       for j in otra.elements.iter(){
          if i.abs_diff(*j)%g != 0{
             continue;
         }
         let value = ((lhs+(*j as u128)*rcofactor)%(n_ring as u128)) as u64;
         res.push(value);
         mem+=8;
         if mem > mem_max{
            return FResult::MemoryExceeded(mem as usize);
         }
       }
     }
     FResult::Value(Self::new(res,n_ring))   
   }
   
   pub fn filter_monier_rabin(&self) -> Self{
       let mut k = vec![];
       
       for i in self.elements.iter(){
          let iminus = i-1;
          if ((iminus/2+1).gcd(self.ring) ==1) || (((iminus+self.ring)/2)+1).gcd(self.ring)==1{
             k.push(*i);
          }
       }
       Self::new(k,self.ring)
   }
   
}

impl Persistent for ResidueClass{
   
   fn to_persistent(&self,locale: &str) -> FResult<()>{
        use std::fs::File;
        use std::io::Write;

        match File::create(locale) {
            Ok(mut out) => {
                let mut w = std::io::BufWriter::new(out);
                
                match w.write(&self.ring.to_bytes()[..]) {
                    Ok(_) => (),
                    Err(message) => return FResult::IOError(message),
                }
                for i in self.elements.iter(){
                   match w.write(&i.to_bytes()[..]){
                     Ok(_) => (),
                     Err(message) => return FResult::IOError(message),
                   }
                }
            }
            Err(message) => return FResult::IOError(message),
        }
        FResult::Success
   }
   
   fn from_persistent(locale: &str) -> FResult<Self>{
        use std::fs::File;
        use std::io::Read;
          let mut ring = 0u64;
          let mut res = vec![];
        match File::open(locale) {
            Ok(mut out) => {
                let mut r = std::io::BufReader::new(out);
                let mut interim = [0u8;8];
                
                match r.read(&mut interim){
                    Ok(_) => ring = u64::from_bytes(&interim),
                    Err(message) => return FResult::IOError(message),
                }
                
                loop {
                match r.read(&mut interim[..]){
                   Ok(totalbytes) => {
                     if totalbytes == 0{
                        break;
                        }
                        res.push(u64::from_bytes(&interim));
                     }
                   Err(message) => return FResult::IOError(message),
                }
               }
               return FResult::Value(Self::new(res,ring));
               }
             Err(message) => FResult::IOError(message)  
              } 
      }

}
