use crate::car::MRC_18;
use crate::filter::WeakFermat;
use crate::iterator::BaseIterator;
use crate::natural::montcore::NTCore;
use crate::primes::{PRIMORIAL, SMALL_PRIMES, WHEEL};
use crate::search::{hash_search, thread_count, unary_ht_par};
use crate::structures::Primes;
use crate::FResult;
use crate::Natural;
use crate::{CompVector, HashTable, SOSet, SmallOrd, WieferichPrime};

use crate::enums::{Search, AUTO_FLAG, MEMORY_MAX, UTF8_FLAG};
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

/// Interval for evaluation [low;high}
#[derive(Clone)]
pub struct Interval<T: Natural> {
    inf: T,
    sup: T,
    mode: Search,
}

impl<T: Natural> Interval<T> {
    pub fn new(inf: T, sup: T) -> Self {
        let (new_inf, new_sup) = inf.min_max(sup);

        Self {
            inf: new_inf,
            sup: new_sup,
            mode: Search::Deterministic,
        }
    }

    pub fn set_weak_heuristic(&mut self) {
        self.mode = Search::WeakHeuristic;
    }

    pub fn set_strong_heuristic(&mut self) {
        self.mode = Search::StrongHeuristic;
    }

    pub fn set_spk_heuristic(&mut self, k: usize) {
        self.mode = Search::SPKHeuristic(k)
    }

    pub fn set_deterministic(&mut self) {
        self.mode = Search::Deterministic;
    }

    /// Searches for generalised Wieferich primes within the interval. p such that a^{p-1} mod p^2 = 1
    pub fn wieferich_search_st(&self, base: u64) -> WieferichPrime {
        let supremum = self.sup.to_u64() / PRIMORIAL;
        let infimum = self.inf.to_u64() / PRIMORIAL;
        // Check for small primes

        let mut res = vec![];

        for i in [2, 3, 5, 7, 11, 13] {
            if base.exp_residue(i - 1, i * i) == 1 {
                res.push(i);
            }
        }

        for i in infimum..supremum {
            let n = PRIMORIAL * i;

            for i in WHEEL {
                let p = n + i;
                if base.sqr_fermat(p) {
                    if p.is_prime() {
                        res.push(p);
                    }
                }
            }
        }

        return WieferichPrime::new(base, res);
    }

    pub fn wieferich_search(&self, a: u64) -> WieferichPrime {
        let subproc = |inf: u64, sup: u64, base: u64| -> Vec<u64> {
            let mut res = vec![];
            for i in inf..sup {
                let n = PRIMORIAL * i;

                for i in WHEEL {
                    let p = n + i;
                    if base.sqr_fermat(p) {
                        if p.is_prime() {
                            res.push(p);
                        }
                    }
                }
            }
            res
        };

        let supremum = self.sup.to_u64() / PRIMORIAL;
        let infimum = self.inf.to_u64() / PRIMORIAL;
        // Check for small primes

        let mut res = vec![];

        for i in [2u64, 3, 5, 7, 11] {
            if a.exp_residue(i - 1, i * i) == 1u64 {
                res.push(i);
            }
        }

        let t_count = thread_count() as u64;

        let mut threads = vec![];
        // FIXME eliminate to_u64, Return error if beyond some bound
        let stride = ((supremum - infimum) / t_count);
        for i in 0..t_count - 1 {
            let start = infimum + i * stride;
            let stop = infimum + (i + 1) * stride;
            let base = a.clone();
            threads.push(std::thread::spawn(move || subproc(start, stop, a)))
        }

        threads.push(std::thread::spawn(move || {
            subproc(infimum + (t_count - 1) * stride, supremum + 1, a)
        }));

        let mut collector = vec![];

        for j in threads {
            collector.push(j.join().unwrap())
        }
        collector.push(res);
        let mut res = collector.into_iter().flatten().collect::<Vec<u64>>();
        res.sort();
        WieferichPrime::new(a, res)
    }

