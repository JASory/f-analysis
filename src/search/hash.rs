use crate::natural::rand::rand;
use crate::Natural;

/*
   In : A list of integers
   Out: The range of the elements
*/

fn delta(x: &[u32]) -> u32 {
    let mut max = x[0];
    let mut min = x[0];

    for i in x {
        if *i > max {
            max = *i
        }
        if *i < min {
            min = *i
        }
    }
    max - min
}

/*
   In: A list of integers, dimension of hashtable, and number of multipliers to test
   Out: The best candidate multiplier that evenly distributes the integers

   TODO Find a better distributor than minimising range, 
   candidates include minimising arithmetic standard deviation or arithmetic variance   
*/


pub(crate) fn hash_search<T: Natural>(ce: &[T], dimen: usize, interval: usize) -> u32 {
    // Maximum permitted delta between minimum and maximum values (i.e range of elements) 
    let mut dlt: u32 = 70000000;
	// Multiplier initialised to zero
    let mut magic: u32 = 0;
	// Divisor shift, as hash is computed over 2^32
    let divisor = (32 - dimen.trailing_zeros()) as usize;
	// Zero initialised array for the buckets of the hash
    let values = vec![0; dimen];
	// Array of 
    let mut valvec = vec![values; interval];
    // List of candidate multipliers
    let magicvec = (0..interval).map(|_| (rand() as u32)).collect::<Vec<u32>>();
	
    // Fill each of the arrays with the hash result produced by the multiplier 
    for i in ce.iter() {
		
        for j in 0..interval{
            valvec[j][i.hash_shift(divisor, magicvec[j])] += 1;
        } //endfor
    }
	
   // Evaluate the strength of each  multiplier
    for i in 0..interval{
        let new_delta = delta(&valvec[i][..]);
        if new_delta < dlt {
            dlt = new_delta;
            magic = magicvec[i];
        }
    }

    return magic;
}

/*
    Hash search without large allocation, current algorithm is extremely slow and not used anywhere
	This is only usable if the standard hash search requires too much memory (extremely unlikely)
*/


pub(crate) fn hash_search_no_alloc<T: Natural>(ce: &[T],dimen: usize,bound: usize) -> u32{

    let divisor = (32 - dimen.trailing_zeros()) as usize;
    // Loop over bound 
    let mut range = u32::MAX;
    let mut magic = 0u32;
    for _ in 0..bound{
  
      let mx = rand() as u32; 
      //let count = 0u32;

      let mut inf = u32::MAX;
      let mut sup = 0u32;
    
      for idx in 0..dimen{
      
          let mut count = 0u32;
          
          for i in ce.iter(){
          
              let k = i.hash_shift(divisor, mx);
              
              if k == dimen{
                  count+=1
              }
          }
    
          if count < inf{
             inf = count;
          }
          
          if count > sup{
             sup=count;
          }
      } // end 
    
    if (sup-inf) < range{
       range = sup-inf;
       magic = mx;
    } 
    // end inner loop
  }
    magic
}
