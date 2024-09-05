use crate::fermat::NTCore;


#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Pseudoprime {
    None,
    Fermat,
    Euler,
    Strong,
}


const INV_8: [u8; 128] = [
    0x01, 0xAB, 0xCD, 0xB7, 0x39, 0xA3, 0xC5, 0xEF, 0xF1, 0x1B, 0x3D, 0xA7, 0x29, 0x13, 0x35, 0xDF,
    0xE1, 0x8B, 0xAD, 0x97, 0x19, 0x83, 0xA5, 0xCF, 0xD1, 0xFB, 0x1D, 0x87, 0x09, 0xF3, 0x15, 0xBF,
    0xC1, 0x6B, 0x8D, 0x77, 0xF9, 0x63, 0x85, 0xAF, 0xB1, 0xDB, 0xFD, 0x67, 0xE9, 0xD3, 0xF5, 0x9F,
    0xA1, 0x4B, 0x6D, 0x57, 0xD9, 0x43, 0x65, 0x8F, 0x91, 0xBB, 0xDD, 0x47, 0xC9, 0xB3, 0xD5, 0x7F,
    0x81, 0x2B, 0x4D, 0x37, 0xB9, 0x23, 0x45, 0x6F, 0x71, 0x9B, 0xBD, 0x27, 0xA9, 0x93, 0xB5, 0x5F,
    0x61, 0x0B, 0x2D, 0x17, 0x99, 0x03, 0x25, 0x4F, 0x51, 0x7B, 0x9D, 0x07, 0x89, 0x73, 0x95, 0x3F,
    0x41, 0xEB, 0x0D, 0xF7, 0x79, 0xE3, 0x05, 0x2F, 0x31, 0x5B, 0x7D, 0xE7, 0x69, 0x53, 0x75, 0x1F,
    0x21, 0xCB, 0xED, 0xD7, 0x59, 0xC3, 0xE5, 0x0F, 0x11, 0x3B, 0x5D, 0xC7, 0x49, 0x33, 0x55, 0xFF,
];



impl NTCore for u64 {

 fn mont_add(&self, y: Self) ->  Self{
     self.wrapping_add(y)
  }

  fn mont_sub(&self, y: Self, n: Self) -> Self{
     if y > *self{
       return n.wrapping_sub(y.wrapping_sub(*self));
     }
     self.wrapping_sub(y)
  }
  
  
  fn n_identity(&self) -> Self{
     (u64::MAX% *self) + 1
  }
  
  fn to_mont(&self, n: Self) -> Self{
     (((*self as u128) << 64)%(n as u128)) as u64
  }

  fn inv_2(&self) -> Self{
     let n =*self;
     let mut est = INV_8[((n>>1) & 0x7F) as usize] as u64;
     est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
     est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
     est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
     est
  }
  
  fn inv_2_neg(&self) -> Self{
     let inv = self.inv_2();
     inv.wrapping_neg()
  }
  
  fn mont_prod(&self, y: Self, n: Self, npi: Self) -> Self{
     let interim = *self as u128 * y as u128;
     let tm = (interim as u64).wrapping_mul(npi);
     let (t,flag) = interim.overflowing_add((tm as u128) * (n as u128));
     let t = (t>>64) as u64;
     
     if flag{
        t+n.wrapping_neg()
     }
     else if t >= n{
       t-n
     }
     else{
        t
     }
  }
  
  fn mont_sqr(&self, n: Self, npi: Self) -> Self{
     self.mont_prod(*self,n,npi)
  }
  
  fn to_z(&self, n: Self, npi: Self) ->  Self{
     let tm = self.wrapping_mul(npi);
     let (t,flag) = (*self as u128).overflowing_add((tm as u128)*(n as u128));
     let t = (t>>64) as u64;
     
     if flag{
        t + n.wrapping_neg()
     } else if t >= n{
        t - n
     }
     else{
       t
     }
  }

