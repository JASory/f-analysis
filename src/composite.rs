use crate::bsv::sprp::{bev_sprpv, bsv_sprpi};
use crate::filters::*;
use crate::io::read::{read_binary, read_composite_newline, read_newline};
use crate::io::write::{format_block, write_binary};

use crate::fdata::{FData, Point};
use crate::fermat::{FInteger, FIterator};
use crate::hashtable::HashTable;
use crate::hs::hashsearch::{hash_search_no_alloc,hash_search};
use crate::hs::basesearch::{base_search};

/// A vector of integers guaranteed to be composite (unless user calls unchecked variants)
#[derive(Clone, Debug, PartialEq)]
pub struct CompVector<T: FInteger> {
    elements: Vec<T>,
}

impl<T: FInteger> CompVector<T> {
    /// Initializes an empty CompVector
    pub fn new() -> Self {
        Self { elements: vec![] }
    }

    /// Initialize from vector, permitting any elements
    pub fn from_vector_unchecked(elements: Vec<T>) -> Self {
        Self { elements }
    }

    /// Adds integer x
    pub fn append_unchecked(&mut self, x: T) {
        self.elements.push(x);
    }

    /// Adds integer x, except if it is prime
    pub fn append(&mut self, x: T) {
        if !x.is_prime() {
            self.elements.push(x);
        }
    }

    /// Initializes from vector removing any primes
    pub fn from_vector(elements: Vec<T>) -> Self {
        let mut k = Self::new();
        for i in elements.iter() {
            k.append(*i)
        }
        k
    }

    /// Returns the length
    pub fn len(&self) -> usize {
        self.elements.len()
    }
    /*
     /// Minimum and maximum integers in the set
    pub fn min_max(&self) -> (u64,u64){
       let mut max = 0u64;
       let mut min = u64::MAX;

       for i in self.elements.iter(){
         if *i > max{
           max = *i;
         }
         if *i < min{
           min = *i
         }
       }
       (min,max)
    }
    */

    /// Sorts the elements of CompVector
    pub fn sort(&mut self) {
        self.elements.sort();
    }

