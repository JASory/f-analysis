use crate::natural::extended::inlineops::*;
use crate::natural::extended::muldiv::*;
use crate::natural::extended::sliceops::*;
use crate::natural::extended::traitimpl::Epz;
use crate::{FResult, Natural};
use std::ops::*;

/*
   Shift left

   Get the number of elements to shift

*/

// FIXME incorrect for shift > 64
impl<const S: usize> ShlAssign<u32> for Epz<S> {
    fn shl_assign(&mut self, shift: u32) {
        let mut carry = 0u64;
        let offset = (shift >> 6) as usize;

        let shift = shift & 63;

        debug_assert!(offset < S);

        if offset >= S {
            *self = Self::ZERO;
        } else {
            for i in 0..(S - offset) {
                self.limbs.swap(S - i - offset - 1, S - i - 1);
            }

            let carry = shl_slice(&mut self.limbs[..], shift, 0u64);
        }
        debug_assert!(carry == 0)
    }
}

// FIXME
impl<const S: usize> ShrAssign<u32> for Epz<S> {
    fn shr_assign(&mut self, shift: u32) {
       *self = *self>>shift;
    }
}

impl<const S: usize> Shl<u32> for Epz<S> {
    type Output = Self;
    fn shl(self, shift: u32) -> Self::Output {
        let mut clonus = self;
        clonus <<= shift;
        clonus
    }
}

impl<const S: usize> Shr<u32> for Epz<S> {
    type Output = Self;
    fn shr(self, shift: u32) -> Self::Output {
        let mut res = Self::ZERO;
        
        let offset = (shift >> 6) as usize;

        let shift = shift & 63;

        debug_assert!(offset < S);
        
        let lastidx = S-1;
        
        for i in 0..(S-offset){
          res.limbs[lastidx-offset-i]=self.limbs[lastidx-i];
        }
        
        let carry = shr_slice(&mut res.limbs[..], shift, 0u64);
        
        debug_assert!(carry==0);
        
        res
        //let mut clonus = self;
        //clonus >>= shift;
        //clonus
    }
}

impl<const S: usize> SubAssign for Epz<S> {
    fn sub_assign(&mut self, otra: Self) {
        let carry = sub_slice(&mut self.limbs[..], &otra.limbs[..], 0u8);
        debug_assert!(carry == 0)
    }
}

impl<const S: usize> AddAssign for Epz<S> {
    fn add_assign(&mut self, otra: Self) {
        let carry = add_slice(&mut self.limbs[..], &otra.limbs[..], 0u8);
        debug_assert!(carry == 0)
    }
}

impl<const S : usize> AddAssign<u64> for Epz<S>{

   fn add_assign(&mut self, increment: u64){
   
      let mut carry = adc(0u8,self.limbs[0],increment,&mut self.limbs[0]);
      
      if carry == 1{
         for i in self.limbs[1..].iter_mut(){
            carry = adc(carry,*i,0,i);
         }
      }
      debug_assert!(carry==0)
   }
}

impl<const S: usize> Sub for Epz<S> {
    type Output = Self;
    fn sub(self, otra: Self) -> Self::Output {
        let mut clonus = self;
        clonus -= otra;
        clonus
    }
}

impl<const S: usize> Add for Epz<S> {
    type Output = Self;
    fn add(self, otra: Self) -> Self::Output {
        let mut clonus = self;
        clonus += otra;
        clonus
    }
}

impl<const S: usize> BitAndAssign for Epz<S> {
    fn bitand_assign(&mut self, otra: Self) {
        for (i, j) in self.limbs.iter_mut().zip(otra.limbs.iter()) {
            *i &= j;
        }
    }
}

impl<const S: usize> BitOrAssign for Epz<S> {
    fn bitor_assign(&mut self, otra: Self) {
        for (i, j) in self.limbs.iter_mut().zip(otra.limbs.iter()) {
            *i |= j;
        }
    }
}

