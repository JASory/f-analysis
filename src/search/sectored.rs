use crate::fermat::FInteger;
use crate::search::parallel::thread_count;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use crate::search::single::unary_det_st;
use crate::structures::Primes;
use crate::result::FResult;




/*
   Functions for constructing hashtables
*/


/*
    In : An integer, a list of primes
	Out: Boolean indicating if the integer is coprime to all primes listed, true if it is, false otherwise
*/
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
                
                if ce.len() == 0{
                   return 2;
                }
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

// FIXME is Exhaustive the right return type?
pub(crate) fn unary_ht_par<T: FInteger, const S: usize>(
    ce: Vec<T>,
    dimen: usize,
    multiplier: u32,
    bound: u64,
) -> FResult<Vec<u64>> {
    // If the dimension is not power of two then the bases cannot be computed
    if !dimen.is_power_of_two() {
        return FResult::NotSupported;
    }
     // Shifting divisor
    let divisor = (32 - dimen.trailing_zeros()) as usize;

    let mut output = vec![];

    for _ in 0..(dimen)*S {
        output.push(AtomicU64::new(0u64))
    }

    // stores all the primes needed to check for coprimality, to prevent false negatives
    let mut primes = vec![vec![]; dimen];
    // List of primes within the search bound
      
    let mut prime_list = Primes::init(bound as usize).to_vector();
    // Primes does not contain 2
    prime_list.push(2);
     
    // Split the primes according to their hash index
    for j in 0..dimen {
        for i in &prime_list {
            if i.hash_shift(divisor, multiplier) == j {
                primes[j].push(*i)
            }
        }
    }
   
     // Number of threads to use, typically the system max
    let tc = thread_count();
    // vector to contain all threads
    let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();
    // Index Max is used for coding simplicity, getting the first 0 index is simply a wrapping addition
    let idx = Arc::new(AtomicUsize::new(usize::MAX));
    // Counterexample Arc
    let ce_vec: Arc<Vec<T>> = Arc::new(ce);
    // Base output Arc
    let o_vec: Arc<Vec<AtomicU64>> = Arc::new(output);
    // Prime list Arc
    let p_vec: Arc<Vec<Vec<u64>>> = Arc::new(primes);
    // Failure flag for inability to find sufficient base
    let flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    // Index of failure
    let f_indx : Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0usize));
    
    for _ in 0..tc {
		// Copy of base idx
        let b_i = Arc::clone(&idx);
        // Copy of pseudoprimes
        let ce_i = Arc::clone(&ce_vec);
        // Copy of base vector
        let ov_i = Arc::clone(&o_vec);
        //  Copy of Boolean flag indicating failure
        let f_i = Arc::clone(&flag);
        // Copy of Index of failure
        let findx_i = Arc::clone(&f_indx);
        // Copy of hashed primes vector
        let p_i = Arc::clone(&p_vec);

        thread_vec.push(std::thread::spawn(move || {
            'search: loop {
                // Get current index and increment by the stride
                let mut c_idx = b_i.load(Ordering::SeqCst);//.wrapping_add(S);
                
                 if c_idx != usize::MAX{
                    c_idx = c_idx.wrapping_add(S);
                 }
                 
                 if c_idx == usize::MAX{
                    c_idx = 0usize
                 }
                
                // Store the current index for other threads to access
                b_i.store(c_idx, Ordering::SeqCst);
                // Get current flag state
                let failure = f_i.load(Ordering::SeqCst);

                // End search loop if all bases have been computed OR a base couldn't be found for a bucket
                if c_idx >= dimen*S || failure {
                    break 'search;
                }
                                 
                let mut p_ce = vec![];
                // Collect composites into the bucket
                for i in ce_i.iter() {
                    if i.hash_shift(divisor, multiplier) == c_idx/S {
                        p_ce.push(*i)
                    }
                }
            
				// Searches for an array of bases that are both coprime to all the primes in the bucket and 
                // Eliminates all pseudoprimes				
                let c_base = array_bs_unary::<T,S>(&p_ce[..],&p_i[c_idx/S][..],bound);
                
                
                // If array could not be filled then set flag as true which terminates the search
                if c_base[S-1] == 0 {
                // Set failure flag
                    f_i.store(true, Ordering::SeqCst);
                // Set the index that was failed at    
                    findx_i.store(c_idx/S,Ordering::SeqCst);
                }

                // Store c_base into vector
                for i in 0..S{
                    let c = unsafe { ov_i.get_unchecked(c_idx+i) };
                    c.store(c_base[i], Ordering::SeqCst);
                }
            }
        }));
    } // end loop

     // Execute all threads
    for handle in thread_vec {
        handle.join().unwrap();
    }
	

    // If flag was set return InsufficientCandidates with the number of valid candidates
    if Arc::try_unwrap(flag).unwrap().load(Ordering::SeqCst) {
        let idx = Arc::try_unwrap(f_indx).unwrap().load(Ordering::SeqCst);
        return FResult::InsufficientCandidates(idx);
    }
     
    let interim = Arc::try_unwrap(o_vec).unwrap();
	// Convert the vector of Arc bases to 64-bit bases and return
    FResult::Exhaustive(
        interim
            .iter()
            .map(|q| q.load(Ordering::SeqCst))
            .collect::<Vec<u64>>(),
    )
}


/*

    Hash interval algorithm
	
	Generate heuristic composites between bounds
	
	Search for an evenly distributing hash multiplier
	
	Search for 10 bases that eliminate the all composites in the bucket
	
	
	Count each prime that goes into each hashbucket
	
	
	Iterate over each base and count the primes it finds 
	
	If the base passed composites then skip to evaluate the next one, if all evaluate to fail then halt
	
*/

 fn array_bs_unary<T:FInteger, const S: usize>(ce: &[T],primes: &[u64], bound: u64) -> [u64;S]{
     	        let mut start = 2u64;
                let mut c_base: u64;
				
				if ce.len() == 0{
				  return [2u64;S];
				}
				
				let mut base_array : [u64;S] = [0u64;S];
				// loop for each index
				for i in 0..S{
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
                    // Set the valid base as the i-th element if the base array
				    base_array[i] = c_base;
				    // Start searching for the next valid base that is greater than the current one
				    start = c_base + 1;
				}
				// Return base array
                base_array
 }
