/*
   Slice operations
*/
use crate::natural::extended::inlineops::*;
use std::cmp::Ordering;

pub(crate) fn leading_idx(x: &[u64]) -> usize {
    //if x[x.len()-1] != 0{
    //   return x.len();
    // }
    /*  match x.iter().rev().rposition(|digit| *digit != 0){
       Some(y) => y,
       None => 0,
     }
    */

    let mut len = x.len() - 1;
    while x[len] == 0 && len > 0 {
        len -= 1;
    }
    len
    /*  */
}

pub(crate) fn remove_lead_zeros(x: &mut Vec<u64>) {
    let lead = leading_idx(x);
    x.truncate(lead + 1);
    //while x[x.len()-1] == 0{
    //      x.pop();
    // }
}

pub(crate) fn leading_digit(x: &[u64]) -> u64 {
    x[leading_idx(x)]
}

pub(crate) fn add_slice(x: &mut [u64], y: &[u64], mut carry: u8) -> u8 {
    for (i, j) in x.iter_mut().zip(y.iter()) {
        carry = adc(carry, *i, *j, i);
    }
    carry
}

pub(crate) fn sub_slice(x: &mut [u64], y: &[u64], mut carry: u8) -> u8 {
    for (i, j) in x.iter_mut().zip(y.iter()) {
        carry = sbb(carry, *i, *j, i);
    }
    carry
}

pub(crate) fn shl_slice(x: &mut [u64], shift: u32, mut carry: u64) -> u64 {
    for i in x.iter_mut() {
        carry = carry_shl(carry, *i, shift, i);
    }
    carry
}

pub(crate) fn shl_vector(x: &mut Vec<u64>, shift: u32, mut carry: u64) {
    let scarry = shl_slice(&mut x[..], shift, carry);
    if scarry > 0 {
        x.push(scarry);
    }
}

pub(crate) fn shr_slice(x: &mut [u64], shift: u32, mut carry: u64) -> u64 {
    for i in x.iter_mut().rev() {
        carry = carry_shr(carry, *i, shift, i);
    }
    carry
}

// FIXME swap with multiplication
pub(crate) fn mod_slice(x: &mut [u64], divisor: u64, mut carry: u64) -> u64 {
    for i in x.iter().rev() {
        carry = carry_mod(carry, *i, divisor);
    }
    carry
}

pub(crate) fn scale_slice(x: &mut [u64], scalar: u64, mut addend: u64) -> u64 {
    for i in x.iter_mut() {
        addend = carry_mul(addend, *i, scalar, i)
    }
    addend
}
/*
// FIXME check that this  works
pub(crate) fn mont_slice(x: &[u64], divisor: u64, dinv: u64) -> u64{
    let mut tmp = 0;
    //let mut bw = 0;
    let mut cy = 0;
    for i in x.iter(){
    let mut bw = 0;
       tmp = i-cy;
       if cy > i{
          bw = 1;
       }
       tmp=tmp.wrapping_mul(dinv)+b;
       cy=(((tmp as u128)*(divisor as u128))>>64) as u64;
    }
}
*/
pub(crate) fn div_slice(x: &mut [u64], divisor: u64, mut carry: u64) -> u64 {
    for i in x.iter_mut().rev() {
        carry = carry_div(carry, *i, divisor, i);
    }
    carry
}

pub(crate) fn cmp_slice(a: &[u64], b: &[u64]) -> Ordering {
    for (i, j) in a.iter().rev().zip(b.iter().rev()) {
        if i > j {
            return Ordering::Greater;
        }
        if i < j {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}
/*
// Determines if a is less than b
pub(crate) fn lessthan(a: &[u64], b: &[u64]) -> bool{
   for (i,j) in a.iter().rev().zip(b.iter().rev()){
       if
   }
}
*/
