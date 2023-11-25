/*
   Parallel variants of single 
*/

/*
   Strips pseudoprimes parallelised
*/

pub(crate) fn strip_pseudo_par(pseudos: &[T],base: T) -> Vec<T>{
   let stride = (stop-start)/tc;
 let mut threads : Vec<std::thread::JoinHandle::<Vec<T>>> = Vec::new();

 for i in 0..(tc-1){
    let thread_start = start+i*stride;
    let thread_stop = start+stride*(i+1);
    threads.push( 
      std::thread::spawn( move || { 
        thread_plist(thread_start,thread_stop)
} ));
  } // end for loop

   // Last interval to account for any integer division flooring
  threads.push(
    std::thread::spawn( move || { 
     thread_plist(start+(tc-1)*stride,stop)
}));
  
  let mut total = vec![];
  
  for handle in threads{
     total.extend_from_slice(&handle.join().unwrap()[..]);
  }
  total
}
