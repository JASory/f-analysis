use crate::fermat::{FInteger,FIterator};



pub(crate) fn strip_pseudo_st<T: FInteger>(pseudos: &[T],base: T) -> Vec<T>{
    let mut veccy = vec![];
    for i in pseudos.iter(){
      if i.sprp(base){
         veccy.push(*i)
      }
    }
    veccy
}

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
  Binary Deterministic search singlethread
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
 Start with a value, find the strongest counterpart to it
*/

pub(crate) fn binary_evo_st<T: FInteger>(pseudos: &[T], inf: u64, sup: u64) -> (u64,u64){
   let mut strong_lhs = inf; 
   let mut initial_pseudos = strip_pseudo_st(pseudos,T::from_u64(strong_lhs));
   
   let (mut strong_rhs,mut strong_count) =  unary_strongest_st(&initial_pseudos[..],inf,sup);
   let mut lhs = strong_rhs;
         println!("base 1: {} base 2: {} count {}",strong_lhs,strong_rhs,strong_count);
   for _ in 0..100{
    let lhs_pseudos = strip_pseudo_st(pseudos,T::from_u64(lhs));
    let rhs_res = unary_strongest_st(&lhs_pseudos[..],inf,sup);
    if rhs_res.1 < strong_count{
      strong_count = rhs_res.1;
      strong_lhs = lhs;
      strong_rhs =  rhs_res.0;
      println!("base 1: {} base 2: {} count {}",strong_lhs,strong_rhs,strong_count);
    }
    lhs = rhs_res.0;
    
   }  
   return (strong_lhs,strong_rhs) 
}

