use crate::natural::signed::*;
use crate::natural::{
    factor::{factorize, Factorization},
    montcore::NTCore,
    rand::{comp_gen_k, rand, prime_gen_k},
};
use crate::primes::{PRIME_INV_128, SMALL_PRIMES};
use crate::{Natural, Pseudoprime};
use machine_prime::PRIME_TABLE;

impl Natural for u64 {
    const ONE: u64 = 1;
    const ZERO: u64 = 0;
    const BYTE_LENGTH: usize = 8;

    fn to_u64(&self) -> u64 {
        *self
    }

    fn is_even(&self) -> bool {
        *self & 1 == 0
    }

    fn is_multiple_of(&self, factor: u64) -> bool {
        *self % factor == 0
    }

    fn inc_by(&mut self, inc: u64) {
        *self += inc;
    }

    fn is_bounded_by(&self, inf: Self, sup: Self) -> bool {
        if *self > inf && *self < sup {
            return true;
        }
        false
    }

    fn min_max(&self, otra: Self) -> (Self, Self) {
        if otra < *self {
            return (otra, *self);
        }
        (*self, otra)
    }

    fn wrapping_sub(&self, otra: Self) -> Self {
        u64::wrapping_sub(*self, otra)
    }

    fn byte_length() -> usize {
        8usize
    }

    fn msb(&self) -> usize {
        64usize - self.leading_zeros() as usize
    }

    fn hash_shift(&self, shift: usize, multiplier: u32) -> usize {
        ((*self as u32).wrapping_mul(multiplier) >> shift) as usize
    }

    fn is_semiprime_k(&self, a: usize) -> bool {
        let fctr = a as u64;
        let sq = (*self - 1) / fctr;
        let k = (sq as f64).sqrt() as u64;

        if ((k * k + k) * fctr + k + 1) == *self {
            return true;
        }

        return false;
    }

    fn even_complement(&self, k: Self) -> Self {
        ((*self - 1) / 2) * k + 1
    }

    fn semi_k_complement(&self, k: usize) -> Self {
        (k as u64) * (*self - 1) + 1
    }

    fn overflowing_mul(&self, otra: Self) -> (Self, bool) {
        u64::overflowing_mul(*self, otra)
    }

    fn euclidean(&self, otra: Self) -> (Self, Self) {
        (*self / otra, *self % otra)
    }

    // FIXME allow arbitrary S value
    fn trial_bound(&self, s: usize) -> bool {
        if *self & 1 == 0 {
            return false;
        }

        if s > 128 {
            panic!("Function not defined for prime bound > 128")
        }
        
        for i in 0..(s-1){
          // FIXME possibly faster indexing by incrementing by 2
           let prod = self.wrapping_mul(PRIME_TABLE[2*i]);
           if prod <= PRIME_TABLE[2*i+1]{
              return prod==1;
           }
        }
        return true;
    }

    fn small_factor(&self) -> Vec<u64> {
        let mut veccy = vec![];

        for i in SMALL_PRIMES.iter() {
            if *self % (*i as u64) == 0 {
                veccy.push(*i);
            }
        }
        veccy
    }
    // FIXME Replace with PRIMETABLE for 
    fn div_vector(&self, f: &[u64]) -> bool {
        for i in f.iter() {
            if *self % *i == 0 {
                return true;
            }
        }
        return false;
    }

    fn from_bytes(x: &[u8]) -> Self {
        Self::from_le_bytes([x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]])
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn successor(&mut self) {
        *self += 1;
    }

    fn comp_gen_k(k: usize) -> Option<Self> {
        if k == 0 || k > 64{
           return None;
        }
        let hi_bit = (1<<(k-1));
        loop{
          let p = (rand()&(u64::MAX>>(64-k)))|hi_bit;
          if !p.is_prime(){
             return Some(p);
          }
        }
    }

