use crate::math::rand::rand;
use crate::fermat::FInteger;

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

pub(crate) fn hash_search<T: FInteger>(ce: &[T], dimen: usize, interval: usize) -> u32 {
 
    let mut dlt: u32 = 70000000;
    let mut magic: u32 = 0;
    let divisor = (32 - dimen.trailing_zeros()) as usize;
    let values = vec![0; dimen];
    let mut valvec = vec![values; interval];

    let magicvec = (0..interval).map(|_| (rand() as u32)).collect::<Vec<u32>>();

    for i in ce.iter() {
        for j in 0..interval{
            valvec[j][i.hash_shift(divisor, magicvec[j])] += 1;
        } //endfor
    }

    for i in 0..interval{
        let new_delta = delta(&valvec[i][..]);
        if new_delta < dlt {
            dlt = new_delta;
            magic = magicvec[i];
        }
    }

    return magic;
}

pub(crate) fn hash_search_no_alloc<T: FInteger>(ce: &[T],dimen: usize,bound: usize) -> u32{

    let divisor = (32 - dimen.trailing_zeros()) as usize;
    // Loop over bound 
    let mut range = u32::MAX;
    let mut magic = 0u32;
    for _ in 0..bound{
  
      let mx = rand() as u32; 
      let mut count = 0u32;

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
       range = (sup-inf);
       magic = mx;
    } 
    // end inner loop
  }
    magic
}
