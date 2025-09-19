use std::slice::Iter;
use std::iter::Map;
use crate::Natural;

// Reduced memory representation of semiprimes pq such that p= 2x+1 q=4x+1
pub struct MonierSemiprime<T: Natural>{
   q: Vec<T>,
}
// FIXME implement Persistent
impl<T: Natural> MonierSemiprime<T>{

   pub(crate) fn new(q: Vec<T>) -> Self{
       Self{q}
   }
   
   pub fn sort(&mut self){
      self.q.sort()
   }
   
   // Iterator over the larger factor
   pub fn q_iter(&self) -> Iter<T>{
      self.q.iter()
   }
   
   pub fn append(&mut self,otra: &mut Self){
       self.q.append(&mut otra.q);
   }
   
   // Iterator over the smaller factor
  // pub fn p_iter(&self) -> Map<Iter<u64>>{
  //    self.q.iter().map(|x| ((*x-1)>>1)+1)
   //}
   
   // Iterator over the semiprimes
   //pub fn iter(&self) -> Map<Iter<u128>>{
   //   self.q.iter().map(|x| (((*x-1)>>1)+1)*x)
  // }
}

impl std::fmt::Display for MonierSemiprime<u64>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
     for rhs in self.q_iter(){
         let lhs = (rhs>>1)+1;
         write!(f,"{}*{}\n",lhs,rhs)?
     }
       Ok(())
  }
}

impl std::fmt::Display for MonierSemiprime<u128>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
     for rhs in self.q_iter(){
         let lhs = (rhs>>1)+1;
         write!(f,"{}*{}\n",lhs,rhs)?
     }
       Ok(())
  }
}

//impl Persistent 