    // FIXME return a vector of Weiferich primes, instead of printing
    /// Fermat quotients to bases of iterator
    /// This function writes to Standard Out, bases are likely to be out of sequence
    pub fn fq_sequence<F: BaseIterator<u64>>(&self, iter: F) -> () {
        let x = iter.to_vector();

        let p_bound = self.sup.isqrt().to_u64() as usize;

        let sup = x.len();
        // Check if primes are already written, if yes then restore, if no generate and write them
        let plist = Primes::generate_or_restore(p_bound);

        let tc = thread_count();
        let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();
        let zero = AtomicU64::new(0u64);
        let base = Arc::new(AtomicUsize::new(usize::MAX));
        let b_vec: Arc<Vec<u64>> = Arc::new(x.clone());
        let p_vec: Arc<Primes> = Arc::new(plist.clone());

        for t in 0..tc {
            let b_i = Arc::clone(&base);
            let pv_i = Arc::clone(&p_vec);
            let bv_i = Arc::clone(&b_vec);

            thread_vec.push(std::thread::spawn(move || 'search: loop {
                let c_idx = b_i.load(Ordering::SeqCst).wrapping_add(1);

                b_i.store(c_idx, Ordering::SeqCst);

                if c_idx >= sup {
                    break 'search;
                }

                let c_base = unsafe { bv_i.get_unchecked(c_idx) };

                let mut veccy: Vec<u64> = vec![*c_base];

                // Case where 2*2
                // let b = *c_base&3;
                // if (b*b*b)&3 == 1{
                //    veccy.push(2);
                // }
                for p in pv_i.iter() {
                    if p.sqr_fermat(*c_base) {
                        veccy.push(p)
                    }
                }
                println!("{:?}", veccy);
            }));
        }

