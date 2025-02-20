use crate::iterator::{BaseIterator};
use crate::Natural;
use crate::filter::{GenericFilter};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use crate::search::exhaustive_st;


/*
   Parallel variants of single searching 
*/

pub(crate) fn thread_count() -> usize {
    match std::thread::available_parallelism() {
        Ok(x) => usize::from(x),
        Err(_) => 1usize,
    }
}

/*
   In: A vector of composites, a base
   Out: A vector of composites pseudoprime to provided base
*/

pub(crate) fn strip_pseudo_par<T: Natural>(pseudos: Vec<T>,base: T) -> Vec<T>{
         // Number of threads
        let tc = thread_count();
         // Starting index
        let start = 0usize;
         // Final index
        let stop = pseudos.len();
        // Length of each vector to be evaluated by individual thread
        let stride = (stop-start)/tc;
 
        let mut threads : Vec<std::thread::JoinHandle::<Vec<T>>> = Vec::new();
        //   
        let p_arc = Arc::new(pseudos);

        // Split  
        for i in 0..(tc-1){
	      // Start index for  thread
          let thread_start = start+i*stride;
	      // Stop index for thread
          let thread_stop = start+stride*(i+1);
	      // Copy 
          let ce_i = Arc::clone(&p_arc);
    
        threads.push( 
            std::thread::spawn( move || {
              let mut res = vec![];
			  
              for i in ce_i[thread_start..thread_stop].iter(){
				  
                 if i.sprp(base){
                    res.push(*i)
                 }
				 
              }
           res
            } ));
        } // end for loop
  
        let ce_i = Arc::clone(&p_arc);
  
  threads.push(
        
      std::thread::spawn( move || {
        let mut res = vec![];
        for i in ce_i[start+(tc-1)*stride..stop].iter(){
           if i.sprp(base){
             res.push(*i)
           }
        }
         res
} ));

 let mut total = vec![];
 
for handle in threads{
     total.extend_from_slice(&handle.join().unwrap()[..]);
  }
  total
}

// Update 

/*
  FIXME Rename to exhaustive_interval_par
   In: A vector of composites, 
  Out: 
  */
pub(crate) fn exhaustive_list_par<T: Natural>(pseudos: Vec<T>, inf: u64, sup: u64) -> Vec<u64>{
             let tc = thread_count();
             
             let p_arc : Arc<Vec<T>> = Arc::new(pseudos);
             let b : Arc<AtomicU64>  = Arc::new(AtomicU64::new(inf));
 
             let mut thread_vec : Vec<std::thread::JoinHandle::<Vec<u64>>> = Vec::new();
             
            for i in 0..tc{
            
              let p_i = Arc::clone(&p_arc);
              let b_i = Arc::clone(&b);
              
              thread_vec.push(std::thread::spawn(move || {
              
                let mut valid_base : Vec<u64> = vec![];
                
                 'search: loop {
                // Get current index and increment by the stride
                let mut base = b_i.load(Ordering::SeqCst);
                
                 if base == sup{
                    break;
                 }
                 
                 if base <= sup{
                    base = base.wrapping_add(1);
                 }
                 
                 
                 b_i.store(base,Ordering::SeqCst);
                 
                 if exhaustive_st(&p_i,base){
                 
                   valid_base.push(base);
                 }
            }
            return valid_base;
            }
            ));
            }
            let mut total = vec![];
 
            for handle in thread_vec{
                total.extend_from_slice(&handle.join().unwrap()[..]);
            }
             total
    }
    
 /*
      In: Vector of composites, Vector of bases to evaluate
     Out: Subset of bases that eliminate all the composites 
 */
    /*
 pub(crate) fn exhaustive_vec_par<T: Natural>(pseudos: Vec<T>,base: Vec<T>) -> Vec<T>{
                     let tc = thread_count();
             
             let p_arc : Arc<Vec<T>> = Arc::new(pseudos);
             let idx : Arc<AtomicU64>  = Arc::new(AtomicUsize::new(usize::MAX));
 
             let mut thread_vec : Vec<std::thread::JoinHandle::<Vec<u64>>> = Vec::new();
             
            for i in 0..tc{
            
              let p_i = Arc::clone(&p_arc);
              let b_i = Arc::clone(&idx);
              
              thread_vec.push(std::thread::spawn(move || {
              
                let mut valid_base : Vec<u64> = vec![];
                
                 'search: loop {
                // Get current index and increment by the stride
                let mut c_idx = b_i.load(Ordering::SeqCst);
                
                 if c_idx == usize::MAX{
                    c_idx = 0;
                 }
                 if c_idx == sup{
                    break;
                 }
                 
                 if c_idx <= sup{
                    c_idx = c_idx.wrapping_add(1);
                 }
                 
                // Store the current index for other threads to access
                b_i.store(c_idx, Ordering::SeqCst);
        
                 
                 if exhaustive_st(&p_i,base){
                   valid_base.push(base);
                 }
            }
            return valid_base;
            }
            ));
            }
            let mut total = vec![];
 
            for handle in thread_vec{
                total.extend_from_slice(&handle.join().unwrap()[..]);
            }
             total
 }   
 */