impl<const S: usize> BitXorAssign for Epz<S> {
    fn bitxor_assign(&mut self, otra: Self) {
        for (i, j) in self.limbs.iter_mut().zip(otra.limbs.iter()) {
            *i ^= j;
        }
    }
}

impl<const S: usize> BitAnd for Epz<S> {
    type Output = Self;
    fn bitand(self, otra: Self) -> Self::Output {
        let mut interim = self;
        interim &= otra;
        interim
    }
}

impl<const S: usize> BitOr for Epz<S> {
    type Output = Self;
    fn bitor(self, otra: Self) -> Self::Output {
        let mut interim = self;
        interim |= otra;
        interim
    }
}

impl<const S: usize> BitXor for Epz<S> {
    type Output = Self;
    fn bitxor(self, otra: Self) -> Self::Output {
        let mut interim = self;
        interim ^= otra;
        interim
    }
}

impl<const S: usize> Not for Epz<S> {
    type Output = Self;
    fn not(self) -> Self::Output {
        let mut out = Self::ZERO;
        for (i, j) in self.limbs.iter().zip(out.limbs.iter_mut()) {
            *j |= !i;
        }
        out
    }
}

impl<const S: usize> Neg for Epz<S> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::ZERO - self
    }
}

// Find a way to check if computation overflowed
impl<const S: usize> Mul for Epz<S> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut zero = Self::ZERO;
        mul_slice(&self.limbs[..], &other.limbs[..], &mut zero.limbs[..]);
        zero
    }
}

impl<const S: usize> Div for Epz<S> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let mut zero = Self::ZERO;
        let mut k = self.clone();
        let mut ring = other.clone();
        let shift = leading_digit(&k.limbs).leading_zeros();
        let leading_k = leading_idx(&k.limbs);
        let leading_ring = leading_idx(&ring.limbs);
        shl_slice(&mut ring.limbs[..], shift, 0);
        let carry = shl_slice(&mut k.limbs[..], shift, 0);
        println!("ring : {}",ring);
        quo_rem_slice(&mut k.limbs[..leading_k+1].to_vec(), &ring.limbs[..leading_ring+1], &mut zero.limbs[..]);
        zero
    }
}

impl<const S: usize> Rem for Epz<S> {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        let mut k = self.clone();
        rem_slice(&mut k.limbs.to_vec(), &other.limbs[..]);
        k
    }
}


/**/

impl<const S: usize> Epz<S> {
    pub fn leading_zeros(&self) -> u32 {
        for (idx, el) in self.limbs.iter().rev().enumerate() {
            if *el != 0 {
                return el.leading_zeros() + 64 * (idx as u32);
            }
        }
        self.limbs[0].leading_zeros()
    }

    pub fn trailing_zeros(&self) -> u32 {
        for (idx, el) in self.limbs.iter().enumerate() {
            if *el != 0 {
                return el.trailing_zeros() + 64 * (idx as u32);
            }
        }
        self.limbs[0].trailing_zeros()
    }

    pub fn mul_mod(&self, y: Self, n: Self) -> Self {
    
        let mut k: Vec<u64> = vec![0u64; S * 2];
        let mut nc = n.clone();
        let mut zero = Self::ZERO;

        mul_slice(&self.limbs[..], &y.limbs[..], &mut k[..]);

        let leading_prod = leading_idx(&k);
        let leading_ring = leading_idx(&nc.limbs[..]);
        
        if leading_prod <= leading_ring {
            if cmp_slice(&k[..leading_prod], &nc.limbs[..leading_ring]) == std::cmp::Ordering::Less
            {
                for i in 0..S {
                    zero.limbs[i] = k[i];
                }
                return zero;
            }
        }

        let shift = leading_digit(&nc.limbs[..]).leading_zeros();
        shl_slice(&mut nc.limbs[..], shift, 0);
        let carry = shl_slice(&mut k[..], shift, 0);

        rem_slice(&mut k, &nc.limbs[..leading_ring + 1]);
        shr_slice(&mut k[..], shift, 0);
        for i in 0..k.len() {
            zero.limbs[i] = k[i];
        }

        zero
    }
}