        for handle in thread_vec {
            handle.join().unwrap();
        }
    }

    /*
    pub fn eval_heuristic(&self, plist: &Primes, file: Option<std::fs::File>) -> FResult<CompVector<T>>{
          match self.mode{
               Search::MRHeuristic =>{
                   for i in plist.iter(){

                     let lhs = T::from(i);

                     for j in [3,4,5,6].iter(){
                  let rhs = lhs.even_complement(T::from(*j));

                  if rhs.is_prime(){
                     let (prod,flag) = lhs.overflowing_mul(rhs);
                     // If multiplication overflowed or the prod exceeds the bound break loop
                     if flag  || !prod.is_bounded_by(T::ZERO,self.sup){
                       break;
                     }

                     if prod.is_bounded_by(self.inf,self.sup){
                        out.write(&prod.to_bytes()[..]).unwrap();
                     }
                  }
                }
               }

               for i in MRC_18{
                 if T::from(i).is_bounded_by(self.inf,self.sup){
                  out.write(&T::from(i).to_bytes()[..]).unwrap();
                 }
              }

                out.flush().unwrap();

               }
               Search::SPKHeuristic(a) => {
               for i in plist.iter(){

                   let lhs = T::from(i);

                      for j in 2..2048{

                     let rhs = lhs.semi_k_complement(j);

                      if rhs.is_prime(){
                         let (prod,flag) = lhs.overflowing_mul(rhs);

                          if flag || !prod.is_bounded_by(T::ZERO,self.sup){
                             break;
                          }

                         if prod.is_bounded_by(self.inf,self.sup) && !flag{
                           ce.push(prod);
                         }
                       }
                  }
                }
              }
               Search::SPKAHeuristic(a,b) =>
          }
    }
     */
    /// Calculates the set of composites generated by the heuristic
    /// # None	   
    /// Deterministic automatically fails, as this is simply all composites
    pub fn compute_heuristic(&self, locale: Option<&str>) -> FResult<CompVector<T>> {
        if self.mode == Search::Deterministic {
            return FResult::NotSupported;
        }

        let p_bound = self.sup.isqrt().to_u64() as usize;
        let plist = Primes::generate_or_restore(p_bound);

        match locale {
            // Write all composites to file
            Some(x) => {
                let mut outfile = File::create(x).unwrap();
                let mut out = std::io::BufWriter::new(outfile.try_clone().unwrap());
                // Monier-Rabin Heuristic
                for i in plist.iter() {
                    let lhs = T::from(i);

                    for j in [3, 4, 5, 6].iter() {
                        let rhs = lhs.even_complement(T::from(*j));

                        if rhs.is_prime() {
                            let (prod, flag) = lhs.overflowing_mul(rhs);
                            // If multiplication overflowed or the prod exceeds the bound break loop
                            if flag || !prod.is_bounded_by(T::ZERO, self.sup) {
                                break;
                            }

                            if prod.is_bounded_by(self.inf, self.sup) {
                                out.write(&prod.to_bytes()[..]).unwrap();
                            }
                        }
                    }
                }

                for i in MRC_18 {
                    if T::from(i).is_bounded_by(self.inf, self.sup) {
                        out.write(&T::from(i).to_bytes()[..]).unwrap();
                    }
                }

                out.flush().unwrap();

                if self.mode == Search::StrongHeuristic {
                    for i in plist.iter() {
                        let lhs = T::from(i);

                        for j in 2..2048 {
                            let rhs = lhs.semi_k_complement(j);

                            if rhs.is_prime() {
                                let (prod, flag) = lhs.overflowing_mul(rhs);

                                if flag || !prod.is_bounded_by(T::ZERO, self.sup) {
                                    break;
                                }

                                if prod.is_bounded_by(self.inf, self.sup) {
                                    out.write(&prod.to_bytes()[..]).unwrap();
                                }
                            }
                        }
                    }
                    out.flush().unwrap();
                }
                return FResult::Value(CompVector::from_file_internal(
                    outfile.try_clone().unwrap(),
                    MEMORY_MAX,
                    UTF8_FLAG,
                    AUTO_FLAG,
                ));
            } // End file write

            // Write all composites to volatile memory (i.e vector)
            None => {
                let mut ce = Vec::<T>::new();

                for i in plist.iter() {
                    let lhs = T::from(i);

                    for j in [3, 4, 5, 6].iter() {
                        let rhs = lhs.even_complement(T::from(*j));

                        if rhs.is_prime() {
                            let (prod, flag) = lhs.overflowing_mul(rhs);

                            if flag || !prod.is_bounded_by(T::ZERO, self.sup) {
                                break;
                            }

                            if prod.is_bounded_by(self.inf, self.sup) {
                                ce.push(prod);
                            }
                        }
                    }
                }

                for i in MRC_18 {
                    if T::from(i).is_bounded_by(self.inf, self.sup) {
                        ce.push(T::from(i))
                    }
                }

                if self.mode == Search::StrongHeuristic {
                    for i in plist.iter() {
                        let lhs = T::from(i);

                        for j in 2..2048 {
                            let rhs = lhs.semi_k_complement(j);

                            if rhs.is_prime() {
                                let (prod, flag) = lhs.overflowing_mul(rhs);

                                if flag || !prod.is_bounded_by(T::ZERO, self.sup) {
                                    break;
                                }

                                if prod.is_bounded_by(self.inf, self.sup) && !flag {
                                    ce.push(prod);
                                }
                            }
                        }
                    }
                }
                return FResult::Value(CompVector::from_vector(ce));
            }
        } // end match
    } // end function

    /*
        Algorithm

        Generate Heuristic within interval

        Calculate set of candidate bases

        Split interval into sections by thread

        Set the output array with element from vector

        Use is_prime to determine if element is valid
    */

    // FIXME return Insufficient Candidate giving index of value that failed
    pub fn to_hashtable(
        &self,
        dimen: Option<usize>,
        multiplier: Option<u32>,
        bound: Option<u64>,
    ) -> FResult<HashTable> {
        // Number of Bases to evaluate
        const STRIDE: usize = 20;

        let trial_div = |x: T, pf: &[u64]| -> bool {
            for i in pf {
                if Natural::is_multiple_of(&x, *i) {
                    return false;
                }
            }
            return true;
        };

        let get_factor = |x: u64| -> Vec<u64> {
            let mut veccy = vec![];
            for i in SMALL_PRIMES {
                if Natural::is_multiple_of(&x, i) {
                    veccy.push(i);
                }
            }
            veccy
        };

        let mut x = self.clone();
        x.set_strong_heuristic();
        // FIXME handle error
        // let mut ce = CompVector::<T>::from_file("fusedfermat44.bin").unwrap().load_to_memory().unwrap();//
        let mut ce = x.compute_heuristic(None).unwrap();
        //println!("Computed heuristic");
        let dim = if let Some(d) = dimen {
            d
        } else {
            (ce.len() / 200).next_power_of_two()*2
        };

        let divisor = (32 - dim.trailing_zeros()) as usize;

        let ce2 = ce.to_vector();

        // If multiplier defined use it, otherwise calculate it
        let mul = if let Some(mx) = multiplier {
            mx
        } else { // Modify hashsearch parameters, 100000
            let iterations = 600000000usize/ce2.len();
            //println!("{}",iterations);
            hash_search(&ce2, dim, iterations)
        };
        //println!("Calculated multiplier");
        // If bound defined use it, otherwise set it as 65535
        let bnd = if let Some(bd) = bound { bd } else { 65535 };
        // Dump to backup stdout
        //  println!("multiplier {}",mul);
        let baseset =
            if let FResult::Value(interim_base) = unary_ht_par::<T, STRIDE>(ce2, dim, mul, bnd) {
                interim_base
            } else {
                vec![]
            };
        // Set to NoCandidates
        if baseset.len() == 0 {
            return FResult::InsufficientCandidates(0usize);
        }

        let (inf, sup) = (self.inf, self.sup);
        let mut output = vec![];
        // Initialise witness table with zeroes
        for _ in 0..dim {
            output.push(AtomicU64::new(0u64))
        }

        let tc = thread_count();

        let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();
        // Index Max is used for coding simplicity, getting the first 0 index is simply a wrapping addition
        let idx = Arc::new(AtomicUsize::new(usize::MAX));
        // Counterexample Arc
        let base_vec: Arc<Vec<u64>> = Arc::new(baseset);
        // Base output Arc
        let o_vec: Arc<Vec<AtomicU64>> = Arc::new(output);
        // Failure flag for inability to find sufficient base
        let flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        
        for _ in 0..tc {
            // Copy of base idx
            let b_i = Arc::clone(&idx);
            // Copy of Baseset
            let be_i = Arc::clone(&base_vec);
            // Copy of base vector
            let ov_i = Arc::clone(&o_vec);
            //  Copy of Boolean flag indicating failure
            let f_i = Arc::clone(&flag);

            thread_vec.push(std::thread::spawn(move || {
                'search: loop {
                    // Get current index and increment by the stride
                    let mut c_idx = b_i.load(Ordering::SeqCst);

                    if c_idx != usize::MAX {
                        c_idx = c_idx.wrapping_add(STRIDE);
                    }

                    if c_idx == usize::MAX {
                        c_idx = 0usize
                    }

                    // Store the current index for other threads to access
                    b_i.store(c_idx, Ordering::SeqCst);

                    // Get current flag state
                    let failure = f_i.load(Ordering::SeqCst);
                    // Current bucket the loop is evaluating
                    let bucket = c_idx / STRIDE;
                    // End search loop if all bases have been computed OR a base couldn't be found for a bucket
                    if c_idx >= dim * STRIDE || failure {
                        break 'search;
                    }

                    let mut outer_flag = true;

                    'base: for j in 0..STRIDE {
                        let c = unsafe { be_i.get_unchecked(c_idx + j) };
                        let base = c.clone();
                        let inner_plist = get_factor(base);
                        // Collect composites into the bucket
                        let mut inner_flag = true;

                        let mut i = inf;
                        if i.is_even() {
                            i.successor();
                        } else {
                            i.inc_by(2);
                        }
                        'inc: loop {
                            if !i.is_bounded_by(inf, sup) {
                                break 'inc;
                            }

                            if i.hash_shift(divisor, mul) == bucket {
                                if trial_div(i, &inner_plist[..]) {
                                    if i.is_prime() != i.sprp(T::from(base)) {
                                        inner_flag = false;
                                        break 'inc;
                                    }
                                }
                            }
                            i.inc_by(2);
                        }
                        //
                        if inner_flag {
                            let d = unsafe { ov_i.get_unchecked(c_idx / STRIDE) };
                            d.store(base, Ordering::SeqCst);
                            outer_flag = false;
                            // Due to the potentially huge computation involved
                            // It helps to print to stdout in a way that lets you sort and construct manually
                            // incase of computer failure
                             println!("{} {}",c_idx/STRIDE,base);
                            break 'base;
                        }
                    } // end total base check

                    // If no valid base found set failure flag
                    if outer_flag {
                        f_i.store(true, Ordering::SeqCst);
                    }
                }
            }));
        }

        // Execute all threads
        for handle in thread_vec {
            handle.join().unwrap();
        }

        // If flag was set return Failure as the base search failed
        if Arc::try_unwrap(flag).unwrap().load(Ordering::SeqCst) {
            return FResult::Failure;
        }

        let interim = Arc::try_unwrap(o_vec).unwrap();
        // Convert the vector of Arc bases to 64-bit bases
        let veccy = interim
            .iter()
            .map(|q| q.load(Ordering::SeqCst))
            .collect::<Vec<u64>>();

        FResult::Value(HashTable::new(veccy, dim, mul))
    }

    fn generate_ce(&self, locale: Option<&str>) -> FResult<CompVector<T>> {
        if self.mode == Search::Deterministic {
            return FResult::NotSupported;
        }

        let p_bound = self.sup.isqrt().to_u64() as usize;
        let plist = Primes::generate_or_restore(p_bound);

        match locale {
            // Write all composites to file
            Some(x) => {
                let mut outfile = File::create(x).unwrap();
                let mut out = std::io::BufWriter::new(outfile.try_clone().unwrap());
                // Monier-Rabin Heuristic
                for i in plist.iter() {
                    let lhs = T::from(i);

                    for j in [3, 4, 6].iter() {
                        let rhs = lhs.even_complement(T::from(*j));

                        if rhs.is_prime() {
                            let (prod, flag) = lhs.overflowing_mul(rhs);
                            // If multiplication overflowed or the prod exceeds the bound break loop
                            if flag || !prod.is_bounded_by(T::ZERO, self.sup) {
                                break;
                            }

                            if prod.is_bounded_by(self.inf, self.sup) {
                                out.write(&prod.to_bytes()[..]).unwrap();
                            }
                        }
                    }
                }

                for i in MRC_18 {
                    if T::from(i).is_bounded_by(self.inf, self.sup) {
                        out.write(&T::from(i).to_bytes()[..]).unwrap();
                    }
                }

                out.flush().unwrap();

                if self.mode == Search::StrongHeuristic {
                    for i in plist.iter() {
                        let lhs = T::from(i);

                        for j in 2..64 {
                            let rhs = lhs.semi_k_complement(j);

                            if rhs.is_prime() {
                                let (prod, flag) = lhs.overflowing_mul(rhs);

                                if flag || !prod.is_bounded_by(T::ZERO, self.sup) {
                                    break;
                                }

                                if prod.is_bounded_by(self.inf, self.sup) {
                                    out.write(&prod.to_bytes()[..]).unwrap();
                                }
                            }
                        }
                    }
                    out.flush().unwrap();
                }
                return FResult::Value(CompVector::from_file_internal(
                    outfile.try_clone().unwrap(),
                    MEMORY_MAX,
                    UTF8_FLAG,
                    AUTO_FLAG,
                ));
            } // End file write

            // Write all composites to volatile memory (i.e vector)
            None => {
                let mut ce = Vec::<T>::new();

                for i in plist.iter() {
                    let lhs = T::from(i);
                    for j in [3, 4, 5, 6].iter() {
                        let rhs = lhs.even_complement(T::from(*j));

                        if rhs.is_prime() {
                            let (prod, flag) = lhs.overflowing_mul(rhs);

                            if flag || !prod.is_bounded_by(T::ZERO, self.sup) {
                                break;
                            }

                            if prod.is_bounded_by(self.inf, self.sup) {
                                ce.push(prod);
                            }
                        }
                    }
                }

                for i in MRC_18 {
                    if T::from(i).is_bounded_by(self.inf, self.sup) {
                        ce.push(T::from(i))
                    }
                }

                if self.mode == Search::StrongHeuristic {
                    for i in plist.iter() {
                        let lhs = T::from(i);

                        for j in 2..2048 {
                            let rhs = lhs.semi_k_complement(j);

                            if rhs.is_prime() {
                                let (prod, flag) = lhs.overflowing_mul(rhs);

                                if flag || !prod.is_bounded_by(T::ZERO, self.sup) {
                                    break;
                                }

                                if prod.is_bounded_by(self.inf, self.sup) && !flag {
                                    ce.push(prod);
                                }
                            }
                        }
                    }
                }
                return FResult::Value(CompVector::from_vector(ce));
            }
        } // end match
    }

    pub fn generate_fermat<F: WeakFermat>(&self) -> CompVector<T> {
        let subproc = |mut start: T, fstride: u64| -> Vec<T> {
            let mut veccy = Vec::new();
            for _ in 0..fstride {
                if F::fermat(start) {
                    if !start.is_prime() {
                        veccy.push(start)
                    }
                }
                start.successor();
            }
            return veccy;
        };

        let t_count = thread_count() as u64;

        let mut threads = vec![];
        // FIXME eliminate to_u64, Return error if beyond some bound
        let stride = self
            .sup
            .wrapping_sub(self.inf)
            .euclidean(T::from(t_count))
            .0
            .to_u64();
        for i in 0..t_count {
            let mut start = self.inf;
            start.inc_by(stride * (i as u64));

            threads.push(std::thread::spawn(move || subproc(start, stride)))
        }

        let mut collector = vec![];

        for j in threads {
            collector.push(j.join().unwrap())
        }
        let res = collector.into_iter().flatten().collect::<Vec<T>>();

        CompVector::<T>::from_vector(res)
    }

    /// Generate Fermat Pseudoprimes runtime base
    pub fn generate_fermat_rt(&self, base: T) -> CompVector<T> {
        let subproc = |mut start: T, fstride: u64, base: T, fact: Vec<u64>| -> Vec<T> {
            let mut veccy = Vec::new();
            for _ in 0..fstride {
                if !start.div_vector(&fact[..]) {
                    if start.fermat(base) {
                        if !start.is_prime() {
                            veccy.push(start)
                        }
                    }
                }
                start.successor();
            }
            return veccy;
        };

        let subproc_two = |mut start: T, fstride: u64, base: T, fact: Vec<u64>| -> Vec<T> {
            let mut veccy = Vec::new();
            for _ in 0..fstride / 2 {
                if !start.div_vector(&fact[..]) {
                    if start.fermat(base) {
                        if !start.is_prime() {
                            veccy.push(start)
                        }
                    }
                }
                start.inc_by(2);
            }
            return veccy;
        };

        let t_count = thread_count() as u64;

        let mut threads = vec![];
        // FIXME eliminate to_u64, Return error if beyond some bound
        let stride = self
            .sup
            .wrapping_sub(self.inf)
            .euclidean(T::from(t_count))
            .0
            .to_u64();
        let sf = base.small_factor();
        for i in 0..t_count {
            let sf_i = sf.clone();
            let mut start = self.inf;
            start.inc_by(stride * (i as u64));
            if base.is_even() {
                if start.is_even() {
                    start.successor();
                }

                threads.push(std::thread::spawn(move || {
                    subproc_two(start, stride, base, sf_i)
                }))
            } else {
                threads.push(std::thread::spawn(move || {
                    subproc(start, stride, base, sf_i)
                }))
            }
        }
        let mut collector = vec![];

        for j in threads {
            collector.push(j.join().unwrap())
        }
        let res = collector.into_iter().flatten().collect::<Vec<T>>();
        /*



            match out{



              Some(x) => {



                  res.write_binary(x);



                  return Composite::from_file(x).unwrap();



                  }



              None =>  Composite::from_vector(res),



        }



        */
        CompVector::<T>::from_vector(res)
    }
}