    fn prime_gen_k(k: usize) -> Option<Self> {
        if k == 0 || k > 64{
           return None;
        }
        // Sets the top bit and last bit to restrict it to an odd integer of k-bit length
        let hi_lowbits = (1<<(k-1))|1;
        loop{
          let p = (rand()&(u64::MAX>>(64-k)))|hi_lowbits;
          if p.is_prime(){
             return Some(p);
          }
        }
    }

    fn gen_k(k: usize) -> Option<Self> {
       if k == 0 || k > 64{
          return None;
       } 
       let hibit = 1<<(k-1);
       let lowbits = rand()&(u64::MAX>>(64-k));
       Some(lowbits|hibit)
    }

    fn gcd(&self, other: Self) -> Self {
        let mut a = *self;
        let mut b = other;
        // Handle the case where a = 0 outside this function, most applications should have no use for it
        debug_assert!(a !=0);
        if b == 0 {
            return a;
           }
        
        let min_two_factor = (a | b).trailing_zeros();
        b >>= b.trailing_zeros();
        while (a != 0) {
            a >>= a.trailing_zeros();
            if b > a { (a,b) = (b,a); }
            a -= b;
        }
        b << min_two_factor
    }

    fn lcm(&self, otra: Self) -> Option<Self> {
        self.checked_mul(otra/self.gcd(otra))
    }

    fn product_residue(&self, other: &Self, n: &Self) -> Self {
        if n == &0 {
            return self.wrapping_mul(*other);
        }
        ((*self as u128 * *other as u128) % *n as u128) as Self
    }

    fn extended_gcd(&self, other: Self) -> (Self, Self, Self) {
        let mut gcd: Self = *self;
        let mut new_r: Self = other;
        let mut bezout_1: Self = 1;
        let mut new_s: Self = 0;
        let mut bezout_2 : Self = 0;
        let mut new_t = 1;
        
        while new_r != 0 {
            let quotient = gcd / new_r;
            let mut temp_r: Self = new_r;
            let prod = quotient.wrapping_mul(temp_r);

            new_r = gcd - prod;
            gcd = temp_r;

            let mut temp = new_s;
            new_s = bezout_1.wrapping_sub(temp.wrapping_mul(quotient));
            bezout_1 = temp;
            
            temp = new_t;
            new_t =bezout_2.wrapping_sub(temp.wrapping_mul(quotient));
            bezout_2 = temp;

        }
        if bezout_1 > 1u64<<63{
          bezout_1=other.wrapping_add(bezout_1);
        }
        if bezout_2 > 1u64<<63{
          bezout_2=self.wrapping_add(bezout_2);
        }
        (gcd,bezout_1,bezout_2)
    }
    // Used for coprime CRT as well as mul-inverse
    fn gcd_bz(&self, ring: Self) -> (Self,Self) {
        let mut gcd: Self = *self;
        let mut new_r: Self = ring;
        let mut bezout_1: Self = 1;
        let mut new_s: Self = 0;

        while new_r != 0 {
            let quotient = gcd / new_r;
            let mut temp_r: Self = new_r;
            let prod = quotient.wrapping_mul(temp_r);

            new_r = gcd - prod;
            gcd = temp_r;

            let mut temp = new_s;
            new_s = bezout_1.wrapping_sub(temp.wrapping_mul(quotient));
            bezout_1 = temp;
        }
        if bezout_1 > 1u64<<63{
          bezout_1=ring.wrapping_add(bezout_1);
        }
        (gcd,bezout_1)        
    }
    
    fn mul_inverse(&self, ring: Self) -> Option<Self>{
       let (g,inv) = self.gcd_bz(ring);
       if g != 1{
          return None
       }
       Some(inv)
    }

    fn jacobi(&self, other: Self) -> i8 {
        let mut n = *self;
        let mut p = other;
        let mut t = 1i8;
        n %= p;

        while n != 0 {
            let zeros = n.trailing_zeros();
            n >>= zeros;

            if (p % 8 == 3 || p % 8 == 5) && (zeros % 2 == 1) {
                t = -t
            }

            std::mem::swap(&mut n, &mut p);
            if n % 4 == 3 && p % 4 == 3 {
                t = -t;
            }
            n %= p;
        }

        if p == 1 {
            t
        } else {
            0
        }
    }

