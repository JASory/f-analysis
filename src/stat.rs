/*
trait FloatMap{
  fn to_f64(&self) -> f64;
}

impl FloatMap for u64{
   fn to_f64(&self) -> f64{
      *self as f64
   }
}
*/
/*
macro_rules! floatimpl(
  ($($t:ty),* $(,)*) => {$(
  fn to_f64(&self) -> f64{
     *self as f64
  }
  )*}
);

floatimpl!(u8,i8,u16,i16,u32,i32,u64,i64,i128,u128,f64,f32);
  */

pub struct Stats<T: std::ops::Sub<Output = T> + std::fmt::Display + Copy>{
   min: T,
   max: T,
   mean: f64,
   variance: f64,
}

impl<T: std::fmt::Display + std::ops::Sub<Output = T> + Copy> Stats<T>{


  pub(crate) fn new(min: T,max: T,mean: f64,variance: f64) -> Self{
     Self{min,max,mean,variance}
  }
  
  pub fn min(&self) -> T{
      self.min
   }
   
  pub fn max(&self) -> T{
      self.max
   }
   
  pub fn arith_mean(&self) -> f64{
      self.mean
   }
   
  pub fn variance(&self) -> f64{
      self.variance 
   }
   
  pub fn range(&self) -> T{
      self.max-self.min
   }
   
   pub fn standard_deviation(&self) -> f64{
       self.variance.sqrt()
   }
   
   
}

impl<T:  std::ops::Sub<Output = T> + std::fmt::Display + Copy> std::fmt::Display for Stats<T>{
  fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
     write!(f," min: {}\n max: {}\n range: {}\n mean: {}\n variance: {}\n standard deviation: {}",self.min,self.max,self.range(),self.mean,self.variance,self.standard_deviation())
  }
}
