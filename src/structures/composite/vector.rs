use crate::search::{
    bev_sprpv, binary_det_iter_st, binary_evo_par, binary_evo_st, filter_par, filter_st,
    hash_search, strip_pseudo_par, strip_pseudo_st, unary_ht_par, unary_strongest_par,
    unary_strongest_st,
};

use crate::filter::*;
use crate::io::read::{read_binary, read_composite_newline, read_newline};
use crate::io::write::{format_block, write_binary};

use crate::iterator::BaseIterator;
use crate::structures::{DataVector, Point};
use crate::Natural;

use crate::structures::hashtable::HashTable;

pub fn filter_generic_v<T: Natural, F: GenericFilter>(x: &Vec<T>, filter_flag: bool) -> Vec<T> {
    if x.len() > 10000 {
        return filter_par::<T, F>(x.clone(), filter_flag);
    }
    filter_st::<T, F>(&x[..], filter_flag)
}

/// Returns numbers that are coprime to the first K primes
pub fn filter_coprime<T: Natural, F: Coprime>(x: &Vec<T>) -> Vec<T> {
    if x.len() > 10000 {
        return filter_par::<T, F>(x.clone(), true);
    }
    filter_st::<T, F>(&x[..], true)
}

/// Returns numbers that are NOT coprime to the first K primes
pub fn nfilter_coprime<T: Natural, F: Coprime>(x: &Vec<T>) -> Vec<T> {
    if x.len() > 10000 {
        return filter_par::<T, F>(x.clone(), false);
    }
    filter_st::<T, F>(&x[..], false)
}

/// Returns numbers that pass the weak fermat test
pub fn filter_fermat<T: Natural, F: WeakFermat>(x: &Vec<T>) -> Vec<T> {
    let mut res = Vec::<T>::new();
    for i in x.iter() {
        if F::fermat(*i) {
            res.push(*i)
        }
    }
    res
}

/// Returns numbers that do NOT pass the weak fermat test
pub fn nfilter_fermat<T: Natural, F: WeakFermat>(x: &Vec<T>) -> Vec<T> {
    let mut res = Vec::<T>::new();
    for i in x.iter() {
        if !F::fermat(*i) {
            res.push(*i)
        }
    }
    res
}

/// Returns numbers that pass the Euler-Jacobi fermat test
pub fn filter_eulerfermat<T: Natural, F: EulerFermat>(x: &Vec<T>) -> Vec<T> {
    let mut res = Vec::<T>::new();
    for i in x.iter() {
        if F::efermat(*i) {
            res.push(*i)
        }
    }
    res
}

/// Returns numbers that pass the Euler-Jacobi fermat test
pub fn nfilter_eulerfermat<T: Natural, F: EulerFermat>(x: &Vec<T>) -> Vec<T> {
    let mut res = Vec::<T>::new();
    for i in x.iter() {
        if !F::efermat(*i) {
            res.push(*i)
        }
    }
    res
}

/// Returns the numbers that pass the strong fermat test
pub fn filter_sprp<T: Natural, F: StrongFermat>(x: &Vec<T>) -> Vec<T> {
    if x.len() > 10000 {
        return filter_par::<T, F>(x.clone(), true);
    }
    filter_st::<T, F>(&x[..], true)
}

/// Returns the numbers that pass the strong fermat test
pub fn nfilter_sprp<T: Natural, F: StrongFermat>(x: &Vec<T>) -> Vec<T> {
    if x.len() > 10000 {
        return filter_par::<T, F>(x.clone(), false);
    }
    filter_st::<T, F>(&x[..], false)
}

/// Returns numbers that are of the form
pub fn filter_form<T: Natural, F: FormCheck>(x: &Vec<T>) -> Vec<T> {
    if x.len() > 10000 {
        return filter_par::<T, F>(x.clone(), true);
    }
    filter_st::<T, F>(&x[..], true)
}

/// Returns numbers that are NOT of the form
pub fn nfilter_form<T: Natural, F: FormCheck>(x: &Vec<T>) -> Vec<T> {
    if x.len() > 10000 {
        return filter_par::<T, F>(x.clone(), false);
    }
    filter_st::<T, F>(&x[..], false)
}

/// Returns numbers between 2 integers, exclusive
pub fn filter_interval<T: Natural>(x: &Vec<T>, inf: T, sup: T) -> Vec<T> {
    let mut res = Vec::<T>::new();
    for i in x.iter() {
        if i.is_bounded_by(inf, sup) {
            res.push(*i)
        }
    }
    res
}

