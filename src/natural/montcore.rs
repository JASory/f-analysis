pub(crate) trait NTCore : PartialEq + Sized {
    fn mont_sub(&self, y: Self, n: Self) -> Self;

    fn mont_add(&self, y: Self, n: Self) -> Self;

    fn inv_2(&self) -> Self;

    fn inv_2_neg(&self) -> Self;

    fn mont_prod(&self, y: Self, inv: Self, n: Self) -> Self;

    fn mont_sqr(&self, inv: Self, n: Self) -> Self;

    fn to_z(&self, inv: Self, n: Self) -> Self;

    fn n_identity(&self) -> Self;
    // 2 in Montgomery form
    fn two_identity(&self, one: Self) -> Self;

    // n-1 Montgomery form
    fn one_inverse_n(&self, one: Self) -> Self;

    fn to_mont(&self, n: Self) -> Self;

    fn mont_pow(&self, one: Self, p: Self, inv: Self, n: Self) -> Self;

    fn sprp(&self, base: Self) -> bool;

    fn mont_sprp(
        &self,
        base: Self,
        d: Self,
        twofactor: u32,
        one: Self,
        oneinv: Self,
        npi: Self,
    ) -> bool;
    // Odd only fermat
    fn odd_fermat(&self, base: Self) -> bool;
    // Prime square
    fn p_sq_fermat(&self, base: Self) -> bool;

    fn fermat(&self, base: Self) -> bool;
    /// Odd only
    fn odd_exp_residue(&self, p: Self, n: Self) -> Self;

    /// Power of two ring
    fn even_exp_residue(&self, p: Self, n: Self) -> Self;

    fn expr(&self, p: Self, n: Self) -> Self;

    fn exp_one(&self, p: Self, n: Self) -> bool;
    
    fn special_sf(&self, p: Self, n: Self) -> bool;
}

impl NTCore for u64 {
    fn mont_add(&self, x: Self, n: Self) -> Self {
        let res = self.wrapping_add(x);

        if res > n {
            return res.wrapping_sub(n);
        }
        res
    }

    fn mont_sub(&self, y: Self, n: Self) -> Self {
        machine_prime::mont_sub(*self, y, n)
    }

    fn n_identity(&self) -> Self {
        machine_prime::one_mont(*self)
    }

    fn two_identity(&self, one: Self) -> Self {
        machine_prime::two_mont(one, *self)
    }

    fn one_inverse_n(&self, one: Self) -> Self {
        self.mont_sub(one, *self)
    }

    fn to_mont(&self, n: Self) -> Self {
        machine_prime::to_mont(*self, n)
    }

    fn inv_2(&self) -> Self {
        machine_prime::mul_inv2(*self)
    }

    fn inv_2_neg(&self) -> Self {
        let inv = self.inv_2();
        inv.wrapping_neg()
    }

    fn mont_prod(&self, y: Self, inv: Self, n: Self) -> Self {
        machine_prime::mont_prod(*self, y, inv, n)
    }

    fn mont_sqr(&self, inv: Self, n: Self) -> Self {
        self.mont_prod(*self, inv, n)
    }

    fn to_z(&self, inv: Self, n: Self) -> Self {
        let lo = self.wrapping_mul(inv);
        let lo = ((lo as u128).wrapping_mul(n as u128) >> 64) as u64;

        lo.wrapping_neg().wrapping_add(n)
    }

    fn mont_pow(&self, mut one: Self, mut p: Self, inv: Self, n: Self) -> Self {
        machine_prime::mont_pow(*self, one, p, inv, n)
    }

    fn odd_exp_residue(&self, p: Self, n: Self) -> Self {
        let one = n.n_identity();
        let base = self.to_mont(n);
        let inv = n.inv_2();

        base.mont_pow(one, p, inv, n).to_z(inv, n)
    }

    fn even_exp_residue(&self, p: Self, n: Self) -> Self {
        //let n = (1<<twofactor)-1;
        let mut z = 1;
        let mut base = *self;
        let mut pow = p;
        while pow > 1 {
            if pow & 1 == 0 {
                base = base.wrapping_mul(base) & n;
                pow >>= 1;
            } else {
                z = base.wrapping_mul(z) & n;
                base = base.wrapping_mul(base) & n;
                pow>>=1;
            }
        }
        base.wrapping_mul(z) & n
    }