 fn mont_pow(&self, mut one: Self, mut p: Self, n: Self, npi: Self) -> Self {
    let mut base = *self;
    
  while p > 1 {
        if p & 1 == 0 {
            base = base.mont_prod(base, n, npi);
            p >>= 1;
        } else {
            one = base.mont_prod(one, n, npi);
            base = base.mont_prod(base, n, npi);
            p = (p - 1) >> 1;
        }
    }
    base.mont_prod(one, n, npi)
}

 fn odd_exp_residue(&self, p: Self, n: Self) -> Self{
    let one = n.n_identity();
    let base = self.to_mont(n);
    let npi = n.inv_2_neg();
    base.mont_pow(one,p,n,npi).to_z(n,npi)
 }
 
 fn even_exp_residue(&self, p: Self, twofactor: Self) -> Self{
       let n = (1<<twofactor)-1;
       let mut z = 1;
       let mut base = *self;
       let mut pow = p;
       while pow > 1 {
           if pow & 1 == 0 {
               base = base.wrapping_mul(base)&n;
                pow >>= 1;
            } else {
                 z = base.wrapping_mul(z)&n;
              base = base.wrapping_mul(base)&n;
               pow = (pow - 1) >> 1
        }
    }
      base.wrapping_mul(z)&n
 }
 
 
 fn expr(&self, p: Self, n: Self) -> Self{
    if n & 1 == 0 {
        let k = n.trailing_zeros() as u64;
        let s = n >> k;

        let reducer = (1 << k) - 1; // A shorthand for arithmetic over Z[2k]

        let k_rem = self.even_exp_residue(p,reducer);//even_pow_64(x, y, reducer); //x.wrapping_pow(y as u32)&reducer;

        let s_rem = self.odd_exp_residue(p,s);
        
        let s_inv = s.inv_2()&reducer;
    
/*
        let mut s_inv = s;
        
        for _ in 0..10 {
            // Multiplicative inverse over Z[2k]
            s_inv = 2u64.wrapping_sub(s_inv.wrapping_mul(s)).wrapping_mul(s_inv) & reducer;
        }
*/
        let y = k_rem.wrapping_sub(s_rem).wrapping_mul(s_inv) & reducer;

        s_rem + s * y
    } else {
        self.odd_exp_residue(p,n)//odd_pow64(x, y, n)
    }
 }

 fn odd_fermat(&self, base: Self) -> bool{
 //println!("branched");
     let one = self.n_identity();
     let npi = self.inv_2_neg();
     let p = *self -1;
     let b = base.to_mont(*self);
     if b.mont_pow(one,p,*self,npi)==one{//.to_z(*self,npi) == 1{
        return true;
     }
     false
 }
 
 fn p_sq_fermat(&self, base: Self) -> bool{
       let n = *self * *self;
       let one = n.n_identity();
       let npi = n.inv_2_neg();
       let p = *self -1;
       let b = base.to_mont(n);
       
       if b.mont_pow(one,p,n,npi) == one{
          return true;
       }
       false
   }
   
    // Full fermat test
 fn fermat(&self,a: Self) -> bool{
    if a.expr(*self-1,*self) == 1{
       return true;
    }
    return false
 }
 
// FIXME - Prove this to have no errors for 2Z except powers of two
 fn sprp(&self, base: Self) -> bool {
 
    let p_minus = *self - 1;
    let twofactor = p_minus.trailing_zeros();
    let mut d = p_minus >> twofactor;
   
    let npi = self.inv_2_neg();
    let one = self.n_identity();
    let mut result = base.to_mont(*self);
    result = result.mont_pow(one,d,*self,npi);
    let oneinv = self.mont_sub(one,*self).mont_prod(one,*self,npi);
    
    if result == one || result == oneinv {
        return true;
    }
    
    for _ in 1..twofactor {
        result = result.mont_sqr(*self,npi);

        if result == oneinv {
            return true;
        }
    }
    false
}

// Simple Multiplicative order