    /// Returns an iterator
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.elements.iter()
    }

    /// Returns a mutable iterator
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.elements.iter_mut()
    }

    /// Add all unique elements from other to self
    pub fn fuse(&mut self, other: &Self) {
        let mut k = std::collections::HashSet::new();
        for i in other.elements.iter() {
            k.insert(i);
        }
        for i in self.elements.iter() {
            k.insert(i);
        }
        let z = k.drain().map(|x| *x).collect::<Vec<T>>();
        self.elements = z;
    }

    /// Reads from a file location, removing any primes it encounters
    pub fn read_utf8(locale: &str) -> Option<Self> {
        match std::fs::read_to_string(locale) {
            Ok(data) => match read_composite_newline::<T>(data) {
                Some(elements) => return Some(Self::from_vector_unchecked(elements)),
                None => None,
            },
            Err(_) => None,
        }
    }

    /// Read from a UTF-8 file location, permitting any value to be included
    pub fn read_utf8_unchecked(locale: &str) -> Option<Self> {
        match std::fs::read_to_string(locale) {
            Ok(data) => match read_newline::<T>(data) {
                Some(elements) => return Some(Self::from_vector_unchecked(elements)),
                None => None,
            },
            Err(_) => None,
        }
    }
    /// Read from binary, permitting any value to be included
    pub fn read_binary_unchecked(locale: &str) -> Option<Self> {
        match std::fs::read(locale) {
            Ok(data) => Some(Self::from_vector_unchecked(read_binary::<T>(data))),
            Err(_) => None,
        }
    }

    /// Write to a utf-8 file
    pub fn write_utf8(&self, locale: &str) -> Option<()> {
        use std::fs::File;
        use std::io::Write;

        match File::create(locale) {
            Ok(mut out) => {
                let strout = self
                    .elements
                    .iter()
                    .map(|k| k.to_string())
                    .collect::<Vec<String>>();
                let res = strout.join("\n");
                match out.write_all(res.as_bytes()) {
                    Ok(_) => Some(()),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }
    /// Write to binary file
    pub fn write_binary(&self, locale: &str) -> Option<()> {
        use std::fs::File;
        use std::io::Write;

        match File::create(locale) {
            Ok(mut out) => {
                let res = write_binary::<T>(&self.elements);
                match out.write_all(&res) {
                    Ok(_) => Some(()),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    /// Returns numbers that are coprime to the first K primes
    pub fn filter_coprime<F: Coprime>(&self) -> Self {
        let mut res = Self::new();
        for i in &self.elements {
            if F::coprime(*i) {
                res.append_unchecked(*i)
            }
        }
        res
    }

    /// Returns numbers that are NOT coprime to the first K primes
    pub fn nfilter_coprime<F: Coprime>(&self) -> Self {
        let mut res = Self::new();
        for i in &self.elements {
            if !F::coprime(*i) {
                res.append_unchecked(*i)
            }
        }
        res
    }

    /// Returns numbers that pass the weak fermat test
    pub fn filter_fermat<F: WeakFermat>(&self) -> Self {
        let mut res = Self::new();
        for i in &self.elements {
            if F::fermat(*i) {
                res.append_unchecked(*i)
            }
        }
        res
    }
    
     /// Returns numbers that do NOT pass the weak fermat test
    pub fn nfilter_fermat<F: WeakFermat>(&self) -> Self {
        let mut res = Self::new();
        for i in &self.elements {
            if !F::fermat(*i) {
                res.append_unchecked(*i)
            }
        }
        res
    }
    
    /// Returns numbers that pass the Euler-Jacobi fermat test
    pub fn filter_eulerfermat<F: EulerFermat>(&self) -> Self {
        let mut res = Self::new();
        for i in &self.elements {
            if F::efermat(*i) {
                res.append_unchecked(*i)
            }
        }
        res
    }
    
     /// Returns numbers that pass the Euler-Jacobi fermat test
    pub fn nfilter_eulerfermat<F: EulerFermat>(&self) -> Self {
        let mut res = Self::new();
        for i in &self.elements {
            if !F::efermat(*i) {
                res.append_unchecked(*i)
            }
        }
        res
    }

    /// Returns the numbers that pass the strong fermat test
    pub fn filter_sprp<F: StrongFermat>(&self) -> Self {
        let mut res = Self::new();
        for i in &self.elements {
            if F::sprp(*i) {
                res.append_unchecked(*i)
            }
        }
        res
    }

    /// Returns the numbers that pass the strong fermat test
    pub fn nfilter_sprp<F: StrongFermat>(&self) -> Self {
        let mut res = Self::new();
        for i in &self.elements {
            if !F::sprp(*i) {
                res.append_unchecked(*i)
            }
        }
        res
    }

    /// Returns numbers that are of the form
    pub fn filter_form<F: FormCheck>(&self) -> Self {
        let mut res = Self::new();
        for i in &self.elements {
            if F::is_form(*i) {
                res.append_unchecked(*i)
            }
        }
        res
    }

    /// Returns numbers that are NOT of the form
    pub fn nfilter_form<F: FormCheck>(&self) -> Self {
        let mut res = Self::new();
        for i in &self.elements {
            if !F::is_form(*i) {
                res.append_unchecked(*i)
            }
        }
        res
    }

    /// Returns numbers between 2 integers, exclusive
    pub fn filter_interval(&self, inf: T, sup: T) -> Self {
        let mut res = Self::new();
        for i in &self.elements {
            if i.is_bounded_by(inf, sup) {
                res.append_unchecked(*i)
            }
        }
        res
    }

    /// Returns numbers NOT between 2 integers, exclusive
    pub fn nfilter_interval(&self, inf: T, sup: T) -> Self {
        let mut res = Self::new();
        for i in &self.elements {
            if !i.is_bounded_by(inf, sup) {
                res.append_unchecked(*i)
            }
        }
        res
    }

    /*
    /// Counts the number of base-n counterexamples
    pub fn sprp_ce(&self, n: u64) -> u64{
       let mut count = 0u64;
       for i in &self.elements{
         if sprp(*i,n){
           count+=1;
         }
       }
      count
    }
    /// Counts the number of Fermat pseudoprimes
    pub fn fermat_ce(&self, n: u64) -> u64{
       let mut count = 0u64;
       for i in &self.elements{
         if fermat(*i,n){
           count+=1;
         }
       }
      count
    }
    */
    /// Collects the elements that comprise both CompVectors
    pub fn set_union(&self, other: &Self) -> Self {
        let mut k = std::collections::HashSet::new();
        for i in other.elements.iter() {
            k.insert(i);
        }
        for i in self.elements.iter() {
            k.insert(i);
        }
        k.drain().map(|x| *x).collect::<CompVector<T>>()
    }

    /// Collects elements that exist in both CompVectors
    pub fn set_intersection(&self, other: &Self) -> Self {
        let mut res = CompVector::new();
        for i in other.iter() {
            if self.elements.contains(i) {
                res.append_unchecked(*i)
            }
        }
        res
    }

    /// Collects elements that exist in both CompVectors
    pub fn set_complement(&self, other: &Self) -> Self {
        let mut res = CompVector::new();
        for i in other.iter() {
            if !self.elements.contains(i) {
                res.append_unchecked(*i)
            }
        }
        res
    }

    /*
    /// Counts the number of fermat and Strong fermat counterexamples
    pub fn fsprp_ce(&self, n: u64) -> (u64,u64){
        let mut s_count = 0u64;
        let mut f_count = 0u64;

        for i in &self.elements{
          match fsprp(*i,n){
          Pseudoprime::Fermat => {f_count+=1},
          Pseudoprime::Strong => {s_count+=1},
          _=> (),
          }
        }
        return (f_count+s_count, s_count)
    }
    */
    /// Returns the strongest base from the provided interval
    pub fn search_sprp(&self, inf: u64, sup: u64) -> u64 {
        bsv_sprpi(self.elements.clone(), inf, sup)
    }

    /// Evaluates the number of composites using the bases provided
    pub fn sprp_eval_vec(self, base: Vec<T>) -> FData<T> {
        let res = bev_sprpv(self.elements, base.clone());
        let mut q = vec![];
        for (i, j) in res.iter().zip(base.iter()) {
            q.push(Point::<T>::new(*j, *i))
        }
        let mut res = FData::<T>::new(q);
        res.sort();
        res
    }

    /// Evaluates the number of composites left for each base produced by the iterator sorted from least to greatest
    /// Returns None if the Interator is not initialised with the proper parameters. See FIterator documentation
    pub fn sprp_eval<F: FIterator<T>>(&self, start: Option<T>, length: usize) -> Option<FData<T>> {
        match F::new(start, length) {
            Some(b) => {
                let base = b.to_vector();
                let res = bev_sprpv(self.elements.clone(), base.clone());
                let mut q = vec![];
                for (i, j) in res.iter().zip(base.iter()) {
                    q.push(Point::<T>::new(*j, *i))
                }
                let mut res = FData::<T>::new(q);
                res.sort();
                return Some(res);
            }
            None => None,
        }
    }

    /// Produces the set of Strong Fermat bases that eliminate all composites, searching up to a bound
    pub fn iter_sprp_search(&self, sup: u64) -> Vec<u64> {
        let mut y = self.elements.clone();
        let mut bases = vec![];

        loop {
            if y.len() == 0 {
                return bases;
            }

            let x = bsv_sprpi(y.clone(), 2, sup);
            bases.push(x);
            let mut res = vec![];

            for i in y.iter() {
                if i.sprp(T::from_u64(x)) {
                    res.push(*i)
                }
            }
            y = res;
        } // end loop
    }


    /// Attempts to construct a hashtable of fermat bases with the provided arguments (size, hash multiplier, and fermat base maximum) or use defaults.  
    /// Note that providing the same integer parameters for the same set results in identical tables being produced, 
    /// allowing reproducibility. 
    /// Variation is primarily determined by the multiplier value which is pseudorandomly generated if not provided. 
    /// For instance to_hashtable(None,Some(3411698987), None) will always produce the same table for the same composite set
    /// as the dimension is computed as a ratio of the length and the base maximum is 65535 by default
    pub fn to_hashtable(&self, dimen: Option<usize>, multiplier: Option<u32>,bound: Option<u64>) -> Option<HashTable> {
        // If dimension of hashtable defined use it, otherwise calculate it 
        let dim = if let Some(dm) = dimen {
              dm
            } else {
            (self.len()/150).next_power_of_two()
        };
       
        // If multiplier defined use it, otherwise calculate it
        let mul = if let Some(mx) = multiplier {
              mx
            } else {
            hash_search(&self.elements[..], dim, 1000)
        };
        
        // If multiplier defined use it, otherwise set it as 65535
        let bnd = if let Some(bd) = bound {
              bd
            } else {
            65535
        };
        
        match base_search(self.elements.clone(), dim, mul, bnd) {
            Some(x) => Some(HashTable::new(x, dim, mul)),
            None => None,
        }
    }
}

impl<T: FInteger> FromIterator<T> for CompVector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut c = CompVector::new();

        for i in iter {
            c.append_unchecked(i);
        }

        c
    }
}

impl<T: FInteger> std::fmt::Display for CompVector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let q = format_block::<4, T>(&self.elements);

        write!(f, "{}", q)
    }
}