    fn expr(&self, p: Self, n: Self) -> Self {
        if n & 1 == 0 {
            let k = n.trailing_zeros() as u64;
            let s = n >> k;

            let reducer = (1 << k) - 1; // A shorthand for arithmetic over Z[2k]

            let k_rem = self.even_exp_residue(p, reducer);

            let s_rem = self.odd_exp_residue(p, s);

            let s_inv = s.inv_2() & reducer;

            let y = k_rem.wrapping_sub(s_rem).wrapping_mul(s_inv) & reducer;
            s_rem + s * y
        } else {
            self.odd_exp_residue(p, n)
        }
    }

    fn odd_fermat(&self, base: Self) -> bool {
        let one = self.n_identity();
        let inv = self.inv_2();
        let p = self.wrapping_sub(1) >> 1;
        let b = base.to_mont(*self);
        let oneinv = self.one_inverse_n(one);
        let res = b.mont_pow(one, p, inv,*self);
        if res == one || res == oneinv {
            return true;
        }
        if res.mont_prod(res, inv,*self) == one {
            return true;
        }
        false
    }
    // Prime square fermat
    // FIXME Test that this is actually correct
    fn p_sq_fermat(&self, p: Self) -> bool {
        let n = p.wrapping_mul(p);
        let one = n.n_identity();
        let inv = n.inv_2();
        let p = p.wrapping_sub(1) >> 1;
        let b = self.to_mont(n);
        let oneinv = n.one_inverse_n(one);
        let res = b.mont_pow(one, p, inv, n);
        if res == one || res == oneinv {
            return true;
        }
        if res.mont_prod(res, inv, n) == one {
            return true;
        }
        false
    }

    // Full fermat test
    fn fermat(&self, a: Self) -> bool {
        if *self & 1 == 1 {
            return self.odd_fermat(a);
        }
        // FIXME Look to see if you can have an Even-only fermat test that is faster
        // Possible candidate a^(n-1) mod n/2 == 1 this can be kept in montgomery arithmetic
        if a.expr(self.wrapping_sub(1), *self) == 1 {
            return true;
        }
        return false;
    }

    // FIXME - Prove this to have no errors for 2Z except powers of two
    fn sprp(&self, base: Self) -> bool {
        let p_minus = self.wrapping_sub(1);
        let twofactor = p_minus.trailing_zeros();
        let mut d = p_minus >> twofactor;

        let inv = self.inv_2();
        let one = self.n_identity();
        let mut result = base.to_mont(*self);
        result = result.mont_pow(one, d, inv,*self);
        let oneinv = self.one_inverse_n(one);

        if result == one || result == oneinv {
            return true;
        }

        for _ in 1..twofactor {
            result = result.mont_sqr(inv,*self);

            if result == oneinv {
                return true;
            }
        }
        false
    }

    fn mont_sprp(
        &self,
        b: Self,
        d: Self,
        twofactor: u32,
        one: Self,
        oneinv: Self,
        inv: Self,
    ) -> bool {
        let mut x = b.mont_pow(one, d, inv,*self);
        if x == one || x == oneinv {
            return true;
        }
        for _ in 1..twofactor {
            x = x.mont_sqr(inv,*self);

            if x == oneinv {
                return true;
            }
        }
        false
    }