 fn mul_ord(&self,n: Self) -> Self{
    let one = n.n_identity();
    let mut init = *self;
    let npi = n.inv_2_neg();
    
    for i in 0..n{
      init = init.mont_prod(*self,n,npi);
      if init == one{
        return i
      }
    }
     0u64
 }

 fn bounded_ord(&self,n: Self, sup: Self) -> Self{
    let one = n.n_identity();
    let mut init = *self;
    let npi = n.inv_2_neg();
    let x = sup/ *self;
    for i in 0..x{
      init = init.mont_prod(*self,n,npi);
      if init == one{
        return i
      }
    }
     0u64
 }

}

#[inline(always)]
const fn split_to_u128(x: u128) -> (u128, u128) { 
    let (lo,hi) = unsafe { std::mem::transmute::<u128, (u64, u64)>(x) };
    (hi as u128, lo as u128)
}

#[inline] // hi,lo
const fn overflowing_add(lhs: (u128, u128), rhs: (u128, u128)) -> ((u128, u128), bool) {
    let (lo, carry) = lhs.1.overflowing_add(rhs.1);
    let (hi, of1) = lhs.0.overflowing_add(rhs.0);
    let (hi, of2) = hi.overflowing_add(carry as u128);
    ((hi, lo), of1 || of2)
}

 fn u256mod128(lhs: (u128, u128), rhs: u128) -> u128 {
    if lhs.0 < rhs {
        // The result fits in 128 bits.
        div_rem1(lhs, rhs)
    } else {
        div_rem1((lhs.0 % rhs, lhs.1), rhs)
    }
}

 fn u256prod(lhs: u128, rhs: u128) -> (u128, u128) {
    // hi,low
    let ((x1, x0), (y1, y0)) = (split_to_u128(lhs), split_to_u128(rhs));

    let z2 = x1 * y1;
    let (c0, z0) = split_to_u128(x0 * y0);
    let (c1, z1) = split_to_u128(x1 * y0 + c0);
    let z2 = z2 + c1;
    let (c1, z1) = split_to_u128(x0 * y1 + z1);

    (z2 + c1, z0 | z1 << 64) // hi,lo returned
}

 fn u256sqr(x: u128) -> (u128, u128) {
    // hi,low
    let (x1, x0) = split_to_u128(x);

    let z2 = x1 * x1;
    let m = x1 * x0;
    let (c0, z0) = split_to_u128(x0 * x0);
    let (c1, z1) = split_to_u128(m + c0);
    let z2 = z2 + c1;
    let (c1, z1) = split_to_u128(m + z1);
    (z2 + c1, z0 | z1 << 64)
}

fn leading_zeros(pair: (u128, u128)) -> u32 {
    //hi.lo
    if pair.0 == 0 {
        pair.1.leading_zeros() + 64
    } else {
        pair.0.leading_zeros()
    }
}

fn mut_shl(pair: &mut (u128, u128), shift: u32) {
    match shift {
        0 => {}
        s if s >= 128 => {
            pair.0 = pair.1 << (s - 128);
            pair.1 = 0;
        }
        s => {
            pair.0 <<= s;
            pair.0 |= pair.1 >> (128 - s);
            pair.1 <<= s;
        }
    }
}
// which is all that is necessary until a faster optimization is constructed.
// Any library that uses this function as a general euclidean function is to be considered critically broken.
 fn div_rem1(pair: (u128, u128), other: u128) -> u128 {
    //takes hi,lo pair

    const RADIX: u128 = 0x10000000000000000;

    // Normalize the divisor.
    let s = other.leading_zeros();
    let d = other << s; // numerator, denominator
    let mut zqk = pair;
    mut_shl(&mut zqk, s);
    let p = zqk;
    let (d1, d0) = split_to_u128(d);
    let (n1, n0) = split_to_u128(p.1); // split lower part of dividend

    let (mut q1, mut rhat) = (p.0/d1,p.0%d1);//p.0.euclidean_div(&d1);

    while q1 >= RADIX || q1 * d0 > RADIX * rhat + n1 {
        q1 -= 1;
        rhat += d1;
        if rhat >= RADIX {
            break;
        }
    }

    let r21 =
        p.0.wrapping_mul(RADIX)
            .wrapping_add(n1)
            .wrapping_sub(q1.wrapping_mul(d));

    // Compute the second quotient digit q0.
    let (mut q0, mut rhat) = (r21/d1,r21%d1);//r21.euclidean_div(&d1);

    // q0 has at most error 2. No more than 2 iterations.
    while q0 >= RADIX || q0 * d0 > RADIX * rhat + n0 {
        q0 -= 1;
        rhat += d1;
        if rhat >= RADIX {
            break;
        }
    }

    let r = (r21
        .wrapping_mul(RADIX)
        .wrapping_add(n0)
        .wrapping_sub(q0.wrapping_mul(d)))
        >> s;
    r
}


