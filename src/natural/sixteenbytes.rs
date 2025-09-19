use crate::natural::factor::{factorize_128, Factorization};
use crate::natural::montcore::NTCore;
use crate::natural::signed::*;
use crate::primes::{PRIME_INV_128, SMALL_PRIMES};
use crate::{Natural, Pseudoprime};
use machine_prime::{is_prime_128, PRIME_INV_64};

/*
    128-bit FInteger check
*/

impl Natural for u128 {
    const ONE: u128 = 1;
    const ZERO: u128 = 0;
    const BYTE_LENGTH: usize = 16;

    fn is_even(&self) -> bool {
        *self & 1 == 0
    }

    fn is_multiple_of(&self, factor: u64) -> bool {
        *self % (factor as u128) == 0
    }

    fn inc_by(&mut self, inc: u64) {
        *self += inc as u128;
    }

    fn to_u64(&self) -> u64 {
        if *self > 1u128 << 64 {
            panic!("to_u64 function panicked due to value being greater than 2^64")
        }
        *self as u64
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
        u128::wrapping_sub(*self, otra)
    }

    fn byte_length() -> usize {
        16usize
    }

    fn msb(&self) -> usize {
        128usize - self.leading_zeros() as usize
    }

    fn hash_shift(&self, shift: usize, multiplier: u32) -> usize {
        ((*self as u32).wrapping_mul(multiplier) >> shift) as usize
    }

    fn is_semiprime_k(&self, a: usize) -> bool {
        let fctr = a as u128;
        let sq = (*self - 1) / fctr;
        let k = sq.isqrt();

        if ((k * k + k) * fctr + k + 1) == *self {
            return true;
        }

        return false;
    }

    fn even_complement(&self, k: Self) -> Self {
        ((*self - 1) / 2) * k + 1
    }

    fn semi_k_complement(&self, k: usize) -> Self {
        (k as u128) * (*self - 1) + 1
    }

    fn overflowing_mul(&self, otra: Self) -> (Self, bool) {
        u128::overflowing_mul(*self, otra)
    }

    fn trial_bound(&self, s: usize) -> bool {
        unimplemented!()
    }

    fn small_factor(&self) -> Vec<u64> {
        let mut veccy = vec![];

        for i in SMALL_PRIMES.iter() {
            if *self % (*i as u128) == 0 {
                veccy.push(*i);
            }
        }
        veccy
    }

    fn div_vector(&self, f: &[u64]) -> bool {
        for i in f.iter() {
            if *self % (*i as u128) == 0 {
                return true;
            }
        }
        return false;
    }

    fn euclidean(&self, otra: Self) -> (Self, Self) {
        (*self / otra, *self % otra)
    }

    fn from_bytes(x: &[u8]) -> Self {
        Self::from_le_bytes([
            x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7], x[8], x[9], x[10], x[11], x[12], x[13],
            x[14], x[15],
        ])
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn successor(&mut self) {
        *self += 1;
    }

    fn comp_gen_k(k: usize) -> Option<Self> {
        if k > 128 {
            return None;
        }
        loop {
            let p = Self::gen_k(k).unwrap();
            if !p.is_prime() {
                return Some(p);
            }
        }
        None
    }

    fn prime_gen_k(k: usize) -> Option<Self> {
        if k > 128 {
            return None;
        }
        loop {
            let p = Self::gen_k(k).unwrap();
            if p.is_prime() {
                return Some(p);
            }
        }
        None
    }

    fn gen_k(k: usize) -> Option<Self> {
        if k < 65 {
            return u64::gen_k(k).map(|x| x as u128);
        }
        let lhs = u64::gen_k(64).unwrap();
        let rhs = u64::gen_k(64).unwrap();
        // FIXME ? Possible error for k = 128
        let hi_digit = 1u128.wrapping_shl(k as u32);
        let mask = (hi_digit - 1) | hi_digit;
        let res = (((lhs as u128) << 64) + (rhs as u128)) & mask;
        Some(res)
    }

    fn gcd(&self, other: Self) -> Self {
        let mut a = *self;
        let mut b = other;

        if b == 0 {
            return a;
        } else if a == 0 {
            return b;
        }

        let self_two_factor = a.trailing_zeros();
        let other_two_factor = b.trailing_zeros();
        let min_two_factor = std::cmp::min(self_two_factor, other_two_factor);
        a >>= self_two_factor;
        b >>= other_two_factor;
        loop {
            if b > a {
                std::mem::swap(&mut b, &mut a);
            }
            a -= b;

            if a == 0 {
                return b << min_two_factor;
            }
            a >>= a.trailing_zeros();
        }
    }

    fn gcd_bz(&self, ring: Self) -> (Self,Self) {
        let mut gcd: Self = *self;
        let mut new_r: Self = ring;
        let mut bezout_1: Signed<Self> = Signed(true, 1);
        let mut new_s: Signed<Self> = Signed(true, 0);

        while new_r != 0 {
            let quotient = gcd / new_r;
            let mut temp_r: Self = new_r;
            let prod = quotient * temp_r;

            new_r = gcd - prod;
            gcd = temp_r;

            let mut temp = new_s;
            new_s = Signed::sub(bezout_1, Signed::prod(temp, quotient));
            bezout_1 = temp;
        }
        (gcd,Signed::residue(bezout_1, ring))
    }
    
    fn mul_inverse(&self, ring: Self) -> Option<Self>{
       let (g,inv) = self.gcd_bz(ring);
       if g != 1{
          return None
       }
       Some(inv)
    }

    fn lcm(&self, otra: Self) -> Option<Self> {
        let g = self.gcd(otra);
        let (res, flag) = self.overflowing_mul(otra / g);
        if flag {
            return None;
        }
        Some(res)
    }

