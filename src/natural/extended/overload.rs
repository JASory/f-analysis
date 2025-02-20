use crate::natural::extended::inlineops::*;
use crate::natural::extended::sliceops::*;
use crate::natural::extended::traitimpl::Epz;
use crate::{FResult,Natural};

/*
   Shift left
   
   Get the number of elements to shift
   
*/

// FIXME incorrect for shift > 64
impl<const S: usize> std::ops::ShlAssign<u32> for Epz<S>{
   fn shl_assign(&mut self,shift: u32){
      let mut carry = 0u64;
      let offset = (shift>>6) as usize;
      
      let shift = shift&63;
      
      debug_assert!(offset < S);
      
      if offset >= S{
         *self = Self::ZERO;
      }
      else{
      for i in 0..(S-offset){
        self.limbs.swap(S-i-offset-1, S-i-1);
      }
      
      let carry = shl_slice(&mut self.limbs[..],shift,0u64);
    }
    debug_assert!(carry==0)
   }
}

// FIXME
impl<const S: usize> std::ops::ShrAssign<u32> for Epz<S>{
   fn shr_assign(&mut self,shift: u32){
      let mut carry = 0u64;
      let offset = (shift>>6) as usize;
      
      let shift = shift&63;
      
      debug_assert!(offset < S);
      
      if offset >= S{
         *self = Self::ZERO;
      }
      
      else{
      
      for i in 0..(S-offset){
        self.limbs.swap(S-offset-i-1,S-i-1);
      }
      
      let carry = shr_slice(&mut self.limbs[..],shift,0u64);
    }
    debug_assert!(carry==0)
   }
}

impl<const S: usize> std::ops::Shl<u32> for Epz<S>{
          type Output = Self;
       fn shl(self,shift: u32)-> Self::Output{
          let mut clonus = self;
          clonus<<=shift;
          clonus
       }
}

impl<const S: usize> std::ops::SubAssign for Epz<S>{
   fn sub_assign(&mut self, otra: Self){
      let carry = sub_slice(&mut self.limbs[..],&otra.limbs[..],0u8);
       debug_assert!(carry==0)
   }
}

impl<const S: usize> std::ops::AddAssign for Epz<S>{
   fn add_assign(&mut self, otra: Self){
      let carry = add_slice(&mut self.limbs[..],&otra.limbs[..],0u8);
       debug_assert!(carry==0)
   }
}

impl<const S: usize> std::ops::Sub for Epz<S>{
    type Output = Self;
   fn sub(self, otra: Self)-> Self::Output{
      let mut clonus = self;
      clonus-=otra;
      clonus
   }
}

impl<const S: usize> std::ops::Add for Epz<S>{
      type Output = Self;
   fn add(self, otra: Self)-> Self::Output{
      let mut clonus = self;
      clonus+=otra;
      clonus
   }
}

impl<const S: usize> std::ops::BitAndAssign for Epz<S>{
   fn bitand_assign(&mut self, otra: Self){
       for (i,j) in self.limbs.iter_mut().zip(otra.limbs.iter()){
          *i&=j;
       }
   }
}


impl <const S: usize> std::ops::BitOrAssign for Epz<S>{
   fn bitor_assign(&mut self, otra: Self){
       for (i,j) in self.limbs.iter_mut().zip(otra.limbs.iter()){
          *i|=j;
       }
   }
}

impl<const S: usize> std::ops::BitXorAssign for Epz<S>{
   fn bitxor_assign(&mut self, otra: Self){
       for (i,j) in self.limbs.iter_mut().zip(otra.limbs.iter()){
          *i^=j;
       }
   }
}

impl<const S: usize> std::ops::BitAnd for Epz<S>{
      type Output = Self;
   fn bitand(self, otra: Self)-> Self::Output{
      let mut interim = self;
      interim&=otra;
      interim
   }
}

impl<const S: usize> std::ops::BitOr for Epz<S>{
      type Output = Self;
   fn bitor(self, otra: Self)-> Self::Output{
      let mut interim = self;
      interim|=otra;
      interim
   }
}

impl<const S: usize> std::ops::BitXor for Epz<S>{
      type Output = Self;
   fn bitxor(self, otra: Self)-> Self::Output{
      let mut interim = self;
      interim^=otra;
      interim
   }
}

impl<const S: usize> std::ops::Not for Epz<S>{
      type Output = Self;
    fn not(self) -> Self::Output{
      let mut out = Self::ZERO;
       for (i,j) in self.limbs.iter().zip(out.limbs.iter_mut()){
          *j|= !i;
       }
       out
    }  
}


impl<const S: usize> std::ops::Neg for Epz<S>{
        type Output = Self;
    fn neg(self) -> Self::Output{
       Self::ZERO-self
    }
}



impl<const S: usize> Epz<S>{
   
   
   pub fn leading_zeros(&self) -> u32{
      for (idx,el) in self.limbs.iter().rev().enumerate(){
        if *el != 0{
          return el.leading_zeros() + 64*(idx as u32);
        }
      }
      self.limbs[0].leading_zeros()
   }
   
   pub fn trailing_zeros(&self) -> u32{
      
      for (idx,el) in self.limbs.iter().enumerate(){
        if *el != 0{
          return el.trailing_zeros() + 64*(idx as u32);
        }
      }
      self.limbs[0].trailing_zeros()
   }
   
}


impl<const S: usize> std::str::FromStr for Epz<S>{
        type Err = FResult<u64>;
    fn from_str(x : &str) -> Result<Self,Self::Err>{
        let str_length = (x.len() as f64/(2f64.log10()*64f64)).ceil() as usize;
        if str_length > S{
           return Err(FResult::Success);
        }
         return Ok(Self::ZERO);    
    }
}

