use crate::fermat::ftrait::{FInteger, FIterator};

/*
    Iterators for Fermat bases

    Full implementation

    CompVector::search::<F:FIterator>(F,top_sample: usize) -> Vec<u64>
*/
/// An iterator that starts from x and produces composites in sequential order skipping perfect powers and primes
#[derive(Clone, Copy)]
pub struct CompSeq<T: FInteger> {
    start: T,
    idx: usize,
    length: usize,
}

/// An iterator producing X number of composites of S-bitlength
#[derive(Clone, Copy)]
pub struct CompRng<const S: usize, T: FInteger> {
    start: T,
    idx: usize,
    length: usize,
}

/// An integer producing only primes in sequential order from X
#[derive(Clone, Copy)]
pub struct PrimeSeq<T: FInteger> {
    start: T,
    idx: usize,
    length: usize,
}

/// An iterator producing X number of random primes of length S-bitlength
#[derive(Clone, Copy)]
pub struct PrimeRng<const S: usize, T: FInteger> {
    start: T,
    idx: usize,
    length: usize,
}
/// Sequential integers over the interval
#[derive(Clone, Copy)]
pub struct IntSeq<T: FInteger> {
    start: T,
    idx: usize,
    length: usize,
}

#[derive(Clone, Copy)]
pub struct IntRng<const S: usize,T: FInteger> {
    start: T,
    idx: usize,
    length: usize,
}

#[derive(Clone, Copy)]
pub struct Ideal<const S: usize,T: FInteger> {
    start: T,
    idx: usize,
    length: usize,
}



impl<T: FInteger> Iterator for IntSeq<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        self.start.successor();

        if self.idx > self.length {
            return None;
        }
        return Some(self.start);
    }
}

impl<const S: usize,T: FInteger> Iterator for IntRng<S,T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        self.start = T::gen_k(S).unwrap();

        if self.idx > self.length {
            return None;
        }
        return Some(self.start);
    }
}

impl<T: FInteger> Iterator for CompSeq<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.start.successor();
            if !self.start.is_prime() && !self.start.is_perfect_power() {
                self.idx += 1;
                if self.idx > self.length {
                    return None;
                }
                return Some(self.start.clone());
            }
        } // end loop
    }
}

impl<const S: usize, T: FInteger> Iterator for CompRng<S, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        
        loop{
             self.start = T::comp_gen_k(S).unwrap();
             
          if !self.start.is_perfect_power(){
             break;
          }
        }
        
        if self.idx > self.length {
            return None;
        }
        Some(self.start)
    }
}

impl<T: FInteger> Iterator for PrimeSeq<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.start.successor();
            if self.start.is_prime() {
                self.idx += 1;
                if self.idx > self.length {
                    return None;
                }
                return Some(self.start.clone());
            }
        } // end loop
    }
}

impl<const S: usize, T: FInteger> Iterator for PrimeRng<S, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        self.start = T::prime_gen_k(S).unwrap();
        if self.idx > self.length {
            return None;
        }
        Some(self.start)
    }
}

impl<T: FInteger> FIterator<T> for IntSeq<T> {
    fn new(s: Option<T>, len: usize) -> Option<Self> {
        match s {
            Some(x) => Some(Self {
                start: x,
                idx: 0,
                length: len,
            }),
            None => None,
        }
    }

    fn state(&self) -> T {
        self.start
    }

    fn inc(&mut self) {
        self.next();
    }

    fn to_vector(&self) -> Vec<T> {
        let mut base = vec![];
        let mut b = self.clone();
        for _ in 0..self.length {
            b.inc();
            base.push(b.state());
        }
        base
    }
}

impl<T: FInteger> FIterator<T> for CompSeq<T> {
    fn new(s: Option<T>, len: usize) -> Option<Self> {
        match s {
            Some(x) => Some(Self {
                start: x,
                idx: 0,
                length: len,
            }),
            None => None,
        }
    }

    fn state(&self) -> T {
        self.start
    }

    fn inc(&mut self) {
        self.next();
    }

    fn to_vector(&self) -> Vec<T> {
        let mut base = vec![];
        let mut b = self.clone();
        for _ in 0..self.length {
            b.inc();
            base.push(b.state());
        }
        base
    }
}

impl<const S: usize, T: FInteger> FIterator<T> for CompRng<S, T> {
    fn new(_s: Option<T>, len: usize) -> Option<Self> {
        Some(Self {
            idx: 0,
            length: len,
            start: T::default(),
        })
    }

    fn state(&self) -> T {
        self.start
    }

    fn inc(&mut self) {
        self.next();
    }

    fn to_vector(&self) -> Vec<T> {
        let mut base = vec![];
        let mut b = self.clone();
        for _ in 0..self.length {
            b.inc();
            base.push(b.state());
        }
        base
    }
}

impl<T: FInteger> FIterator<T> for PrimeSeq<T> {
    fn new(s: Option<T>, len: usize) -> Option<Self> {
        match s {
            Some(x) => Some(Self {
                start: x,
                idx: 0,
                length: len,
            }),
            None => None,
        }
    }

    fn state(&self) -> T {
        self.start
    }

    fn inc(&mut self) {
        self.next();
    }

    fn to_vector(&self) -> Vec<T> {
        let mut base = vec![];
        let mut b = self.clone();
        for _ in 0..self.length {
            b.inc();
            base.push(b.state());
        }
        base
    }
}

impl<const S: usize, T: FInteger> FIterator<T> for PrimeRng<S, T> {
    fn new(_s: Option<T>, len: usize) -> Option<Self> {
        Some(Self {
            idx: 0,
            length: len,
            start: T::default(),
        })
    }

    fn state(&self) -> T {
        self.start
    }

    fn inc(&mut self) {
        self.next();
    }

    fn to_vector(&self) -> Vec<T> {
        let mut base = vec![];
        let mut b = self.clone();
        for _ in 0..self.length {
            b.inc();
            base.push(b.state());
        }
        base
    }
}

impl<const S: usize, T: FInteger> FIterator<T> for IntRng<S, T> {
    fn new(_s: Option<T>, len: usize) -> Option<Self> {
        Some(Self {
            idx: 0,
            length: len,
            start: T::default(),
        })
    }

    fn state(&self) -> T {
        self.start
    }

    fn inc(&mut self) {
        self.next();
    }

    fn to_vector(&self) -> Vec<T> {
        let mut base = vec![];
        let mut b = self.clone();
        for _ in 0..self.length {
            b.inc();
            base.push(b.state());
        }
        base
    }
}
