use crate::natural::finite::FiniteArith;
use crate::natural::extended::sliceops::{sub_slice,add_slice};

use crate::Epz;

impl<const S: usize> FiniteArith for Epz<S> {

  fn finite_add(&self, other: Self) -> Self{
     let mut res = self.clone();
     let _ = add_slice(&mut res.limbs[..],&other.limbs[..],0u8);
     res
  }
  
  fn overflow_add(&self, other: Self) -> (Self,bool){
     let mut res = self.clone();
     let carry = add_slice(&mut res.limbs[..],&other.limbs[..],0u8);
     (res, carry != 0)
  }
 
  fn finite_sub(&self, other: Self) -> Self{
     let mut res = self.clone();
     let _ = sub_slice(&mut res.limbs[..],&other.limbs[..],0u8);
     res
  }
  
  fn overflow_sub(&self, other: Self) -> (Self,bool){
     let mut res = self.clone();
     let carry = sub_slice(&mut res.limbs[..],&other.limbs[..],0u8);
     (res, carry != 0)
  }
  
  fn finite_neg(&self) -> Self{
     unimplemented!()     
  }
  
  fn finite_mul(&self, other: Self) -> Self{
     unimplemented!()
  }
  
  fn widening_mul(&self, other: Self) -> (Self,Self){
     unimplemented!()
  }
  
  
  fn invert(&self)-> Self{
    unimplemented!()
  }
  
  fn to_float(&self) -> f64{
    // debug_assert!(false)
     // f64::NAN
    unimplemented!()
  }
   
}