/*
/*
   In: A vector of composites, a base
   Out: A vector of composites pseudoprime to provided base
*/

pub(crate) fn filter_func_par<T: Natural>(pseudos: Vec<T>,func: &dyn Fn(T) -> bool) -> Vec<T>{
         // Number of threads
        let tc = thread_count();
         // Starting index
        let start = 0usize;
         // Final index
        let stop = pseudos.len();
        // Length of each vector to be evaluated by individual thread
        let stride = (stop-start)/tc;
 
        let mut threads : Vec<std::thread::JoinHandle::<Vec<T>>> = Vec::new();
        //   
        let p_arc = Arc::new(pseudos);

        // Split  
        for i in 0..(tc-1){
	      // Start index for  thread
          let thread_start = start+i*stride;
	      // Stop index for thread
          let thread_stop = start+stride*(i+1);
	      // Copy 
          let ce_i = Arc::clone(&p_arc);
          let f_i = func.clone();
        threads.push( 
            std::thread::spawn( move || {
              let mut res = vec![];
			  
              for i in ce_i[thread_start..thread_stop].iter(){
				  
                 if (f_i)(*i){
                    res.push(*i)
                 }
				 
              }
           res
            } ));
        } // end for loop
  
        let ce_i = Arc::clone(&p_arc);
  
  threads.push(
        
      std::thread::spawn( move || {
        let mut res = vec![];
        for i in ce_i[start+(tc-1)*stride..stop].iter(){
           if func(*i){
             res.push(*i)
           }
        }
         res
} ));

 let mut total = vec![];
 
for handle in threads{
     total.extend_from_slice(&handle.join().unwrap()[..]);
  }
  total
}

*/
/*

*/

pub(crate) fn filter_par<T: Natural,F: GenericFilter>(pseudos: Vec<T>, filter_flag: bool) -> Vec<T>{

 let tc = thread_count();
 let start = 0usize;
 let stop = pseudos.len();
 let stride = (stop-start)/tc;
 let mut threads : Vec<std::thread::JoinHandle::<Vec<T>>> = Vec::new();
 
 let p_arc = Arc::new(pseudos);


 for i in 0..(tc-1){
    let thread_start = start+i*stride;
    let thread_stop = start+stride*(i+1);
    let ce_i = Arc::clone(&p_arc);
    
    threads.push( 
      std::thread::spawn( move || {
        let mut res = vec![];
        for i in ce_i[thread_start..thread_stop].iter(){
           if F::filter_check(*i)==filter_flag{
             res.push(*i)
           }
        }
        res
} ));
  } // end for loop
  
  let ce_i = Arc::clone(&p_arc);
  
  threads.push(
        
      std::thread::spawn( move || {
        let mut res = vec![];
        for i in ce_i[start+(tc-1)*stride..stop].iter(){
           if F::filter_check(*i)==filter_flag{
             res.push(*i)
           }
        }
         res
} ));

 let mut total = vec![];
 
for handle in threads{
     total.extend_from_slice(&handle.join().unwrap()[..]);
  }
  total
}



pub(crate) fn unary_strongest_par<T: Natural>(x: Vec<T>, inf: u64, sup: u64) -> (u64,u64) {
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
                 //let cb = T::gen_k(64).unwrap();
                'check: for i in ce_i.iter() {
                    if i.sprp(T::from(c_base)) {
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

    (best_base.load(Ordering::SeqCst),ce_count.load(Ordering::SeqCst))
}
// FIXME take the total and split across threads
pub(crate) fn unary_strongest_rand_par<T: Natural>(x: Vec<T>,thread_stride: u64) -> (u64,u64) {
    let tc = thread_count();
    let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();
    
    let best_base = Arc::new(AtomicU64::new(1));
    let ce_count = Arc::new(AtomicU64::new(x.len() as u64));
    let ce_vec: Arc<Vec<T>> = Arc::new(x);
    
    
    for _ in 0..tc {
        let bb_i = Arc::clone(&best_base);
        let cc_i = Arc::clone(&ce_count);
        let ce_i = Arc::clone(&ce_vec);
        
        thread_vec.push(std::thread::spawn(move || {
        
            'search : for _ in 0..thread_stride{
            
              let inner_bound = cc_i.load(Ordering::SeqCst);
              let c_base = T::gen_k(64).unwrap();
              
              if  inner_bound == 0 {
                    break 'search;
                }
                
              let mut count = 0u64;
              
              'check: for i in ce_i.iter() {
                    if i.sprp(c_base) {
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
                    bb_i.store(c_base.to_u64(), Ordering::SeqCst);
                    cc_i.store(count, Ordering::SeqCst);
                }
                
            }
            
        }));
        
   }
   
   for handle in thread_vec {
        handle.join().unwrap();
   }
      
    (best_base.load(Ordering::SeqCst),ce_count.load(Ordering::SeqCst))
   
}    