impl NTCore for u128{

   fn mont_add(&self, y: Self) ->  Self{
     self.wrapping_add(y)
  }

  fn mont_sub(&self, y: Self, n: Self) -> Self{
     if y > *self{
       return n.wrapping_sub(y.wrapping_sub(*self));
     }
     self.wrapping_sub(y)
  }
  
  
  fn n_identity(&self) -> Self{
     (u128::MAX% *self) + 1
  }
  
  fn inv_2(&self) -> Self{
     let n = *self;
     // inverse of odd n in  2^128
    let mut est = INV_8[((n >> 1) & 0x7F) as usize] as u128;
    est = 2u128.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
    est = 2u128.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
    est = 2u128.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
    est = 2u128.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
    est = 2u128.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
    est
  }
  
  fn inv_2_neg(&self) -> Self{
     let inv = self.inv_2();
     inv.wrapping_neg()
  }
  
  fn to_mont(&self,n: u128) -> Self{
      u256mod128((*self, 0), n)
  }
  
  
  fn mont_prod(&self, y: Self, n: Self, npi: Self) ->  Self{
     let (phi, plo) = u256prod(*self, y);

    let tm = plo.wrapping_mul(npi);

    let (t, overflow) = overflowing_add(u256prod(n, tm), (phi, plo));

    let t = t.0;

    if overflow {
        t + n.wrapping_neg()
     } else if t >= n {
        t - n
     } else {
        t
     }
     
    }

   fn mont_sqr(&self, n: Self, npi: Self) ->  Self{
     let (phi, plo) = u256sqr(*self);

    let tm = plo.wrapping_mul(npi);

    let (t, overflow) = overflowing_add(u256prod(n, tm), (phi, plo));

    let t = t.0;

    if overflow {
        t + n.wrapping_neg()
     } else if t >= n {
        t - n
     } else {
        t
     }
     
    }
    
    /*
  fn to_z(&self , n: Self, npi: Self) -> Self{
     let tm = self.wrapping_mul(npi);
    
     let (t, flag) = overflowing_add((0, tm), u256prod(tm, n));

     let t = t.0;
    
     if flag{
        t + n.wrapping_neg()
     } else if t >= n{
        t - n
     }
     else{
       t
     }
  }
  
  */
  fn to_z(&self, n: Self, npi: Self) ->  Self{
     //let (phi, plo) = u256prod(*self, y);

    let tm = self.wrapping_mul(npi);

    let (t, overflow) = overflowing_add(u256prod(n, tm), (0, *self));

    let t = t.0;

    if overflow {
        t + n.wrapping_neg()
     } else if t >= n {
        t - n
     } else {
        t
     }
   }  
     
  
    
fn mont_pow(&self, mut one: Self, mut p: Self, n: Self, npi: Self) -> Self {
     
        let mut base = *self;

       while p > 1 {
           if p & 1 == 0 {
               base = base.mont_sqr(n,npi);
                p >>= 1;
            } else {
                 one = base.mont_prod(one,n,npi);
              base = base.mont_sqr(n,npi);
               p = (p - 1) >> 1
        }
    }
      base.mont_prod(one, n, npi)
 }