    fn fast_classify(&self, a: Self) -> Pseudoprime {
        if self & 1 == 0 {
            if Natural::fermat(self, a) {
                return Pseudoprime::Strong;
            } else {
                return Pseudoprime::Composite;
            }
        }
        let inv = machine_prime::mul_inv2(*self);
        let pminus = self.wrapping_sub(1);
        let tzc = pminus.trailing_zeros();
        let d = pminus>>tzc;
        let one = machine_prime::one_mont(*self);
        let oneinv = self.wrapping_sub(one);
        let b = machine_prime::to_mont(a, *self);

        let mut mbase = machine_prime::mont_pow(b, one, d, inv,*self);

        if mbase == one || mbase == oneinv {
            return Pseudoprime::Strong;
        }

        for i in 1..tzc {
            mbase = machine_prime::mont_prod(mbase, mbase, inv ,*self);
            if mbase == oneinv {
                return Pseudoprime::Strong;
            }
            if i == tzc - 1 {
                let sym = a.jacobi(*self);
                if sym == 1 && mbase == one {
                    return Pseudoprime::EulerJacobi;
                }
                if mbase == one {
                    return Pseudoprime::Euler;
                }
            }
        }
        let mbase = machine_prime::mont_prod(mbase, mbase, inv,*self);

        if mbase == one {
            return Pseudoprime::Fermat;
        }
        return Pseudoprime::Composite;
    }

    fn classify(&self, a: Self) -> Pseudoprime {
        let set = self.fast_classify(a);
        if *self == 2 {
            return Pseudoprime::Prime;
        }
        if set == Pseudoprime::Strong {
            if machine_prime::is_prime_wc(*self) {
                return Pseudoprime::Prime;
            }
        }
        return set;
    }

    fn fermat(&self, a: Self) -> bool {
        //    if *self&1 == 0{
        //   return NTCore::fermat(self,a)
        //  }
        NTCore::fermat(self, a)
    }

    // Odd-only semifermat, this is not efficient
    // Calculating the inverses takes too long
    fn semi_fermat(&self, x: Self, y: Self) -> bool {
        let prod = x as u128 * y as u128;
        let (_, xinv, yinv) = x.extended_gcd(y);

        let rhs = self.exp_residue(x, x);
        let lhs = self.exp_residue(y, y);
        //   rhs*xinv % x
        // if prod > 1u128<<64 {
        //    return
        // }
        //return true;
        unimplemented!()
    }

    fn sqr_fermat(&self, base: Self) -> bool {
        if *self > 1u64 << 32 {
            return (*self as u128).p_sq_fermat(base as u128);
        } else {
            return self.p_sq_fermat(base);
        }
    }

    fn exp_residue(&self, p: Self, n: Self) -> Self {
        self.expr(p, n)
    }

    fn exp_unit(&self, p: Self, n: Self) -> bool {
        self.exp_one(p, n)
    }
    // counts the number of fermat and strong fermat pseudoprimes to N, minus the trivial case of 1
    fn pseudoprime_count(&self) -> (Self, Self) {
        // decompose x-1 into d*2^k
        let decomp = |x: Self| -> (u32, Self) {
            let xminus = x - 1;
            let twofactor = xminus.trailing_zeros();
            (twofactor, xminus >> twofactor)
        };

        if self.is_prime() {
            return (0, 0);
        }

        let fctr = self.factor().unwrap();

        let xminus = (*self - 1);

        let mut fermatprod = 1;

        for i in fctr.factors.iter() {
            // when x \in 2Z then skip 2
            if *i == 2 {
                continue;
            }
            fermatprod *= xminus.gcd(*i - 1);
        }
        // Omit 1 since we aren't interested in it
        fermatprod -= 1;
        // If self is even then the number of pseudoprimes is identical
        if *self & 1 == 0 {
            return (fermatprod, fermatprod);
        }

        let (xe, xd) = decomp(*self);

        let mut mine = 128;
        let m = fctr.factors.len() as u32;

        let mut strongprod = 1;

        for p in fctr.factors.iter() {
            let (pe, pd) = decomp(*p);
            if pe < mine {
                mine = pe;
            }
            strongprod *= xd.gcd(pd);
        }

        let denom = 2u64.pow(m) - 1;
        let numer = 2u64.pow(m * mine) - 1;
        let multiplicand = (numer / denom) + 1;
         
        (fermatprod, strongprod * multiplicand - 1)
    }