pub(crate) fn exhaustive_par<T: Natural>(x: Vec<T>) -> u64{

     const STRIDE : u64 = 1_000_000;
     
     let tc = thread_count();
     let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();
     
     let t_base = Arc::new(AtomicU64::new(0u64));
     let step = Arc::new(AtomicU64::new(u64::MAX));
     let ce = Arc::new(x);
     
     for i in 0..tc{
        let t_i = Arc::clone(&t_base);
        let s_i = Arc::clone(&step);
        let ce_i = Arc::clone(&ce);
        
       thread_vec.push(std::thread::spawn(move || {
          'search :loop {
           
            if t_i.load(Ordering::SeqCst) != 0u64{
              break 'search;
            }
            
           let st = s_i.load(Ordering::SeqCst).wrapping_add(1);
           s_i.store(st, Ordering::SeqCst);
           
           let start = st*STRIDE;
           let stop = (st+1)*STRIDE;
            
           for base in start..stop{
           
            for (idx,el) in ce_i.iter().enumerate(){
              if el.sprp(T::from(base)){
                break;
              }
             if idx==ce_i.len()-1{
               t_i.store(base,Ordering::SeqCst);
             
               break 'search;
            } 
             } // end inner loop
       
           } // end base loop 
          } // end infinite loop  
          }// end closure
       ));  
    }
      for handle in thread_vec {
        handle.join().unwrap();
      }
    t_base.load(Ordering::SeqCst)
}


pub(crate) fn exhaustive_rand_par<T: Natural>(x: Vec<T>) -> u64{

     const STRIDE : u64 = 1_000_000;
     
     let tc = thread_count();
     let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();
     
     let t_base = Arc::new(AtomicU64::new(0u64));
     let ce = Arc::new(x);
     
     for i in 0..tc{
        let t_i = Arc::clone(&t_base);
        let ce_i = Arc::clone(&ce);
        
       thread_vec.push(std::thread::spawn(move || {
          'search :loop {
           
            if t_i.load(Ordering::SeqCst) != 0u64{
              break 'search;
            }
        
           for base in 0..STRIDE{
           
            let base = T::gen_k(64).unwrap();
            
            for (idx,el) in ce_i.iter().enumerate(){
              if el.sprp(base){
                break;
              }
             if idx==ce_i.len()-1{
               t_i.store(base.to_u64(),Ordering::SeqCst);
               break 'search;
            } 
             } // end inner loop
       
           } // end base loop 
          } // end infinite loop  
          }// end closure
       ));  
    }
      for handle in thread_vec {
        handle.join().unwrap();
      }
    t_base.load(Ordering::SeqCst)
}


/*
 Base Evaluation Vector Strong Pseudoprime Vector

   In: Vector of 64-bit composite integers, Vector of 64-bit bases
   Out: Vector comprising the number of pseudoprimes for each base
*/

pub(crate) fn bev_sprpv<T: Natural>(x: Vec<T>, base_vec: Vec<T>) -> Vec<u64> {
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

pub(crate) fn binary_evo_par<T: Natural>(pseudos: Vec<T>, inf: u64, sup: u64) -> (u64,u64){
   let mut strong_lhs = inf; 
   let mut initial_pseudos = strip_pseudo_par(pseudos.clone(),T::from(strong_lhs));
   
   let (mut strong_rhs,mut strong_count) =  unary_strongest_par(initial_pseudos,inf,sup);
   let mut lhs = strong_rhs;
        
   for _ in 0..4{
    let lhs_pseudos = strip_pseudo_par(pseudos.clone(),T::from(lhs));
    let rhs_res = unary_strongest_par(lhs_pseudos,inf,sup);
    if rhs_res.1 < strong_count{
      strong_count = rhs_res.1;
      strong_lhs = lhs;
      strong_rhs =  rhs_res.0;
      
    }
    lhs = rhs_res.0;
    
   }  
   return (strong_lhs,strong_rhs) 
}
