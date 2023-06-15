use crate::fermat::{FInteger};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc};

pub(crate) fn thread_count() -> usize {
    match std::thread::available_parallelism() {
        Ok(x) => usize::from(x),
        Err(_) => 1usize,
    }
}

/*
 Base Search Vector Strong Pseudoprime Interval

   In: Vector of 64-bit composite integers; Lower bound and upper bound of interval to search bases
   Out: Base, and number of composites it passes
*/

pub(crate) fn bsv_sprpi<T: FInteger>(x: Vec<T>, inf: u64, sup: u64) -> u64 {
    let tc = thread_count();
    let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();

    let base = Arc::new(AtomicU64::new(inf - 1));
    let best_base = Arc::new(AtomicU64::new(u64::MAX));
    let ce_count = Arc::new(AtomicU64::new(x.len() as u64));
    let ce_vec: Arc<Vec<T>> = Arc::new(x);

    for _ in 0..tc {
        let b_i = Arc::clone(&base);
        let bb_i = Arc::clone(&best_base);
        let cc_i = Arc::clone(&ce_count);
        let ce_i = Arc::clone(&ce_vec);

        thread_vec.push(std::thread::spawn(move || {
            'search: loop {
                let mut count = 0u64;
                // Set x as the base
                let c_base = b_i.load(Ordering::SeqCst) + 1;
                let inner_bound = cc_i.load(Ordering::SeqCst);

                b_i.store(c_base, Ordering::SeqCst);

                /*
                  Terminating conditions
                */
                if c_base > sup || inner_bound == 0 {
                    break 'search;
                }

                'check: for i in ce_i.iter() {
                    if i.sprp(T::from_u64(c_base)) {
                        count += 1
                    }
                    // Short-circuiting, if the current base passes more composites
                    // than the best base found so far then the inner loop is ended
                    if count > inner_bound {
                        break 'check;
                    }
                }

                // Update the best base so far across threads, and the number of counter examples
                if count < cc_i.load(Ordering::SeqCst) {
                    bb_i.store(c_base, Ordering::SeqCst);
                    cc_i.store(count, Ordering::SeqCst);
                }
            } // end loop
        }));
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    best_base.load(Ordering::SeqCst)
}



/*
 Base Evaluation Vector Strong Pseudoprime Vector

   In: Vector of 64-bit composite integers, Vector of 64-bit bases
   Out:
*/

pub(crate) fn bev_sprpv<T: FInteger>(x: Vec<T>, base_vec: Vec<T>) -> Vec<u64> {
    let mut output: Vec<AtomicU64> = vec![];
    let sup = base_vec.len();

    for _i in 0..sup {
        output.push(AtomicU64::new(0u64))
    }

    let tc = thread_count();
    let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();

    let base = Arc::new(AtomicUsize::new(usize::MAX));
    let ce_vec: Arc<Vec<T>> = Arc::new(x);
    let b_vec: Arc<Vec<T>> = Arc::new(base_vec);
    let o_vec: Arc<Vec<AtomicU64>> = Arc::new(output);

    for _ in 0..tc {
        let b_i = Arc::clone(&base);
        let ce_i = Arc::clone(&ce_vec);
        let bv_i = Arc::clone(&b_vec);
        let ov_i = Arc::clone(&o_vec);

        thread_vec.push(std::thread::spawn(move || 'search: loop {
            let c_idx = b_i.load(Ordering::SeqCst).wrapping_add(1);

            b_i.store(c_idx, Ordering::SeqCst);

            if c_idx > sup {
                break 'search;
            }

            let c_base = unsafe { bv_i.get_unchecked(c_idx) };

            let mut count = 0u64;

            for i in ce_i.iter() {
                if i.sprp(*c_base) {
                    count += 1
                }
            }

            let c = unsafe { ov_i.get_unchecked(c_idx) };
            c.store(count, Ordering::SeqCst);
        }));
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    let interim = Arc::try_unwrap(o_vec).unwrap();
    interim
        .iter()
        .map(|q| q.load(Ordering::SeqCst))
        .collect::<Vec<u64>>()
}
