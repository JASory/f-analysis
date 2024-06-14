use crate::fermat::{FInteger,FIterator};
use crate::filter::GenericFilter;

/*
   In: A slice of composites, a base
   Out: A vector of composites pseudoprime to the provided base
*/

pub(crate) fn strip_pseudo_st<T: FInteger>(pseudos: &[T],base: T) -> Vec<T>{
    let mut veccy = vec![];
    for i in pseudos.iter(){
      if i.sprp(base){
         veccy.push(*i)
      }
    }
    veccy
}

pub(crate) fn filter_st<T: FInteger, F: GenericFilter>(pseudos: &[T], filter_flag: bool) -> Vec<T>{
    let mut veccy = vec![];
    for i in pseudos.iter(){
      if F::filter_check(*i)==filter_flag{
         veccy.push(*i)
      }
    }
    veccy
}

/*
   In: A slice of composites, a lower bound of bases to search, an upperbound of bases to search
   Out: The base that eliminates all composites, Zero if none found in the bound
*/
pub(crate) fn unary_det_st<T: FInteger>(pseudos: &[T], inf: u64, sup: u64) -> u64 {
     if inf >= sup {
      return 0;
     }
    for i in inf..sup {
        for (idx, k) in pseudos.iter().enumerate() {
            if k.sprp(T::from_u64(i)) {
                break;
            }
            if idx == (pseudos.len() - 1) {
                return i;
            }
        }
    }
    return 0u64;
}

/*
   In: A vector of composites, An FIterator that generates bases of some form
   Out: The base that eliminates all composites, Zero if no base exists
*/

pub(crate) fn unary_det_iter_st<T: FInteger, F: FIterator<T>>(pseudos: &[T], iter: F) -> T {
    for i in iter.to_vector(){
        for (idx, k) in pseudos.iter().enumerate() {
            if k.sprp(i) {
                break;
            }
            if idx == (pseudos.len() - 1) {
                return i;
            }
        }
    }
    return T::from_u64(0u64);
}

/*
   In: A list of composites, a lower bound, a upper bound
   Out: The base that eliminates the most composites, along with the number of composites remaining
*/

pub(crate) fn unary_strongest_st<T: FInteger>(pseudos: &[T],inf: u64, sup: u64) -> (u64,u64){
      let mut bound = pseudos.len() as u64; 
      let mut strongest = inf;
      for i in inf..sup{
       let base = T::from_u64(i);
       let mut count = 0u64;
         for j in pseudos.iter(){
           if j.sprp(base){
             count+=1;
           }
         }
         if count < bound{
           bound = count;
           strongest = i;
         } 
      }
      (strongest,bound)
}

/*
     In : A list of composites, the number of random 64-bit bases to test
	 Out: The base that eliminates the most composites, along with the number of composites 
*/
// FIXME return T base of maximum length
pub(crate) fn unary_strongest_st_rand<T: FInteger>(pseudos: &[T],step : usize) -> (u64,u64){
      let mut bound = pseudos.len() as u64;
      	  
      let mut strongest = T::ZERO;
	  
      for _ in 0..step{
	   // No panic as T is guaranteed to be at least 64-bits long	  
       let base = T::gen_k(64).unwrap();
	   
       let mut count = 0u64;
	   // Loop counting how many composites pass the base-SPRP test
         for j in pseudos.iter(){
			 
           if j.sprp(base){
             count+=1;
           }
		   
		   // Terminate loop if the base is weaker than the strongest found
           if count > bound{
			   break;
		   }
		   
         }
		 // Update values if a stronger base is found
         if count < bound{
           bound = count;
           strongest = base;
         } 
		 
      }
	  
      (strongest.to_u64(),bound.to_u64())
}

/*
  In: A vector of composites, An FIterator that generates bases of some form
  Out: The two bases that eliminate the most composites after a deterministic search
  
  Deterministic search is performed by exhaustively checking the every pair combination and 
  selecting the strongest pairing. This is extraordinarily slow, for a faster version see 
  the evolutionary search variant
*/
pub(crate) fn binary_det_iter_st<T: FInteger, F: FIterator<T>>(pseudos: &[T], iter: F) -> (T,T) {
    let base_set = iter.to_vector(); 
    let mut ce_count = u64::MAX;
    let mut lhs = T::from_u64(0u64);
    let mut rhs = T::from_u64(0u64);
    
    for i in &base_set{
     let mut interim = vec![];
    
        for k in pseudos.iter(){
            if k.sprp(*i) {
               interim.push(k)
            }
        }
        
        for j in &base_set{
             let mut count = 0u64;
          for k in &interim{
            if k.sprp(*j){
            count+=1;
            }
          }
         
          if count < ce_count{
           ce_count = count;
           lhs = *i;
           rhs = *j
          }
        }
        
        if ce_count == 0{
          return (lhs,rhs)
        }
    }
    return (lhs,rhs);
}

/*
 In: A list of composites, a lower bound, an upper bound
 Out: A pair of bases that eliminates the most composites after an evolutionary search
 
   Evolutionary search in this instance is achieved by finding the strongest base to the initial compvector 
   then finding it's strongest complementary base, and swapping for 100 iterations then selecting the strongest
   pair
*/

pub(crate) fn binary_evo_st<T: FInteger>(pseudos: &[T], inf: u64, sup: u64) -> (u64,u64){
   let mut strong_lhs = inf; 
   let mut initial_pseudos = strip_pseudo_st(pseudos,T::from_u64(strong_lhs));
   
   let (mut strong_rhs,mut strong_count) =  unary_strongest_st(&initial_pseudos[..],inf,sup);
   let mut lhs = strong_rhs;
        
   for _ in 0..4{
    let lhs_pseudos = strip_pseudo_st(pseudos,T::from_u64(lhs));
    let rhs_res = unary_strongest_st(&lhs_pseudos[..],inf,sup);
    if rhs_res.1 < strong_count{
      strong_count = rhs_res.1;
      strong_lhs = lhs;
      strong_rhs =  rhs_res.0;
      
    }
    lhs = rhs_res.0;
    
   }  
   return (strong_lhs,strong_rhs) 
}

/*
    Select a random base, sequentially search for a strong complement 
	and store the strongest pairing
	
	FIXME 
*/

pub (crate) fn binary_evo_st_rand_partial<T : FInteger>(pseudos: &[T],inf: u64,sup: u64) -> (u64,u64){
	
	let mut strong_lhs = T::gen_k(64).unwrap(); 
    let mut initial_pseudos = strip_pseudo_st(pseudos,strong_lhs);
   
    let (mut strong_rhs,mut strong_count) =  unary_strongest_st(&initial_pseudos[..],inf,sup);
    let mut lhs = strong_rhs;
        
   for _ in 0..100{
	  
      let rand_base = T::gen_k(64).unwrap();
	  let rand_ce = strip_pseudo_st(pseudos,rand_base);
      let (det_complement, complement_ce) = unary_strongest_st(&rand_ce[..],inf,sup);

      if complement_ce < strong_count {
		  strong_count = complement_ce;
		  strong_lhs = rand_base;
		  strong_rhs = det_complement;  
	  }
	  
   }  
   return (strong_lhs.to_u64(),strong_rhs.to_u64())
}
