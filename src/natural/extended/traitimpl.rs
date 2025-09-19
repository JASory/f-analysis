use crate::natural::extended::{sliceops::{div_slice, leading_idx, scale_slice},muldiv::quo_rem_slice};
use crate::natural::{factor::Factorization,signed::Signed,rand::rand};
use crate::{Natural, Pseudoprime};

/// Extended Precision Integer (Zahl)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Ord)]
pub struct Epz<const S: usize> {
    pub limbs: [u64; S],
}

const fn zero<const S: usize>() -> Epz<S> {
    Epz { limbs: [0u64; S] }
}

const fn one<const S: usize>() -> Epz<S> {
    let mut k = zero();
    k.limbs[0] = 1;
    k
}

const fn byte_length<const S: usize>() -> usize {
    S * 8usize
}

impl<const S: usize> Default for Epz<S> {
    fn default() -> Self {
        Epz { limbs: [0u64; S] }
    }
}

impl<const S: usize> std::convert::From<u64> for Epz<S> {
    fn from(k: u64) -> Self {
        let mut z = Self::ZERO;
        z.limbs[0] = k;
        z
    }
}

impl<const S: usize> std::convert::From<u128> for Epz<S> {
    fn from(k: u128) -> Self {
        let mut z = Self::ZERO;
        z.limbs[0] = k as u64;
        z.limbs[1] = (k>>64) as u64;
        z
    }
}


impl<const S: usize> std::cmp::PartialOrd for Epz<S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for (i, j) in self.limbs.iter().rev().zip(other.limbs.iter().rev()) {
            if i > j {
                return Some(std::cmp::Ordering::Greater);
            }
            if i < j {
                return Some(std::cmp::Ordering::Less);
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}

impl<const S: usize> Natural for Epz<S> {
    /// Additive Identity
    const ZERO: Self = zero::<S>();
    /// Multiplicative Identity; 1
    const ONE: Self = one::<S>();
    /// Number of bytes used in representation
    const BYTE_LENGTH: usize = byte_length::<S>();

    ///
    fn product_residue(&self, other: &Self, n: &Self) -> Self {
        self.mul_mod(*other,*n)
    }

    fn wrapping_sub(&self, otra: Self) -> Self {
        *self - otra
    }

    fn is_even(&self) -> bool {
        self.limbs[0] & 1 == 0
    }

    fn is_multiple_of(&self, factor: u64) -> bool {
        unimplemented!()
    }

    fn small_factor(&self) -> Vec<u64> {
        unimplemented!()
    }

    fn div_vector(&self, div: &[u64]) -> bool {
        unimplemented!()
    }
     // FIXME Look to remove since all integers should support incrementing by 64-bit integer
    fn inc_by(&mut self, inc: u64) {
        *self+=inc;
    }

    /// Evaluates if integer is between inf and sup exclusive
    fn is_bounded_by(&self, inf: Self, sup: Self) -> bool {
    
       if *self < inf && *self > sup{
          return true;
       }
       false
    }

    /// Returns the Minimum and Maximum of a 2-tuple
    fn min_max(&self, otra: Self) -> (Self, Self) {
        if *self > otra{
           return (otra,*self)
        }
        (*self,otra)
    }

    /// (x*multiplier / 2^shift) mod 2^32
    fn hash_shift(&self, shift: usize, multiplier: u32) -> usize {
        ((self.limbs[0] as u32).wrapping_mul(multiplier) as usize) >> shift
    }

    fn fast_classify(&self, a: Self) -> Pseudoprime {
        unimplemented!()
    }
    /// Classifies the integer into the result of
    fn classify(&self, a: Self) -> Pseudoprime {
        unimplemented!()
    }

    fn is_semiprime_k(&self, a: usize) -> bool {
        unimplemented!()
    }

    fn semi_k_complement(&self, k: usize) -> Self {
        unimplemented!()
    }

    // 4x+1 component to the number of the form (x+1)(4x+1)
    fn even_complement(&self, k: Self) -> Self {
        unimplemented!()
    }

    fn overflowing_mul(&self, otra: Self) -> (Self, bool) {
        unimplemented!()
    }

