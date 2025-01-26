use crate::fermat::NTCore;


#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Pseudoprime {
    None,
    Fermat,
    Euler,
    Strong,
}


impl NTCore for u64 {

 fn mont_add(&self,x: Self, n: Self) ->  Self{
      let res = self.wrapping_add(x);
     
     if res > n{
        return res.wrapping_sub(n)
     }
     res
  }
  
  fn mont_sub(&self, y: Self, n: Self) -> Self{
     machine_prime::mont_sub(*self,y,n)
  }
  
  
  fn n_identity(&self) -> Self{
     machine_prime::one_mont(*self)
  }
  
  fn two_identity(&self,one: Self) -> Self{
     machine_prime::two_mont(one,*self)
  }
  
  fn one_inverse_n(&self,one: Self, npi: Self ) -> Self{
     self.mont_sub(one,*self).mont_prod(one,*self,npi)
  }
  
  fn to_mont(&self, n: Self) -> Self{
     machine_prime::to_mont(*self,n)
  }

  fn inv_2(&self) -> Self{
     machine_prime::mul_inv2(*self)
  }
  
  fn inv_2_neg(&self) -> Self{
     let inv = self.inv_2();
     inv.wrapping_neg()
  }
  
  fn mont_prod(&self, y: Self, n: Self, npi: Self) -> Self{
    machine_prime::mont_prod(*self,y,n,npi)
  }
  
  fn mont_sqr(&self, n: Self, npi: Self) -> Self{
     self.mont_prod(*self,n,npi)
  }
  
  fn to_z(&self, n: Self, npi: Self) ->  Self{
     
    let lo = self.wrapping_mul(npi);
    let lo = ((lo as u128).wrapping_mul(n as u128)>>64) as u64;

    lo.wrapping_neg().wrapping_add(n)
  }

 fn mont_pow(&self, mut one: Self, mut p: Self, n: Self, npi: Self) -> Self {
    machine_prime::mont_pow(*self,one,p,n,npi)
}

 fn odd_exp_residue(&self, p: Self, n: Self) -> Self{
    let one = n.n_identity();
    let base = self.to_mont(n);
    let npi = n.inv_2();
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

        let k_rem = self.even_exp_residue(p,reducer);

        let s_rem = self.odd_exp_residue(p,s);
        
        let s_inv = s.inv_2()&reducer;
    
        let y = k_rem.wrapping_sub(s_rem).wrapping_mul(s_inv) & reducer;

        s_rem + s * y
    } else {
        self.odd_exp_residue(p,n)
    }
 }

 fn odd_fermat(&self, base: Self) -> bool{
     let one = self.n_identity();
     let npi = self.inv_2();
     let p = self.wrapping_sub(1);
     let b = base.to_mont(*self);
     if b.mont_pow(one,p,*self,npi)==one{
        return true;
     }
     false
 }
 // FIXME Test that this is actually correct
 fn p_sq_fermat(&self, base: Self) -> bool{
       let n = self.wrapping_mul(*self);
       let one = n.n_identity();
       let npi = n.inv_2();
       let p = self.wrapping_sub(1);
       let b = base.to_mont(*self);
       
       if b.mont_pow(one,p,n,npi) == one{
          return true;
       }
       false
   }
   
    // Full fermat test
 fn fermat(&self,a: Self) -> bool{
     if *self&1 == 1{
        return self.odd_fermat(a)
     }
     // FIXME Look to see if you can have an Even-only fermat test that is faster
    if a.expr(self.wrapping_sub(1),*self) == 1{
       return true;
    }
    return false
 }
 
// FIXME - Prove this to have no errors for 2Z except powers of two
 fn sprp(&self, base: Self) -> bool {
 
    let p_minus = self.wrapping_sub(1);
    let twofactor = p_minus.trailing_zeros();
    let mut d = p_minus >> twofactor;
   
    let npi = self.inv_2();
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

fn mont_sprp(&self, b: Self,d: Self, twofactor: u32, one: Self, oneinv: Self, npi: Self) -> bool{
    let mut x = b.mont_pow(one,d,*self,npi);
    if x == one || x == oneinv {
        return true;
    }
    for _ in 1..twofactor {
        x = x.mont_sqr(*self,npi);

        if x == oneinv {
            return true;
        }
    }
    false
}

// Simple Multiplicative order

 fn mul_ord(&self,n: Self) -> Self{
    let one = n.n_identity();
    let mut init = *self;
    let npi = n.inv_2();
    
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
    let npi = n.inv_2();
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

 



impl NTCore for u128{

 fn mont_add(&self,x: Self, n: Self) ->  Self{
      let res = self.wrapping_add(x);
     
     if res > n{
        return res.wrapping_sub(n)
     }
     res
  }

  fn mont_sub(&self, y: Self, n: Self) -> Self{
     machine_prime::mont_sub_128(*self,y,n)
  }
  
  
  fn n_identity(&self) -> Self{
     machine_prime::one_mont_128(*self)
  }
  
  fn two_identity(&self,one: Self) -> Self{
     machine_prime::two_mont_128(one,*self)
  }
  
  fn one_inverse_n(&self,one: Self, npi: Self ) -> Self{
     self.mont_sub(one,*self).mont_prod(one,*self,npi)
  }
  
  fn inv_2(&self) -> Self{
     machine_prime::mul_inv2_128(*self)
  }
  
  fn inv_2_neg(&self) -> Self{
     let inv = self.inv_2();
     inv.wrapping_neg()
  }
  
  fn to_mont(&self,n: Self) -> Self{
     machine_prime::to_mont_128(*self,n)
  }
  
  
  fn mont_prod(&self, y: Self, n: Self, npi: Self) ->  Self{
     machine_prime::mont_prod_128(*self,y,n,npi)
    }

   fn mont_sqr(&self, n: Self, npi: Self) ->  Self{
     machine_prime::mont_sqr_128(*self,n,npi)
    }
    
  fn to_z(&self, n: Self, npi: Self) ->  Self{
    let lo = self.wrapping_mul(npi);
    let lo = machine_prime::u256prod_lo(lo,n);

    lo.wrapping_neg().wrapping_add(n)
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
     let p = self.wrapping_sub(1);
     let b = base.to_mont(*self);
     if b.mont_pow(one,p,*self,npi) == one{
        return true;
     }
     false
 }
 
 fn p_sq_fermat(&self, base: Self) -> bool{
       let n = self.wrapping_mul(*self);
       let one = n.n_identity();
       let npi = n.inv_2_neg();
       let p = self.wrapping_sub(1);
       let b = base.to_mont(*self);
       
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

        let y = k_rem.wrapping_sub(s_rem).wrapping_mul(s_inv) & reducer;

        s_rem + s * y
    } else {
        self.odd_exp_residue(p,n) //odd_pow64(x, y, n)
    }
 }
 // Full fermat test
 fn fermat(&self,a: Self) -> bool{
    if *self&1==1{
       return self.odd_fermat(a)
    }
    if a.expr(self.wrapping_sub(1),*self) == 1{
       return true
    }
    return false
 }

 fn sprp(&self, base: Self) -> bool {
    let p_minus = self.wrapping_sub(1);
    let zeroes = p_minus.trailing_zeros();
    let d = p_minus >> zeroes;

    let npi = self.inv_2();
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

fn mont_sprp(&self, b: Self,d: Self, twofactor: u32, one: Self, oneinv: Self, npi: Self) -> bool{
    let mut x = b.mont_pow(one,d,*self,npi);
    if x == one || x == oneinv {
        return true;
    }
    for _ in 1..twofactor {
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
    let npi = n.inv_2();
    
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
    let npi = n.inv_2();
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



