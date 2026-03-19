use crate::filter::filtertype::*;
use crate::filter::ftraits::{Coprime, FormCheck, GenericFilter};
use crate::Natural;

impl<const S: usize> Coprime for Trial<S> {
    fn coprime<T: Natural>(x: T) -> bool {
        Self::filter_check(x)
    }
}

impl<const S: usize> Coprime for GCD<S> {
    fn coprime<T: Natural>(x: T) -> bool {
        //x.gcd(T::from_u64(S as u64)) == T::ONE
        Self::filter_check(x)
    }
}

impl FormCheck for Square {
    fn is_form<T: Natural>(x: T) -> bool {
        // x.is_square()
        Self::filter_check(x)
    }
}

impl<const P: u64, const Q: u64> FormCheck for SPK<P,Q>{
    fn is_form<T: Natural>(x: T) -> bool {
        Self::filter_check(x)
    }
}

impl<const A: usize> FormCheck for Power<A> {
    fn is_form<T: Natural>(x: T) -> bool {
        Self::filter_check(x)
    }
}

impl<const A: usize> FormCheck for SPKA<A> {
    fn is_form<T: Natural>(x: T) -> bool {
        Self::filter_check(x)
    }
}