    /// Evaluates if integer is coprime to all primes under the provided bound (exclusive)
    fn trial_bound(&self, s: usize) -> bool {
        unimplemented!()
    }

    fn euclidean(&self, otra: Self) -> (Self, Self) {
       let mut quo = Self::ZERO;
       let mut rem = self.clone();
       quo_rem_slice(&mut rem.limbs.to_vec(), &otra.limbs[..], &mut quo.limbs[..]);
       (quo,rem)
    }

    fn to_u64(&self) -> u64 {
       self.limbs[0]
    }

    /// Number of bytes used in representation
    fn byte_length() -> usize {
        Self::BYTE_LENGTH
    }

    fn msb(&self) -> usize {
        match self.limbs.iter().rposition(|x| *x != 0) {
            Some(pos) => {
                return pos * 64 + (64 - self.limbs[pos].leading_zeros() as usize);
            }
            None => return 0usize,
        }
    }

    /// From Big-Endian bytes
    fn from_bytes(x: &[u8]) -> Self {
       let mut zeroed = Self::ZERO;
       
       for i in 0..S{
          zeroed.limbs[i]=u64::from_bytes(&x[i*8..(i+1)*8]);
       } 
        zeroed
    }

    /// To Little-Endian bytes
    fn to_bytes(&self) -> Vec<u8> {
       let mut res = vec![];
       
       for i in self.limbs{
          res.extend_from_slice(&mut i.to_bytes());
       }
       res
    }

    /// Randomly generate guaranteed composite of k-bit length
    fn comp_gen_k(k: usize) -> Option<Self> {
        loop {
           match Self::gen_k(k){
             Some(x) => if !x.is_prime(){return Some(x)},
             None => return None,
           }
        }
    }

    /// Randomly generate probable prime of k-bit length
    fn prime_gen_k(k: usize) -> Option<Self> {
        loop {
           match Self::gen_k(k){
             Some(x) => {if x.is_prime(){return Some(x)}},
             None => return None,
           }
        }
    }

    /// Randomly generate integer of k-bit length
    fn gen_k(k: usize) -> Option<Self> {
        if k > S*8{
           return None;
        }
        
        let mut zeroed = Self::ZERO;
        let length = k>>3;
        for i in zeroed.limbs[0..length].iter_mut(){
            *i=rand();
        }
        zeroed.limbs[length]=u64::gen_k(k&7).unwrap();
        
        Some(zeroed)
    }

    /// Successor function; X+1
    fn successor(&mut self) {
        *self+=1u64;
    }

    ///  Greatest common divisor
    fn gcd(&self, otra: Self) -> Self {
         let mut a = *self;
         let mut b = otra;
         while b != Self::ZERO {
            let t = b.clone();

            b = a%b;
            a = t;
        }
        a
    }
    