    fn exp_one(&self, p: Self, n: Self) -> bool {
        let tzc = n.trailing_zeros();
        // If n in  then we can keep it in Montgomery form
        // If n in 2Z AND n \notin 4Z then we can also keep it in montgomery form
        // As a \in 2Z+1 therefore a^p mod 2 = 1 which means that the result is entirely dependent on
        // a^p mod n/2
        if tzc < 2 {
            let ring = n >> tzc;
            let one = ring.n_identity();
            let inv = ring.inv_2();
            let b = self.to_mont(ring);
            let oneinv = ring.one_inverse_n(one);
            if p & 1 == 0 {
                let res = b.mont_pow(one, p >> 1, inv,ring);

                if res == one || res == oneinv {
                    return true;
                }
                if res.mont_prod(res, inv,ring) == one {
                    return true;
                }
                return false;
            } else {
                let res = b.mont_pow(one, p, inv ,ring);
                if res == one {
                    return true;
                }
                return false;
            }
        } else {
            if self.expr(p, n) == 1 {
                return true;
            }
            return false;
        }
    }
    fn special_sf(&self, p: Self, n: Self) -> bool{
        let zeroes = p.trailing_zeros();
        let d = p>>zeroes;
        let inv = n.inv_2();
        let one = n.n_identity();
        let b = self.to_mont(n);
        let mut x = b.mont_pow(one, d, inv,n);
        let oneinv = n.one_inverse_n(one);
        if x == one || x == oneinv {
            return true;
        }
        for _ in 1..zeroes {
            x = x.mont_sqr(inv,n);

            if x == oneinv {
                return true;
            }
        }
        false
        
    }

}

#[inline(always)]
const fn split_to_u128(x: u128) -> (u128, u128) {
    let (lo, hi) = unsafe { std::mem::transmute::<u128, (u64, u64)>(x) };
    (hi as u128, lo as u128)
}

impl NTCore for u128 {
    fn mont_add(&self, x: Self, n: Self) -> Self {
        let res = self.wrapping_add(x);

        if res > n {
            return res.wrapping_sub(n);
        }
        res
    }

    fn mont_sub(&self, y: Self, n: Self) -> Self {
        machine_prime::mont_sub_128(*self, y, n)
    }

    fn n_identity(&self) -> Self {
        machine_prime::one_mont_128(*self)
    }

    fn two_identity(&self, one: Self) -> Self {
        machine_prime::two_mont_128(one, *self)
    }

    fn one_inverse_n(&self, one: Self) -> Self {
        self.mont_sub(one, *self)
    }

    fn inv_2(&self) -> Self {
        machine_prime::mul_inv2_128(*self)
    }

    fn inv_2_neg(&self) -> Self {
        let inv = self.inv_2();
        inv.wrapping_neg()
    }

    fn to_mont(&self, n: Self) -> Self {
        machine_prime::to_mont_128(*self, n)
    }

    fn mont_prod(&self, y: Self, inv: Self, n: Self) -> Self {
        machine_prime::mont_prod_128(*self, y, inv, n)
    }

    fn mont_sqr(&self, inv: Self, n: Self) -> Self {
        machine_prime::mont_sqr_128(*self, inv, n)
    }

    fn to_z(&self, inv: Self, n: Self) -> Self {
        let lo = self.wrapping_mul(inv);
        let lo = machine_prime::u256prod_hi(lo, n);

        lo.wrapping_neg().wrapping_add(n)
    }

    fn mont_pow(&self, one: Self, p: Self, inv: Self, n: Self) -> Self {
        machine_prime::mont_pow_128(*self,one,p,inv,n)
    }

    fn odd_fermat(&self, base: Self) -> bool {
        let one = self.n_identity();
        let inv = self.inv_2();
        let p = self.wrapping_sub(1) >> 1;
        let b = base.to_mont(*self);
        let oneinv = self.one_inverse_n(one);
        let res = b.mont_pow(one, p, inv,*self);
        if res == one || res == oneinv {
            return true;
        }

        if res.mont_sqr(inv,*self) == one {
            return true;
        }
        false
    }

    fn exp_one(&self, p: Self, n: Self) -> bool {
        let tzc = n.trailing_zeros();
        // If n in  then we can keep it in Montgomery form
        // If n in 2Z AND n \notin 4Z then we can also keep it in montgomery form
        // As a \in 2Z+1 therefore a^p mod 2 = 1 which means that the result is entirely dependent on
        // a^p mod n/2
        if tzc < 2 {
            let ring = n >> tzc;
            let one = ring.n_identity();
            let oneinv = ring.one_inverse_n(one);
            let inv = ring.inv_2();
            let b = self.to_mont(ring);
            if p & 1 == 0 {
                let res = b.mont_pow(one, p >> 1, inv,ring);
                if res == one || res == oneinv {
                    return true;
                }
                if res.mont_sqr(inv,ring) == one {
                    return true;
                }
                return false;
            } else {
                if b.mont_pow(one, p, inv,ring) == one {
                    return true;
                }
                return false;
            }
        } else {
            if self.expr(p, n) == 1 {
                return true;
            }
            return false;
        }
    }