    // FIXME
    fn product_residue(&self, other: &Self, n: &Self) -> Self {
        if n == &0 {
            return self.wrapping_mul(*other);
        }
        ((*self as u128 * *other as u128) % *n as u128) as Self
    }

    fn extended_gcd(&self, other: Self) -> (Self, Self, Self) {
        let mut gcd: Self = *self;
        let mut new_r: Self = other;
        let mut bezout_1: Signed<Self> = Signed(true, 1);
        let mut new_s: Signed<Self> = Signed(true, 0);
        let mut bezout_2: Signed<Self> = Signed(true, 0);
        let mut new_t: Signed<Self> = Signed(true, 1);

        while new_r != 0 {
            let quotient = gcd / new_r;
            let mut temp_r: Self = new_r;
            let prod = quotient * temp_r;

            new_r = gcd - prod;
            gcd = temp_r;

            let mut temp = new_s;
            new_s = Signed::sub(bezout_1, Signed::prod(temp, quotient));
            bezout_1 = temp;

            temp = new_t;
            new_t = Signed::sub(bezout_2, Signed::prod(temp, quotient));
            bezout_2 = temp;
        }
        (
            gcd,
            Signed::residue(bezout_1, other),
            Signed::residue(bezout_2, *self),
        )
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
        unimplemented!()
    }

    fn classify(&self, a: Self) -> Pseudoprime {
        unimplemented!()
    }

    fn fermat(&self, a: Self) -> bool {
        NTCore::fermat(self, a)
    }

    // Odd-only semifermat
    fn semi_fermat(&self, x: Self, y: Self) -> bool {
        unimplemented!()
    }

    fn special_sf(&self, p: Self, n: Self) -> bool{
       NTCore::special_sf(self,p,n)
    }
    
    fn sqr_fermat(&self, base: Self) -> bool {
        if *self > 1u128 << 32 {
            return (*self as u128).p_sq_fermat(base as u128);
        } else {
            return self.p_sq_fermat(base);
        }
    }

    fn exp_residue(&self, p: Self, n: Self) -> Self {
        NTCore::expr(self, p, n)
    }

    fn exp_unit(&self, p: Self, n: Self) -> bool {
        self.exp_one(p, n)
    }

    fn pseudoprime_count(&self) -> (Self, Self) {
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

        fermatprod -= 1;

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

        let denom = 2u128.pow(m) - 1;
        let numer = 2u128.pow(m * mine) - 1;
        let multiplicand = (numer / denom) + 1;

        (fermatprod, strongprod * multiplicand - 1)
    }

    fn euler_jacobi(&self, a: Self) -> bool {
        let r = a.jacobi(*self);
        if r == -1 {
            return a.exp_residue((*self - 1) / 2, *self) == *self - 1;
        }
        a.exp_residue((*self - 1) / 2, *self) == (Self::from(r as u64))
    }

    fn sprp(&self, a: Self) -> bool {
        if *self & 1 == 0 {
            return NTCore::fermat(self, a);
        }
        NTCore::sprp(self, a)
    }

    fn is_prime(&self) -> bool {
        machine_prime::is_prime_128(*self)
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
        if *self == x as u128 {
            return true;
        }
        if x == 1 || x == 0 {
            return false;
        }
        if *self % (x as u128) != 0 {
            return false;
        }
        let mut val = x as u128;

        for _ in 0..128 {
            let (val_interim, flag) = val.overflowing_mul(x as u128);

            val = val_interim;

            if flag {
                // if overflowed then not perfect power
                return false;
            }
            if val == *self as u128 {
                return true;
            }
        }
        return false;
    }

    fn isqrt(&self) -> Self {
        let mut est = (*self as f64).sqrt() as u128 + 1;

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
            let t = (n as u128 - 1) * s + *self / s.pow(n as u32 - 1);
            est = t / (n as u128);
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
                return (base, p as u128);
            }
        }
        return (*self, 1u128);
    }

    fn is_square(&self) -> bool {
        let sq = self.isqrt();
        sq * sq == *self
    }

    fn factor(&self) -> Option<Factorization<Self>> {
        Some(factorize_128(*self))
    }

    fn ord(&self, a: Self) -> Option<Self> {
        unimplemented!()
    }

    // Multiplicative order for P and some A guaranteed to be coprime to P
    // P= 2 is not supported. This is to be handled separately
    fn p_ord(&self, a: Self) -> Self {
        let mut pminus = *self - 1;
        let fctr = pminus.factor().unwrap();
        let one = self.n_identity();
        let base = a.to_mont(*self);
        let inv = self.inv_2();
        for (idx, el) in fctr.factors.iter().enumerate() {
            for _ in 0..fctr.powers[idx] {
                if base.mont_pow(one, pminus / *el, *self, inv) == one {
                    pminus = pminus / *el;
                } else {
                    break;
                }
            }
        }
        pminus
    }
    // Returns order and the signature (the largest factor of 2 dividing the order)
    fn signature(&self, a: Self) -> Option<(Self, u32)> {
        let ord = self.p_ord(a);
        Some((ord, ord.trailing_zeros()))
    }

    /*
       Calculate ord(a,p) with signature
       then calculate a1 and compare signature
       return none if signature does not match
    */
    fn signature_v(&self, base: &[Self]) -> Option<(Self, Vec<u32>)> {
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
                    if base.mont_pow(one, ord / *el, *self, inv) == one {
                        ord = ord / *el;
                    } else {
                        break;
                    }
                }
            }
            /*
            if aidx == 0{
              sig = ord.trailing_zeros();
            }
            else{
              if ord.trailing_zeros() != sig{
                 return None;
              }
              */
            sig.push(ord.trailing_zeros());
            totalord = totalord.lcm(ord).unwrap();
        } // base loop
        Some((totalord, sig))
    }
}
