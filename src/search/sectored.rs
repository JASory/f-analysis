use crate::fermat::FInteger;
use crate::bsv::sprp::thread_count;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use crate::search::single::unary_det_st;
use crate::math::plist;




fn gcd_check(b: u64, primes: &[u64]) -> bool {
    for i in primes {
        if b.gcd(*i) != 1 {
            return true;
        }
    }
    false
}

 fn bs_unary<T:FInteger>(ce: &[T],primes: &[u64], bound: u64) -> u64{
                let mut start = 2u64;
                let mut c_base: u64;
                // inner loop ensures that the base is coprime to any primes hashed into it
                // Failure is relatively uncommon, but it is necessary for correctness
                loop {
                    // Search for base that eliminates all the composites in the bucket
                    c_base = unary_det_st(&ce[..], start, bound);
                    // if no base found then terminate loop, c_base = zero will trigger the shortcut 
                    if c_base == 0{
                       break;
                    }
                     // If base found and coprime to all hashed primes to this index then break 
                    if !gcd_check(c_base, &primes[..]) {
                        break;
                    }
                    
                    // If base was not coprime then continue searching starting from that base
                    start = c_base + 1;
                }
                c_base
 }


pub(crate) fn base_search<T: FInteger>(
    ce: Vec<T>,
    dimen: usize,
    multiplier: u32,
    bound: u64,
) -> Option<Vec<u64>> {
    // If the dimension is not power of two then the bases cannot be computed
    if !dimen.is_power_of_two() {
        return None;
    }

    let divisor = (32 - dimen.trailing_zeros()) as usize;

    let mut output = vec![];

    for _ in 0..dimen {
        output.push(AtomicU64::new(0u64))
    }

    // stores all the primes needed to check for coprimality, to prevent false negatives
    let mut primes = vec![vec![]; dimen];
    // List of primes within the search bound
    let prime_list = plist(bound as usize);

    // Split the primes according to their hash index
    for j in 0..dimen {
        for i in &prime_list {
            if i.hash_shift(divisor, multiplier) == j {
                primes[j].push(*i)
            }
        }
    }

    let tc = thread_count();

    let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();
    // Index Max is used for coding simplicity, getting the first 0 index is simply a wrapping addition
    let idx = Arc::new(AtomicUsize::new(usize::MAX));
    // Counterexample Arc
    let ce_vec: Arc<Vec<T>> = Arc::new(ce);
    // Base output Arc
    let o_vec: Arc<Vec<AtomicU64>> = Arc::new(output);
    // Prime list Arc
    let p_vec: Arc<Vec<Vec<u64>>> = Arc::new(primes);
    // Failure flag for inability to find base
    let flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    for _ in 0..tc {
        let b_i = Arc::clone(&idx);

        let ce_i = Arc::clone(&ce_vec);

        let ov_i = Arc::clone(&o_vec);

        let f_i = Arc::clone(&flag);

        let p_i = Arc::clone(&p_vec);

        thread_vec.push(std::thread::spawn(move || {
            'search: loop {
                // Get current index and increment by one
                let c_idx = b_i.load(Ordering::SeqCst).wrapping_add(1);
                // Store the current index for other threads to access
                b_i.store(c_idx, Ordering::SeqCst);
                // Get current flag state
                let failure = f_i.load(Ordering::SeqCst);

                // End search loop if all bases have been computed OR a base couldn't be found for a bucket
                if c_idx >= dimen || failure {
                    break 'search;
                }
                
                let mut p_ce = vec![];
                // Collect composites into the bucket
                for i in ce_i.iter() {
                    if i.hash_shift(divisor, multiplier) == c_idx {
                        p_ce.push(*i)
                    }
                }
                
                let c_base = bs_unary(&p_ce[..],&p_i[c_idx][..],bound);

                // If no base could be found then set flag as true which terminates the search
                if c_base == 0 {
                    f_i.store(true, Ordering::SeqCst);
                }

                // Store c_base into vector
                let c = unsafe { ov_i.get_unchecked(c_idx) };
                c.store(c_base, Ordering::SeqCst);
            }
        }));
    } // end loop

    for handle in thread_vec {
        handle.join().unwrap();
    }

    // If flag was set return None as the base search failed
    if Arc::try_unwrap(flag).unwrap().load(Ordering::SeqCst) {
        return None;
    }

    let interim = Arc::try_unwrap(o_vec).unwrap();
    Some(
        interim
            .iter()
            .map(|q| q.load(Ordering::SeqCst))
            .collect::<Vec<u64>>(),
    )
}