    fn euler_jacobi(&self, a: Self) -> bool {
        let r = a.jacobi(*self);
        if r == -1 {
            return a.exp_residue((*self - 1) / 2, *self) == *self - 1;
        }
        a.exp_residue((*self - 1) / 2, *self) == r as u64
    }

    // Performs particularly bad for base-2 for some reason
    fn sprp(&self, a: Self) -> bool {
        if self & 1 == 0 {
            return NTCore::fermat(self, a);
        }
        NTCore::sprp(self, a)
    }
    
    fn special_sf(&self, p: Self, n: Self) -> bool{
       NTCore::special_sf(self,p,n)
    }

    fn is_prime(&self) -> bool {
        machine_prime::is_prime(*self)
    }

    fn euler_p(&self) -> bool {
        let residue = *self & 7;
        let mut param = 0;

        if residue == 1 {
            param = 1;
        }

        let ap = 2.exp_residue((*self - 1) >> (1 + param), *self);
        if ap == 1 {
            return residue == 1 || residue == 7;
        } else if ap == *self - 1 {
            return residue == 1 || residue == 3 || residue == 5;
        }
        return false;
    }

    fn is_perfect_power(&self) -> bool {
        if self.max_exp().1 > 1 {
            return true;
        }
        false
    }

    fn is_power_of(&self, x: usize) -> bool {
        if *self == x as u64 {
            return true;
        }
        if x == 1 || x == 0 {
            return false;
        }
        if *self % (x as u64) != 0 {
            return false;
        }
        let mut val = x as u64;

        for _ in 0..64 {
            let (val_interim, flag) = val.overflowing_mul(x as u64);

            val = val_interim;

            if flag {
                // if overflowed then not perfect power
                return false;
            }
            if val == *self as u64 {
                return true;
            }
        }
        return false;
    }
    // FIXME replace with the new isqrt
    fn isqrt(&self) -> Self { 
        let mut est = (*self as f64).sqrt() as u64 + 1;

        loop {
            let s = est;
            let t = s + *self / s;
            est = t >> 1;
            if est >= s {
                return s;
            }
        }
    }

    fn nth_root(&self, n: usize) -> Self {
        if n > 63 {
            return 1;
        }

        if n == 1 {
            return *self;
        }

        if n == 0 {
            panic!("No integer is a zeroth factor ")
        }

        let mut est = (*self as f64).powf((n as f64).recip()) as Self + 1;

        loop {
            let s = est;
            let t = (n as u64 - 1) * s + *self / s.pow(n as u32 - 1);
            est = t / (n as u64);
            if est >= s {
                return s;
            }
        }
    }

    fn max_exp(&self) -> (Self, Self) {
        for i in 1..64 {
            let p = 64 - i;
            let base = self.nth_root(p);
            if base.pow(p as u32) == *self {
                return (base, p as u64);
            }
        }
        return (*self, 1);
    }

    fn is_square(&self) -> bool {
        let sq = self.isqrt();
        sq * sq == *self
    }

    fn factor(&self) -> Option<Factorization<Self>> {
        Some(factorize(*self))
    }