    fn p_sq_fermat(&self, p: Self) -> bool {
        let n = p.wrapping_mul(p);
        let one = n.n_identity();
        let inv = n.inv_2();
        let p = p.wrapping_sub(1) >> 1;
        let b = self.to_mont(n);
        let oneinv = n.one_inverse_n(one);
        let res = b.mont_pow(one, p, inv,n);
        if res == one || res == oneinv {
            return true;
        }
        if res.mont_sqr(inv,n) == one {
            return true;
        }
        false
    }

    fn odd_exp_residue(&self, p: Self, n: Self) -> Self {
        let base = (self%n).to_mont(n);
        let inv = n.inv_2();
        let one = n.n_identity();
        base.mont_pow(one, p, inv, n).to_z(inv, n)
    }

    fn even_exp_residue(&self, p: Self, mask: Self) -> Self {
        let mut z = 1;
        let mut base = *self;
        let mut pow = p;

        while pow > 1 {
            if pow & 1 == 0 {
                base = base.wrapping_mul(base) & mask;
                pow >>= 1;
            } else {
                z = base.wrapping_mul(z) & mask;
                base = base.wrapping_mul(base) & mask;
                pow >>=1;
            }
        }
        base.wrapping_mul(z) & mask
    }

    fn expr(&self, p: Self, n: Self) -> Self {
        if n & 1 == 0 {
            let k = n.trailing_zeros() as u64;
            let s = n >> k;

            let reducer = (1u128 << k) - 1; // A shorthand for arithmetic over Z[2k]

            let k_rem = self.even_exp_residue(p, reducer);

            let s_rem = self.odd_exp_residue(p, s);

            let s_inv = s.inv_2() & reducer;

            let y = k_rem.wrapping_sub(s_rem).wrapping_mul(s_inv) & reducer;
            s_rem + s * y
        } else {
            self.odd_exp_residue(p, n)
        }
    }
    // Full fermat test
    fn fermat(&self, a: Self) -> bool {
        if *self & 1 == 1 {
            return self.odd_fermat(a);
        }
        if a.expr(self.wrapping_sub(1), *self) == 1 {
            return true;
        }
        return false;
    }

    fn sprp(&self, base: Self) -> bool {
        let p_minus = self.wrapping_sub(1);
        let zeroes = p_minus.trailing_zeros();
        let d = p_minus >> zeroes;

        let inv = self.inv_2();
        let one = self.n_identity();
        let b = base.to_mont(*self);
        let mut x = b.mont_pow(one, d, inv,*self);
        let oneinv = self.one_inverse_n(one);
        //println!("one {} oneinv {} bmont {} pow{}",one,oneinv,b,x);
        if x == one || x == oneinv {
            return true;
        }
        for _ in 1..zeroes {
            x = x.mont_sqr(inv,*self);

            if x == oneinv {
                return true;
            }
        }
        false
    }

    fn mont_sprp(
        &self,
        b: Self,
        d: Self,
        twofactor: u32,
        one: Self,
        oneinv: Self,
        inv: Self,
    ) -> bool {
        let mut x = b.mont_pow(one, d, inv,*self);
        if x == one || x == oneinv {
            return true;
        }
        for _ in 1..twofactor {
            x = x.mont_sqr(inv,*self);

            if x == oneinv {
                return true;
            }
        }
        false
    }
    fn special_sf(&self, p: Self, n: Self) -> bool{
        let zeroes = p.trailing_zeros();
        let d = p>>zeroes;
        let inv = n.inv_2();
        let one = n.n_identity();
        let b = self.to_mont(n);
        let mut x = b.mont_pow(one, d, inv,n);
        let oneinv = n.one_inverse_n(one);
        if x == one || x == oneinv {
            return true;
        }
        for _ in 1..zeroes {
            x = x.mont_sqr(inv,n);

            if x == oneinv {
                return true;
            }
        }
        false
        
    }

}

#[test]
fn prp() {
    assert!(2047u64.sprp(2));
    assert!(341u64.odd_fermat(2));
}
