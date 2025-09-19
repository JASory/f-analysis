use crate::filter::filtertype::*;
use crate::filter::ftraits::GenericFilter;
use crate::Natural;

impl<const S: usize> GenericFilter for Base<S> {
    fn filter_check<T: Natural>(x: T) -> bool {
        x.sprp(T::from(S as u64))
    }
}

impl GenericFilter for Verum {
    fn filter_check<T: Natural>(x: T) -> bool {
        true
    }
}

impl<const S: usize, const P: usize> GenericFilter for DBase<S, P> {
    fn filter_check<T: Natural>(x: T) -> bool {
        if !x.sprp(T::from(S as u64)) {
            return false;
        }
        x.sprp(T::from(P as u64))
    }
}

impl<const S: usize, const P: usize, const Q: usize> GenericFilter for TBase<S, P, Q> {
    fn filter_check<T: Natural>(x: T) -> bool {
        if !x.sprp(T::from(S as u64)) {
            return false;
        }
        if !x.sprp(T::from(P as u64)) {
            return false;
        }
        x.sprp(T::from(Q as u64))
    }
}

impl GenericFilter for NQR {
    fn filter_check<T: Natural>(x: T) -> bool {
        if x.is_square() {
            return false;
        }

        let mut witness = T::from(3);
        loop {
            if x.jacobi(witness) == -1 {
                break;
            }
            witness.successor();
        }

        x.sprp(witness)
    }
}

impl GenericFilter for EPF {
    fn filter_check<T: Natural>(x: T) -> bool {
        x.euler_p()
    }
}
// First quadratic residue fermat base selection
//pub struct QR;

impl<const S: usize> GenericFilter for PFB<S> {
    fn filter_check<T: Natural>(x: T) -> bool {
        const BASES: [u64; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

        for i in BASES[..S].iter() {
            if !x.sprp(T::from(*i)) {
                return false;
            }
        }
        true
    }
}

impl<const S: usize> GenericFilter for GCD<S> {
    fn filter_check<T: Natural>(x: T) -> bool {
        x.gcd(T::from(S as u64)) == T::ONE
    }
}

impl<const S: usize> GenericFilter for Trial<S> {
    fn filter_check<T: Natural>(x: T) -> bool {
        if S > 128 {
            panic!("Cannot support trial division that high")
        }
        x.trial_bound(S)
    }
}

impl<const A: usize> GenericFilter for SPK<A> {
    fn filter_check<T: Natural>(x: T) -> bool {
        x.is_semiprime_k(A)
    }
}

impl<const A: usize> GenericFilter for SPKA<A> {
    fn filter_check<T: Natural>(x: T) -> bool {
        for i in 2..A {
            if x.is_semiprime_k(i) {
                return true;
            }
        }
        return false;
    }
}

impl<const S: usize> GenericFilter for Power<S> {
    fn filter_check<T: Natural>(x: T) -> bool {
        x.is_power_of(S)
    }
}
/*
impl GenericFilter for Miller{
    fn filter_check<T: Natural>(x: T) -> bool{
       Self::sprp(x)
    }

}
*/
/*
impl GenericFilter for MRC{
    fn filter_check(x: T) -> bool{
      Self::is_form(x)
    }
}
*/

impl<const A: usize> GenericFilter for SFSqr<A> {
    fn filter_check<T: Natural>(x: T) -> bool {
        let mut b = x.euclidean(T::from(A as u64)).0.isqrt();
        b.successor();
        x.sprp(b)
    }
}

impl GenericFilter for Square {
    fn filter_check<T: Natural>(x: T) -> bool {
        x.is_square()
    }
}

impl GenericFilter for Prime {
    fn filter_check<T: Natural>(x: T) -> bool {
        x.is_prime()
    }
}
