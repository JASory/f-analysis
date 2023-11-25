use crate::bsv::sprp::thread_count;
use crate::math::plist;
use machine_prime::is_prime;

use crate::filter::WeakFermat;

use crate::CompVector;

pub struct Interval {
    inf: usize,
    sup: usize,
}

impl Interval {
    pub fn new(inf: usize, sup: usize) -> Self {
        if inf > sup {
            return Self { inf: sup, sup: inf };
        }
        Self { inf, sup }
    }

    // Generates integers of the form (2x+1)(4x+1),which potentially satisfy the Monier-Rabin bound
    pub fn mr_semiprime(&self) -> CompVector<u64> {
        let mut veccy = vec![];

        for x in 1..(self.sup as f64).sqrt() as u64 {
            let lhs = 2 * x + 1;
            let rhs = 4 * x + 1;

            if is_prime(lhs) {
                if is_prime(rhs) {
                    let (prod, flag) = lhs.overflowing_mul(rhs);

                    if flag {
                        break;
                    }

                    if prod < self.sup as u64 || prod > self.inf as u64 {
                        veccy.push(prod)
                    }
                }
            }
        }
        return CompVector::from_vector_unchecked(veccy);
    }
    /*
    pub fn gen_k(&self, a: u64) -> CompVector<u64>{

        let sup_sqrt = (self.sup as f64).sqrt() as usize;


    }
    */
    pub fn gen(&self, a: u64) -> CompVector<u64> {
        let mut kset = vec![]; //std::collections::HashSet::new();

        let sup_sqrt = (self.sup as f64).sqrt() as u64;

        for i in 2..a {
            for k in 1..sup_sqrt {
                let lhs = k + 1;
                let rhs = i * k + 1;

                if is_prime(lhs) {
                    if is_prime(rhs) {
                        let (prod, flag) = lhs.overflowing_mul(rhs);

                        if flag {
                            break;
                        }

                        if prod < self.sup as u64 && prod > self.inf as u64 {
                            kset.push(prod);
                            
                        }
                    }
                }
            }
        }
        CompVector::from_vector_unchecked(kset)
    }

    pub fn ak_semiprime(&self, a: u64) -> CompVector<u64> {
        let mut veccy = vec![];
        for k in 1..(self.sup as f64).sqrt() as u64 {
            let lhs = k + 1;
            let rhs = a * k + 1;

            if is_prime(lhs) {
                if is_prime(rhs) {
                    let (prod, flag) = lhs.overflowing_mul(rhs);

                    if flag {
                        break;
                    }

                    if prod < self.sup as u64 || prod > self.inf as u64 {
                        veccy.push(prod)
                    }
                }
            }
        }
        return CompVector::from_vector_unchecked(veccy);
    }

    pub fn generate_fermat<T: WeakFermat>(&self) -> CompVector<u64> {
        let subproc = |start: u64, stop: u64| -> Vec<u64> {
            let mut veccy = Vec::new();
            for i in start..stop {
                if T::fermat(i) {
                    if !is_prime(i) {
                        veccy.push(i)
                    }
                }
            }
            return veccy;
        };
        let sup = self.sup as u64;
        let inf = self.inf as u64;
        let threadcount = thread_count();

        let mut threads = vec![];

        let stride = (sup - inf) / threadcount as u64;
        for i in 0..threadcount {
            threads.push(std::thread::spawn(move || {
                subproc(inf + (stride * i as u64), inf + stride * (i as u64 + 1))
            }))
        }

        let mut collector = vec![];

        for j in threads {
            collector.push(j.join().unwrap())
        }
        collector.into_iter().flatten().collect::<CompVector<u64>>()
    }
    
    
/*
    pub fn generate_ce(&self, a: u64) -> CompVector<u64> {
        let mut k = CompVector::<u64>::new();

        let sup_sqrt = ((self.sup as f64).sqrt() as u64) * (((a as f64).log2().ceil() as u64) >> 1);

        let primes = plist(sup_sqrt as usize);
       

         for i in 2..a {
            'search: for p in primes.iter() {
                let multiplicand = i * (p - 1) + 1;

                if multiplicand > sup_sqrt {
                    break 'search;
                }

                match primes[..].binary_search(&multiplicand) {
                    Ok(_) => {
                        let (prod, flag) = p.overflowing_mul(multiplicand);
                        if flag {
                            break 'search;
                        }

                        if prod < self.sup as u64 && prod > self.inf as u64 {
                            k.append_unchecked(prod);
                        }
                    }
                    Err(_) => (),
                }
            }
        }
        k.sort();
        k
    }
*/
    /*
    // Strip primes
    pub fn filter_coprime<T: Coprime>(&self) -> CompVector{

    }

    // Strip primes
    pub fn filter_fermat<T: WeakFermat>(&self) -> CompVector{

    }
    */
}