 fn odd_fermat(&self, base: Self) -> bool{
     let one = self.n_identity();
     let npi = self.inv_2_neg();
     let p = *self -1;
     let b = base.to_mont(*self);
     if b.mont_pow(one,p,*self,npi) == one{
        return true;
     }
     false
 }
 
 fn p_sq_fermat(&self, base: Self) -> bool{
       let n = *self * *self;
       let one = n.n_identity();
       let npi = n.inv_2_neg();
       let p = *self -1;
       let b = base.to_mont(n);
       
       if b.mont_pow(one,p,n,npi) == one{
          return true;
       }
       false
   }
 
 fn odd_exp_residue(&self, p: Self, n: Self) -> Self{
    let base = self.to_mont(n);
    let npi = n.inv_2_neg();
    let one = n.n_identity();
    
    base.mont_pow(one,p,n,npi).to_z(n,npi)
 }
 
 fn even_exp_residue(&self, p: Self, twofactor: Self) -> Self{
       let n = (1<<twofactor)-1;
       let mut z = 1;
       let mut base = *self;
       let mut pow = p;
       
       while pow > 1 {
           if pow & 1 == 0 {
               base = base.wrapping_mul(base)&n;
                pow >>= 1;
            } else {
                 z = base.wrapping_mul(z)&n;
              base = base.wrapping_mul(base)&n;
               pow = (pow - 1) >> 1
        }
    }
      base.wrapping_mul(z)&n
 }
 
 fn expr(&self, p: Self, n: Self) -> Self{
    if n & 1 == 0 {
        let k = n.trailing_zeros() as u64;
        let s = n >> k;

        let reducer = (1 << k) - 1; // A shorthand for arithmetic over Z[2k]

        let k_rem = self.even_exp_residue(p,reducer);//even_pow_64(x, y, reducer); //x.wrapping_pow(y as u32)&reducer;

        let s_rem = self.odd_exp_residue(p,s);
        
        let s_inv = s.inv_2()&reducer;
/*
        let mut s_inv = s;
        
        for _ in 0..10 {
            // Multiplicative inverse over Z[2k]
            s_inv = 2u64.wrapping_sub(s_inv.wrapping_mul(s)).wrapping_mul(s_inv) & reducer;
        }
*/
        let y = k_rem.wrapping_sub(s_rem).wrapping_mul(s_inv) & reducer;

        s_rem + s * y
    } else {
        self.odd_exp_residue(p,n) //odd_pow64(x, y, n)
    }
 }
 // Full fermat test
 fn fermat(&self,a: Self) -> bool{
    if a.expr(*self-1,*self) == 1{
       return true
    }
    return false
 }

 fn sprp(&self, base: Self) -> bool {
    let p_minus = *self - 1;
    let zeroes = p_minus.trailing_zeros();
    let d = p_minus >> zeroes;

    let npi = self.inv_2_neg();
    let one = self.n_identity();
    let b = base.to_mont(*self);
    let mut x = b.mont_pow(one,d,*self,npi);
    let oneinv = self.mont_sub(one,*self).mont_prod(one,*self,npi);
    if x == one || x == oneinv {
        return true;
    }
    for _ in 1..zeroes {
        x = x.mont_sqr(*self,npi);

        if x == oneinv {
            return true;
        }
    }
    false
}

fn mul_ord(&self,n: Self) -> Self{
    let one = n.n_identity();
    let mut init = *self;
    let npi = n.inv_2_neg();
    
    for i in 0..n{
      init = init.mont_prod(*self,n,npi);
      if init == one{
        return i
      }
    }
     0u128
 }
 
 fn bounded_ord(&self,n: Self, sup: Self) -> Self{
    let one = n.n_identity();
    let mut init = *self;
    let npi = n.inv_2_neg();
    let x = sup/ *self;
    for i in 0..x{
      init = init.mont_prod(*self,n,npi);
      if init == one{
        return i
      }
    }
     0u128
 }

  
}


#[test]
fn prp(){
  assert!(2047u64.sprp(2));
  assert!(341u64.odd_fermat(2));
}



