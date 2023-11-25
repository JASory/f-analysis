use crate::filter::filtertype::*;
use crate::filter::ftraits::{Coprime,FormCheck};
use crate::fermat::FInteger;



impl<const S: usize> Coprime for Trial<S> {
    fn coprime<T: FInteger>(x: T) -> bool {
        if S > 128 {
            panic!("Cannot support trial division that high")
        }
        x.trial_bound(S)
    }
}

impl<const S: usize> Coprime for GCD<S> {
    fn coprime<T: FInteger>(x: T) -> bool {
        x.gcd(T::from_u64(S as u64)) == T::ONE
    }
}

impl FormCheck for Square {
    fn is_form<T: FInteger>(x: T) -> bool {
        x.is_square()
    }
}

impl<const A: usize> FormCheck for SPK<A> {
    fn is_form<T: FInteger>(x: T) -> bool {
        x.is_semiprime_k(A)
    }
}

impl<const A: usize> FormCheck for Power<A> {
    fn is_form<T: FInteger>(x: T) -> bool {
        x.is_power_of(A)
    }
}

impl<const A: usize> FormCheck for SPKA<A> {
    fn is_form<T: FInteger>(x: T) -> bool {
        for i in 2..A {
            if x.is_semiprime_k(i) {
                return true;
            }
        }
        return false;
    }
}