/// Returns numbers NOT between 2 integers, exclusive
pub fn nfilter_interval<T: Natural>(x: &Vec<T>, inf: T, sup: T) -> Vec<T> {
    let mut res = Vec::<T>::new();
    for i in x.iter() {
        if !i.is_bounded_by(inf, sup) {
            res.push(*i)
        }
    }
    res
}
//
pub fn sprp_ce<T: Natural>(x: Vec<T>, n: T) -> Vec<T> {
    if x.len() > 1000 {
        return strip_pseudo_st::<T>(&x[..], n);
    }
    strip_pseudo_par::<T>(x, n)
}
/*
/// Counts the number of Fermat pseudoprimes
pub fn fermat_ce(x : &Vec<T>, n: u64) -> u64{
   let mut count = 0u64;
   for i in x.iter(){
     if fermat(*i,n){
       count+=1;
     }
   }
  count
}
*/

/// Collects the elements that comprise both CompVectors
pub fn set_union<T: Natural>(x: &Vec<T>, otra: &Vec<T>) -> Vec<T> {
    let mut k = std::collections::HashSet::new();
    for i in otra.iter() {
        k.insert(i);
    }
    for i in otra.iter() {
        k.insert(i);
    }
    k.drain().map(|x| *x).collect::<Vec<T>>()
}

/// Collects elements that exist in both CompVectors
pub fn set_intersection<T: Natural>(x: &Vec<T>, otra: &Vec<T>) -> Vec<T> {
    let mut res = Vec::<T>::new();
    for i in otra.iter() {
        if x.contains(i) {
            res.push(*i)
        }
    }
    res
}

/// Collects elements that exist in both CompVectors
pub fn set_complement<T: Natural>(x: &Vec<T>, otra: &Vec<T>) -> Vec<T> {
    let mut res = Vec::<T>::new();
    for i in otra.iter() {
        if !x.contains(i) {
            res.push(*i)
        }
    }
    res
}

/*
   Algorithm

   Find the strongest base in the interval
   Perform evolutionary search until one composite

*/

/// Evaluates the number of composites using the bases provided
pub fn sprp_eval_vec<T: Natural>(x: Vec<T>, base: Vec<T>) -> DataVector<T> {
    let res = bev_sprpv(x, base.clone());
    let mut q = vec![];
    for (i, j) in res.iter().zip(base.iter()) {
        q.push(Point::<T>::new(*j, *i))
    }
    let mut res = DataVector::<T>::new(q);
    res.sort();
    res
}

/// Evaluates the number of composites left for each base produced by the iterator sorted from least to greatest
/// Returns None if the Interator is not initialised with the proper parameters. See BaseIterator documentation
pub fn sprp_eval<F: BaseIterator<T>, T: Natural>(
    x: &Vec<T>,
    start: Option<T>,
    length: usize,
) -> Option<DataVector<T>> {
    match F::new(start, length) {
        Some(b) => {
            let base = b.to_vector();
            let res = bev_sprpv(x.clone(), base.clone());
            let mut q = vec![];
            for (i, j) in res.iter().zip(base.iter()) {
                q.push(Point::<T>::new(*j, *i))
            }
            let mut res = DataVector::<T>::new(q);
            res.sort();
            return Some(res);
        }
        None => None,
    }
}

pub fn det_search<F: BaseIterator<T>, T: Natural>(x: &Vec<T>, iter: F) -> (T, T) {
    binary_det_iter_st::<T, F>(x, iter)
}

/// Evolutionary search for the two strongest bases within the exclusive interval
/// First the strongest initial base is found and then a strong complement
/// These two values are then swapped for up to four cycles
pub fn evo_search<T: Natural>(x: &Vec<T>, inf: u64, sup: u64) -> (u64, u64) {
    if x.len() > 10000 {
        return binary_evo_par(x.clone(), inf, sup);
    }
    binary_evo_st(&x[..], inf, sup)
}

/// Infinite search until a base is found that eliminates all the composites
pub fn terminating_search<F: BaseIterator<T>, T: Natural>(x: &Vec<T>, mut iter: F) -> T {
    loop {
        iter.next();
        let base = iter.state();
        for (idx, el) in x.iter().enumerate() {
            if el.sprp(base) {
                break;
            }

            if idx == x.len() - 1 {
                return base;
            }
        }
    }
    return T::ZERO;
}

/// Produces the set of Strong Fermat bases that eliminate all composites, searching up to a bound
pub fn iter_sprp_search<T: Natural>(x: &Vec<T>, sup: u64) -> Vec<u64> {
    let mut y = x.clone();
    let mut bases = vec![];

    loop {
        if y.len() == 0 {
            return bases;
        }

        let x = unary_strongest_par(y.clone(), 2, sup).0;
        bases.push(x);
        let mut res = vec![];

        for i in y.iter() {
            if i.sprp(T::from(x)) {
                res.push(*i)
            }
        }
        y = res;
    } // end loop
}
