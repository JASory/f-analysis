use crate::enums::{MEMORY_MAX,UTF8_FLAG,AUTO_FLAG};
use crate::CompVector;
use crate::Natural;

/// Convenience macro for initialising CompVectors
///
/// Example implementations, only the simplest implementations are provided
/// ```
///    // Initialise a empty CompVector that can be assigned values later
///   let empty : CompVector<u64> = cvec![];
///    // Allocate 4 elements into a CompVector
///   let small = cvec![6u64,9,12,15];
///    // Unchecked file input for binary, note this is just an initialisation, no loading of the file is performed
///   let filein = cvec!["filename.bin";u64];
/// ```
#[macro_export]
 macro_rules! cvec{
    // empty vector
    () => {
    CompVector::new()
    };
    // cvec![]
   ( $( $x:expr ),* ) => {
      {
        let mut tmpvec = CompVector::new();
        $(
          tmpvec.push($x);
        )*
        tmpvec
      }
   };
   // string input for file
    ($str: tt; $ty: tt) => {
      CompVector::<$ty>::from_file($str).unwrap()
    };
 }
 
 pub struct Constructor<T: Natural>{
    mem_max : u64, 
    utf_flag: bool,
    filename: String,
    vector: Vec<T>,
    auto_flag: bool,
    load_flag: bool,
 }
 
 impl<T: Natural> Constructor<T>{
 
    pub (crate) fn new() -> Self{
       Self{
        mem_max: MEMORY_MAX,
        utf_flag: UTF8_FLAG,
        filename: String::new(),
        vector: Vec::<T>::new(),
        auto_flag: AUTO_FLAG,
        load_flag: false,
       }
    }
    
    pub fn file(mut self, filename: &str) -> Self{
       if self.vector.len() > 0{
          panic!("Already assigned a vector");
       }
       self.filename = filename.to_string();
       self
    }
    
    pub fn memory(mut self, mem_max : u64) -> Self{
       self.mem_max = mem_max;
       self
    }
    /// Configures to immediate load to memory, note this does not make the vector autoloading
    pub fn load(mut self) -> Self{
       self.load_flag = true;
       self
    }
    /// Configures to autoload, note this does not immediately load the vector to RAM
    /// Some function that autoloads has to be called first. 
    pub fn auto_load(mut self, aflag: bool) -> Self{
       self.auto_flag = aflag;
       self
    }
    
    pub fn utf8(mut self, uflag: bool) -> Self{
       self.utf_flag = uflag;
       self
    }
    
    pub fn vector(mut self, vect: Vec<T>) -> Self{
       if self.filename.len() > 0{
          panic!("Already assigned a file")
       }
       self.vector = vect.clone();
       self
    }
    
    pub fn construct(self) -> CompVector<T>{
       let mut res = CompVector::<T>::new();
       res.set_memory_max(self.mem_max);
       if self.utf_flag{
         res.set_utf8();
       }
       
       if !self.auto_flag{
           res.set_manual();
       }
       
       if self.vector.len() == 0 && self.filename.len() == 0{
         panic!("No file or vector provided");
       }
       
       if self.vector.len() > 0{
          for i in self.vector.iter(){
            if i.is_prime(){
               panic!("Prime {}: Primes are not allowed",i);
            }
            res.push(*i);
          }
          return res;
       }
       else{
         res.set_file(&self.filename);
         if self.load_flag{
            return res.load_to_memory().unwrap();
         }
         return res
       }
    }
 }
