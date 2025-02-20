/*
   Slice operations
*/
use crate::natural::extended::inlineops::*;

pub (crate) fn add_slice(x: &mut[u64], y: &[u64], mut carry: u8) -> u8{
       for (i,j) in x.iter_mut().zip(y.iter()){
         carry = adc(carry,*i,*j,i);
       }
       carry
}


pub (crate) fn sub_slice(x:  &mut[u64], y: &[u64], mut carry: u8) -> u8{
       for (i,j) in x.iter_mut().zip(y.iter()){
         carry = sbb(carry,*i,*j,i);
       }
       carry
}

pub (crate) fn shl_slice(x: &mut[u64], shift: u32, mut carry: u64) -> u64{
    for i in x.iter_mut().rev(){
        carry = carry_shl(carry, *i, shift, i);
    }
    carry
}

pub (crate) fn shr_slice(x: &mut[u64],shift: u32, mut carry: u64) -> u64{
    for i in x.iter_mut().rev(){
        carry = carry_shr(carry, *i, shift, i);
    }
    carry
}

// FIXME swap with multiplication
pub(crate) fn mod_slice(x: &mut[u64],divisor: u64, mut carry: u64) -> u64{
    for i in x.iter().rev() {
        carry = carry_mod(carry, *i, divisor);
    }
    carry
}

pub(crate) fn div_slice(x: &mut[u64], divisor: u64, mut carry: u64) -> u64{
   for i in x.iter_mut().rev() {
        carry = carry_div(carry, *i, divisor, i);
    }
    carry
}