impl Interval<u64> {
    pub fn fast_fermat(a: u64, sup: u64) -> CompVector<u64> {
        let st = std::time::Instant::now();
        let (sm, smpseudo) = SmallOrd::initialise(a,(sup as f64).powf(2.0/3.0).ceil() as u64,sup);
        println!("Ord initialise {:?}",st.elapsed());
        let st = std::time::Instant::now();
        let sfpseudo = sm.pseudoprime_shallow();
        let stop = st.elapsed();
        println!("Small factor {:?}",stop);
                let st = std::time::Instant::now();
        let (k, start) = SOSet::from_general_mersenne(a, sup);
        let stop = st.elapsed();
        println!("Cunningham {:?}",stop);
                        let st = std::time::Instant::now();
        let p = SOSet::from_interval(a, start, sup);
        let stop = st.elapsed();
        println!("Interval {:?}",stop);
        let intrvl = Interval::new(2, sup.isqrt());

        let wieferich = intrvl.wieferich_search(a);

        let mut hash = std::collections::HashSet::new();

        for i in sfpseudo {
            hash.insert(i);
        }
        
        for i in smpseudo{
           hash.insert(i);
        }

        for i in k.pseudoprimes() {
            hash.insert(i);
        }
        for i in p.pseudoprimes() {
            hash.insert(i);
        }
        let mut k = hash.drain().collect::<CompVector<u64>>();

        let mut double = vec![];
        for i in k.iter().unwrap() {
            for j in wieferich.iter() {
                if i % j == 0 {
                    let n = i * j;
                    if Natural::fermat(&n, a) {
                        double.push(n);
                    }
                }
            }
        }
        k.elements.extend_from_slice(&mut double[..]);
        k.sort();
        k
    }
}