    fn ord(&self, n: Self) -> Option<Self> {
        let ord_2 = |a: u64, p: u64| -> u64 {
            let modulo = (1u64 << p) - 1;
            let mut b = a & modulo;

            if b == 1 {
                return 1;
            }
            for i in 1..p {
                b = b.wrapping_mul(b) & modulo;
                if b == 1 {
                    return 1 << i;
                }
            }
            return p;
        };

        // Given ord(a,p)  calculate ord(a,p^n)
        let pp_ord = |a: u64, b: u64, p: u64, e: u32| -> u64 {
            for i in 0..e + 1 {
                if a.exp_residue(b * p.pow(i), p.pow(e)) == 1 {
                    return b * p.pow(i);
                }
            }
            return b * p.pow(e);
        };

        let p_ord = |a: u64, p: u64| -> u64 {
            let fctr = (p - 1).factor().unwrap();

            let mut m = p - 1;
            for i in fctr.pair_iter() {
                for _ in 0..*i.1 { // FIXME Replace with exp_unit ?
                    if a.exp_residue(m / *i.0, p) == 1 {
                        m = m / *i.0;
                    } else {
                        break;
                    }
                }
            }
            m
        };

        if self.gcd(n) != 1 {
            return None;
        }
        let fctr = n.factor().unwrap();
        let mut fullord = 1u64;
        for i in fctr.pair_iter() {
            let mut ord: Self;
            if *i.0 == 2 {
                ord = ord_2(*self, *i.1 as u64);
            } else { // calculate the initial prime-order
                ord = p_ord(*self, *i.0);
                if *i.1 > 1 { // if n contains a perfect power then calculate the power of the prime-order
                    ord = pp_ord(*self, ord, *i.0, *i.1 as u32);
                }
            }// Ord_a(n) is always less than n so this never overflows
            fullord = fullord.lcm(ord).unwrap();
        }
        Some(fullord)
    }

    // Multiplicative order for some prime P and some A guaranteed to be coprime to P
    // P= 2 is not supported. This is to be handled separately
    fn p_ord(&self, a: Self) -> Self {
        let mut pminus = *self - 1;
        let fctr = pminus.factor().unwrap();
        let one = self.n_identity();
        let base = a.to_mont(*self);
        let inv = self.inv_2();
        for (idx, el) in fctr.factors.iter().enumerate() {
            for _ in 0..fctr.powers[idx] {
            // FIXME Replace with exp_one ?
                if base.mont_pow(one, pminus / *el, inv,*self) == one {
                    pminus = pminus / *el;
                } else {
                    break;
                }
            }
        }
        pminus
    }
    // Returns order and the signature (the largest factor of 2 dividing the order) of a prime
    fn signature(&self, a: Self) -> Option<(Self, u32)> {
        let ord = self.p_ord(a);
        Some((ord, ord.trailing_zeros()))
    }
    
    // Signature set to a set of bases
    fn signature_v(&self, base: &[Self]) -> Option<(Self, Vec<u32>)> {
        for i in base {
            if self.gcd(*i) > 1 {
                return None;
            }
        }
        let pminus = *self - 1;
        let fctr = pminus.factor().unwrap();
        let one = self.n_identity();
        let inv = self.inv_2();
        let mut totalord = 1;
        let mut sig = vec![];
        for (aidx, a) in base.iter().enumerate() {
            let base = a.to_mont(*self);
            let mut ord = pminus;
            for (idx, el) in fctr.factors.iter().enumerate() {
                for _ in 0..fctr.powers[idx] {
                    if base.mont_pow(one, ord / *el, inv,*self) == one {
                        ord = ord / *el;
                    } else {
                        break;
                    }
                }
            }
            // if aidx == 0{
            //   sig = ord.trailing_zeros();
            // }
            // else{
            //  if ord.trailing_zeros() != sig{
            //    return None;
            // }
            sig.push(ord.trailing_zeros());
            totalord = totalord.lcm(ord).unwrap();
        } // base loop
        Some((totalord, sig))
    }
}