    fn gcd_bz(&self, ring: Self) -> (Self,Self){
        let mut gcd: Self = *self;
        let mut new_r: Self = ring;
        let mut bezout_1: Signed<Self> = Signed(true, Self::ONE);
        let mut new_s: Signed<Self> = Signed(true, Self::ZERO);

        while new_r != Self::ZERO {
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
    
    // FIXME handle multiplicative overflow
    fn lcm(&self, otra: Self) -> Option<Self> {
        let g = self.gcd(otra);
        if g > Self::ONE{
           return Some((otra/g)* *self);
        }
        Some(*self*otra)
    }

    ///  Finite ring gcd
    fn extended_gcd(&self, ring: Self) -> (Self, Self, Self) {
        let mut gcd: Self = *self;
        let mut new_r: Self = ring;
        let mut bezout_1: Signed<Self> = Signed(true, Self::ONE);
        let mut new_s: Signed<Self> = Signed(true, Self::ZERO);
        
        let mut bezout_2: Signed<Self> = Signed(true, Self::ONE);
        let mut new_t: Signed<Self> = Signed(true, Self::ZERO);
        
        while new_r != Self::ZERO {
            let quotient = gcd / new_r;
            let mut temp_r: Self = new_r;
            let prod = quotient * temp_r;

            new_r = gcd - prod;
            gcd = temp_r;

            let mut temp = new_s;
            new_s = Signed::sub(bezout_1, Signed::prod(temp, quotient));
            bezout_1 = temp;
            
            temp = new_t;
            new_t = Signed::sub(bezout_1, Signed::prod(temp, quotient));
            bezout_1 = temp;
        }
        (gcd,Signed::residue(bezout_1, ring),Signed::residue(bezout_2,ring))

    }

    fn mul_inverse(&self, ring: Self) -> Option<Self> {
        let (g,inv) = self.gcd_bz(ring);
        if g == Self::ONE{
           return Some(inv);
        }
        None
    }

    /// x^p mod n
    fn exp_residue(&self, p: Self, n: Self) -> Self {
        let mut pow = p;
        let mut one = Self::ONE;
        let mut base = self.clone();
        while pow > Self::ONE {
            if pow.is_even() {
                base = base.mul_mod(base, n);
                pow >>= 1;
            } else {
                one = base.mul_mod(one, n);
                base = base.mul_mod(base, n);
                pow -= Self::ONE;
                pow >>= 1;
            }
        }
        base.mul_mod(one, n)
    }

    fn exp_unit(&self, p: Self, n: Self) -> bool {
        // FIXME optimise preferably using Montgomery arithmetic
        self.exp_residue(p,n)==Self::ONE
    }
    
    fn special_sf(&self,p: Self, n: Self) -> bool{
        unimplemented!()
    }

    fn pseudoprime_count(&self) -> (Self, Self) {
        unimplemented!()
    }

    ///
    fn semi_fermat(&self, p: Self, q: Self) -> bool {
        unimplemented!()
    }

    // Strong-fermat to a semiprime with provided factors p,q
    // fn semi_sprp(&self, p: Self, q: Self) -> bool;

    fn sqr_fermat(&self, p: Self) -> bool {
        self.exp_unit(*self-Self::ONE,*self* *self)
    }

    /// Jacobi symbol
    fn jacobi(&self, other: Self) -> i8 {
       let mut n = self.clone();
        let mut p = other.clone();
        let mut t = 1i8;
        n = n%p;

        while n != Self::ZERO {
            let zeroes = n.trailing_zeros();
            n>>=zeroes;
            
            let presidue = p.limbs[0]&7;
            
            if (presidue == 3 || presidue==5) && (zeroes&1 == 1) {
                t = -t
            }

            std::mem::swap(&mut n, &mut p);

            if n.limbs[0]&3==3 && presidue==3 {
                t = -t;
            }

            n = n%p;
        }

        if p == Self::ONE {
            t
        } else {
           0i8
        }
    }

    /// a^p-1 mod p = 1
    fn fermat(&self, a: Self) -> bool {
        a.exp_residue(*self-Self::ONE,*self)==Self::ONE
    }
    /// a^p-1 mod p = jacobi(a,p)
    fn euler_jacobi(&self, a: Self) -> bool {
        //unimplemented!()
        let e = self.jacobi(a);
        let res = self.exp_residue(*self-Self::ONE,*self);
        
        if e == -1{
           return res == *self-Self::ONE;
        }
        res==Self::ONE
    }

    /// Colin Plumb's variant of Fermat test
    fn euler_p(&self) -> bool {
        unimplemented!()
    }

    /// Strong Fermat
    fn sprp(&self, a: Self) -> bool {
    
        if self.is_even(){
           return self.fermat(a);        
        }
        
        let pminus = *self-Self::ONE;
        let twofactor = pminus.trailing_zeros();
        let d = pminus>>twofactor;
        
        let mut res = a.exp_residue(d,*self);
        
        if res == Self::ONE || res == pminus{
           return true;
        }
        
        for _ in 1..twofactor{
        
           res = res.mul_mod(res,*self);
           if res == pminus{
              return true;
           }
           
        }
        return false;
    }

    /// Probable prime
    fn is_prime(&self) -> bool {
        if self.is_even(){
           return false;
        }
        // FIXME add trial division
        for _ in 0..10{
           let w = rand();
           
           let witness = Self::from(w);
           
           if !self.sprp(witness){
               return false;
           }
           
        }
        return true
    }

    fn is_perfect_power(&self) -> bool {
        unimplemented!()
    }

    fn is_power_of(&self, x: usize) -> bool {
        unimplemented!()
    }
    
    /// Integer sqrt
    fn isqrt(&self) -> Self {
    
       let mut est = self.clone()>>((self.msb() as u32 / 2) - 1);

        loop {
            let s = est;
            let t =s+*self/s; 
            est = t>>1;
            
            if est >= s{
               return s;
            }
            
            }
    }

    /// Integer nth root
    fn nth_root(&self, n: usize) -> Self {
        unimplemented!()
    }

    fn max_exp(&self) -> (Self, Self) {
        unimplemented!()
    }

    fn is_square(&self) -> bool {
        unimplemented!()
    }

    fn factor(&self) -> Option<Factorization<Self>> {
        unimplemented!()
    }

    fn ord(&self, a: Self) -> Option<Self> {
        unimplemented!()
    }

    // Multiplicative order for P and some A guaranteed to be coprime to P
    // P= 2 is not supported. This is to be handled separately
    fn p_ord(&self, a: Self) -> Self {
        unimplemented!()
    }
    // Returns order and the signature (the largest factor of 2 dividing the order)
    fn signature(&self, a: Self) -> Option<(Self, u32)> {
        unimplemented!()
    }

    fn signature_v(&self, base: &[Self]) -> Option<(Self, Vec<u32>)> {
        unimplemented!()
    }
}

/*
  Algorithm

  Check if Length will fit in binary array of length S
  check if String is all ASCII digits

  Map string to radix-10^19 vector

  Convert to radix-2^64, fill the resultant array


*/

impl<const S: usize> std::str::FromStr for Epz<S> {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let maxlen = (19.265919722f64 * (S as f64)).floor() as usize;
        let strlen = input.len();
        if strlen > maxlen {
            return Err("Does not fit into datatype");
        }
        let strrem = strlen % 19;
        let stepcount = strlen / 19;
        let mut digits = vec![];
        
         if strrem != 0{
           match input[..strrem].parse::<u64>(){
              Ok(first_digit) => digits.push(first_digit),
              Err(_) => return Err("Parse error of first substring"),
           }
        }
        
        for i in 0..stepcount {
            let startidx = i * 19 + strrem;
            let stopidx = startidx + 19;

            match input[startidx..stopidx].parse::<u64>() {
                Ok(x) => digits.push(x),
                Err(mess) => return Err("Parse error of substring"),
            }
        }

        let mut value = Self::ZERO;

        value.limbs[0] = digits[0];

        for i in 1..digits.len() {
        // FIXME show final carry to determine if it fitted
            scale_slice(&mut value.limbs[..], 0x8AC7230489E80000, digits[i]);
        }
        Ok(value)
    }
}

// FIXME speed up algorithm; extend to hold all integers that can fit
impl<const S: usize> std::fmt::Display for Epz<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn string_format(x: u64) -> String {
            let k = x.to_string();
            let leading = (0..(19 - k.len())).map(|_| "0").collect::<String>();
            leading + &k
        }
         
        const RADIX : u64 = 0x8AC7230489E80000; 

        let mut k = vec![];
        let mut x = self.limbs;

        loop {
            let idx = leading_idx(&x[..]) + 1;

            if idx == 1 {
                break;
            }

            k.push(div_slice(&mut x[..idx], 0x8AC7230489E80000u64, 0));
        }
        let last = x[0]/RADIX;
        k.push(x[0]%RADIX);
        if last != 0{
           k.push(last);
        }

        let mut count = 0usize;
        for i in k.iter().rev() {
            if *i > 0u64 {
                break;
            }
            count += 1;
        }

        k.truncate(k.len() - count);

        let len = k.len() - 1;
        let interim = k[..len]
            .iter()
            .rev()
            .map(|x| string_format(*x))
            .collect::<Vec<String>>();
        let last = k[len].to_string() + &interim.join("");
        write!(f, "{}", last)
    }
}
