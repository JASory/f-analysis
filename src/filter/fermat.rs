use crate::filter::filtertype::*;
use crate::filter::ftraits::{WeakFermat,EulerFermat,StrongFermat,GenericFilter};
use crate::fermat::FInteger;

impl<const S: usize> WeakFermat for Base<S> {
    fn fermat<T: FInteger>(x: T) -> bool {
        x.fermat(T::from_u64(S as u64))
    }
}

impl<const S: usize> EulerFermat for Base<S>{
   fn efermat<T: FInteger>(x: T) -> bool{
      x.euler_jacobi(T::from_u64(S as u64))
   }
}


impl<const S: usize> StrongFermat for Base<S> {
    fn sprp<T: FInteger>(x: T) -> bool {
        x.sprp(T::from_u64(S as u64))
    }
}

impl<const S: usize, const P : usize> WeakFermat for DBase<S,P> {
    fn fermat<T: FInteger>(x: T) -> bool {
        if !x.fermat(T::from_u64(S as u64)){
           return false
        }
        x.fermat(T::from_u64(P as u64))
    }
}

impl<const S: usize, const P: usize> EulerFermat for DBase<S,P>{
   fn efermat<T: FInteger>(x: T) -> bool{
      if !x.euler_jacobi(T::from_u64(S as u64)){
           return false
        }
        x.euler_jacobi(T::from_u64(P as u64))
   }
}


impl<const S: usize, const P : usize> StrongFermat for DBase<S,P> {
    fn sprp<T: FInteger>(x: T) -> bool {
        if !x.sprp(T::from_u64(S as u64)){
           return false
        }
        x.sprp(T::from_u64(P as u64))
    }
}

impl<const S: usize, const P : usize, const Q: usize> WeakFermat for TBase<S,P,Q> {
    fn fermat<T: FInteger>(x: T) -> bool {
        if !x.fermat(T::from_u64(S as u64)){
           return false
        }
        if !x.fermat(T::from_u64(P as u64)){
           return false
        }
        x.fermat(T::from_u64(Q as u64))
    }
}

impl<const S: usize, const P : usize, const Q: usize> EulerFermat for TBase<S,P,Q> {
    fn efermat<T: FInteger>(x: T) -> bool {
        if !x.euler_jacobi(T::from_u64(S as u64)){
           return false
        }
        if !x.euler_jacobi(T::from_u64(P as u64)){
           return false
        }
        x.euler_jacobi(T::from_u64(Q as u64))
    }
}

impl<const S: usize, const P : usize, const Q: usize> StrongFermat for TBase<S,P,Q> {
    fn sprp<T: FInteger>(x: T) -> bool {
        if !x.sprp(T::from_u64(S as u64)){
           return false
        }
        if !x.sprp(T::from_u64(P as u64)){
           return false
        }
        x.sprp(T::from_u64(Q as u64))
    }
}

impl<const S: usize> WeakFermat for PFB<S> {
    fn fermat<T: FInteger>(x: T) -> bool {
        const BASES: [u64; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

        for i in BASES[..S].iter() {
            if !x.fermat(T::from_u64(*i)) {
                return false;
            }
        }
        true
    }
}

impl<const S: usize> EulerFermat for PFB<S> {
    fn efermat<T: FInteger>(x: T) -> bool {
        const BASES: [u64; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

        for i in BASES[..S].iter() {
            if !x.euler_jacobi(T::from_u64(*i)) {
                return false;
            }
        }
        true
    }
}

impl<const S: usize> StrongFermat for PFB<S> {
    fn sprp<T: FInteger>(x: T) -> bool {
        const BASES: [u64; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

        for i in BASES[..S].iter() {
            if !x.sprp(T::from_u64(*i)) {
                return false;
            }
        }
        true
    }
}
/*
impl WeakFermat for SFSqr<const A: usize>{

  fn fermat<T: FInteger>(x: T) -> bool{
     let b = (X.to_u64()/A as u64).isqrt()+1;
     x.fermat(T::from_u64(b))
  }

}

impl StrongFermat for Miller{

  fn sprp<T: FInteger>(x: T) -> bool{
    let b = (X.to_u64()/A as u64).isqrt()+1;
     x.fermat(T::from_u64(b))
  }

}
*/

impl WeakFermat for NQR {
    fn fermat<T: FInteger>(x: T) -> bool {
        if x.is_square() {
            return false;
        }

        let mut witness = T::from_u64(3);
        loop {
            if x.jacobi(witness) == -1 {
                break;
            }
            witness.successor();
        }

        x.fermat(witness)
    }
}

impl EulerFermat for NQR {
    fn efermat<T: FInteger>(x: T) -> bool {
        if x.is_square() {
            return false;
        }

        let mut witness = T::from_u64(3);
        loop {
            if x.jacobi(witness) == -1 {
                break;
            }
            witness.successor();
        }

        x.euler_jacobi(witness)
    }
}

impl StrongFermat for NQR {
    fn sprp<T: FInteger>(x: T) -> bool {
        if x.is_square() {
            return false;
        }

        let mut witness = T::from_u64(3);
        loop {
            if x.jacobi(witness) == -1 {
                break;
            }
            witness.successor();
        }

        x.sprp(witness)
    }
}

impl WeakFermat for EPF{
  fn fermat<T: FInteger>(x: T) -> bool{
     x.euler_p()
  }
}
