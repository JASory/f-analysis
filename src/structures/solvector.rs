use crate::{FResult,CompVector,Natural,Persistent,cvec,filter::GenericFilter,Stats};


//CompVector with the number of fermat solutions to each integer
#[derive(Clone)]
pub struct SolVector<T: Natural>{
  elements: Vec<(T,f64)>,
}



fn variance(x: &[f64]) -> f64{
   let mut sum= 0f64;
   let mut sumsqr = 0f64;
   for i in x.iter(){
     sum += i;
     sumsqr+=i*i;
   }
   let n = x.len() as f64;
   sumsqr/n - (sum/n).powi(2)
}
/**/
//mean of x^2 - (mean of x)^2

impl<T: Natural> SolVector<T>{

  fn new(elements: Vec<(T,f64)>)-> Self{
     Self{elements}
  }
  
  pub fn len(&self) -> usize{
      self.elements.len()
  }
  
  pub fn iter(&self) -> std::slice::Iter<'_, (T,f64)>{
     self.elements.iter()
  }
  // Adjusts the perfect powers, which are evaluated to be 1, compare against p^k rather than phi(p^k)
  // This makes gathering statistics on pseudoprime strength more accurate
  pub fn adjust_power(&mut self){
      for i in self.elements.iter_mut(){
         if i.1==1f64{
            let fctr = i.0.factor().unwrap();
            if fctr.factors.len()==1{
               let quo = i.0.euclidean(fctr.factors[0]).0;
               i.1=quo.to_float().recip();
            }
         }
      }
  }
  
  pub fn interval_solution_count(&self, inf: f64, sup: f64) ->  u64{
       let mut count = 0;
       for i in self.iter(){
          if i.1 > inf && i.1 <= sup{
             count+=1;
          }
       }
       count
  }
   // FIXME only works with volatile memory, not file's
   // change to borrow
  pub fn from_comp(x: &CompVector<T>) -> FResult<Self>{
      let mut elements = vec![];
      for i in x.iter().unwrap(){
         elements.push((*i,i.fermat_solution_ratio().1 as f64));
      }
      FResult::Value(Self::new(elements))
  }
  
  pub fn to_comp(&self) -> CompVector<T>{
     let mut res = cvec![];
     for i in self.elements.iter(){
       res.push(i.0);
     }
     res
  }
  
  pub fn nfilter_generic<F: GenericFilter>(&self) -> Self{
      let mut v = vec![];
      for i in self.elements.iter(){
          if !F::filter_check(i.0){
            v.push(*i);
          }
      }
      Self::new(v)
  }
  
  pub fn filter_generic<F: GenericFilter>(&self) -> Self{
      let mut v = vec![];
      for i in self.elements.iter(){
          if F::filter_check(i.0){
            v.push(*i);
          }
      }
      Self::new(v)
  }
  
  pub fn hash_search(&self, dimen: usize, iterations: usize) -> u32{
      let shift = (((1<<32)/dimen).trailing_zeros()+1) as usize ;
      let mut minvariance = f64::MAX;
      let mut minmultiplier = 0;
     for i in 0..iterations{
         let multiplier = u64::gen_k(32).unwrap() as u32;
         let mut values = vec![0f64;dimen];
         
         for i in self.iter(){
            let idx = i.0.hash_shift(shift,multiplier);
            values[idx as usize]+=i.1;
         }
         let var = variance(&values[..]);
         if var < minvariance{
           minmultiplier = multiplier;
           minvariance = var;
         }
         
     }
     return minmultiplier
  }
  
  /*
  pub fn stats(&self) -> Stats<T>{
     for i in self.iter(){
        
     }
  }
  */
  
  pub fn solution_stat(&self) -> Stats<f64>{
       let mut min = f64::MAX;
       let mut max = f64::MIN;
       let mut sum = 0f64;
       let mut sumsqr = 0f64;
       
       for i in self.iter(){
         let s = i.1;
         
         if s > max{
           max=s;
         }
         if s < min{
           min=s;
         }
    
       sum += s;
       sumsqr+=s*s;
      }
      let n = self.len() as f64;
      let mean = sum/n;
      Stats::new(min,max,mean,sumsqr/n - mean.powi(2))
       
  }
  
    // Randomly generate a multiplier 
    // Partition according to that multiplier summing up the solution ratios
    // select the array with the minimum variance
}


impl<T: Natural> Persistent for SolVector<T>{

    fn to_persistent(&self, locale: &str) -> FResult<()>{
    
        use std::fs::File;
        use std::io::Write;

        match File::create(locale) {
            Ok(mut out) => {
                let mut w = std::io::BufWriter::new(out);
                
                for i in self.elements.iter(){
                   match w.write(&i.0.to_bytes()[..]){
                     Ok(_) => (),
                     Err(message) => return FResult::IOError(message),
                   }
                   match w.write(&i.1.to_le_bytes()[..]){
                     Ok(_) => (),
                     Err(message) => return FResult::IOError(message),
                   }
                }
            }
            Err(message) => return FResult::IOError(message),
        }
        FResult::Success
 }

    /// Load from file
    fn from_persistent(locale: &str) -> FResult<Self>{
        use std::fs::File;
        use std::io::Read;
          let mut res = vec![];
          //println!("Called from persistent");
        match File::open(locale) {
            Ok(mut out) => {
                let mut r = std::io::BufReader::new(out);
                let mut interim = vec![0u8;T::BYTE_LENGTH];
                let mut f64buf = [0u8;8];
                
                loop {
                let mut pair = (T::ZERO,0f64);
                match r.read(&mut interim[..]){
                   Ok(totalbytes) => {
                     if totalbytes == 0{
                        break; // FIXME throw error since we must finish reading a float
                        }
                        pair.0=T::from_bytes(&interim);//res.push(u64::from_bytes(&interim));
                     }
                   Err(message) => return FResult::IOError(message),
                }
               match r.read(&mut f64buf[..]){
                  Ok(floatbytes) => {
                     if floatbytes ==0{
                        break;
                     }
                     pair.1=f64::from_le_bytes(f64buf);
                  }
                  Err(message) => {return FResult::IOError(message);},

               }
               res.push(pair);
               
               }
               return FResult::Value(Self::new(res))
               }
             Err(message) => {println!("failed with {}",message);return FResult::IOError(message);},  
              } 
      }

    }

