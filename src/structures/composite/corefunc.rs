use crate::structures::{Primes,Point,BaseVector,CompVector};
use crate::fermat::{FInteger,IntSeq,FIterator};

use crate::{HashTable};
use crate::filter::{StrongFermat,Coprime,GenericFilter};
use std::io::{Read,BufRead,Write};
use crate::compeval::{vector::*,file::*};
use crate::search::{hash_search,unary_ht_par,strip_pseudo_par,strip_pseudo_st,binary_evo_st,binary_det_iter_st,binary_evo_par,binary_evo_st_rand_partial, unary_strongest_st,unary_strongest_par,unary_strongest_rand_par,exhaustive_par,exhaustive_rand_par};
use crate::io::write::format_block;
use crate::result::FResult;
use crate::compconfig::{Search,AUTO_FLAG,UTF8_FLAG,MEMORY_MAX};

impl<T: FInteger> Clone for CompVector<T>{

   fn clone(&self) -> Self{
   
    match &self.file{
      Some(filey) => Self{
           file: Some(filey.try_clone().unwrap()),
       elements: self.elements.clone(),
       memory_max: self.memory_max,
       utf8_flag: self.utf8_flag,
       auto_flag: self.auto_flag,
       },
      None => 
         Self{
           file: None,
       elements: self.elements.clone(),
       memory_max: self.memory_max,
       utf8_flag: self.utf8_flag,
       auto_flag: self.auto_flag,
       }
      }
      
    }
     
}

impl<T: FInteger> std::fmt::Display for CompVector<T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    
      match &self.file{
       Some(_) => {
       
         let mem_est = (self.len()*T::BYTE_LENGTH/3) as u64;
         
         if mem_est > self.memory_max {
           return write!(f,"Stored in file");
         }
         
         match self.load_to_memory(){
               FResult::Value(p) => {
                   let q = format_block::<4,T>(&p.elements);
                   write!(f, "{}", q)
               }
               _ => write!(f,"Memory Exceeded"),
         }
       }
       None => {
        let q = format_block::<4, T>(&self.elements);
        write!(f, "{}", q)
       }
       
      }
   }
}


impl<T: FInteger> CompVector<T>{

  pub fn new() -> Self{
      Self{
      file: None,
      elements: Vec::<T>::new(),
      memory_max: MEMORY_MAX,
      utf8_flag: UTF8_FLAG,
	  auto_flag: AUTO_FLAG
     }
  }
  
  pub fn is_assigned(&self) -> bool{
    match self.file{
      Some(_) => true,
      None => {
        if self.elements.len() > 0{
         return true
        }
        return false
      }
    }
  }
  
  // Returns true if already loaded to memory
  // Returns false if kept in file
  pub fn is_loaded(&self) -> bool{
     match self.file{
      Some(_) => false,
      None => true,
     }
  }
  
  
  pub fn set_file(&mut self,locale: &str){
     if self.is_assigned(){
       panic!("Already assigned a value")
     }
    self.file = Some(std::fs::OpenOptions::new().read(true).write(true).open(locale).unwrap()); 
    
  }
  
  
 /// Set  CompVector to be equal to vector
  pub fn set_vector(&mut self, el: Vec<T>){
     if self.is_assigned(){
       panic!("Already assigned a value")
     }
      self.elements = el;
  }
  
  /// Set to read and write in binary 
  pub fn set_binary(&mut self){
      self.utf8_flag = false;
  }
  
  /// Set to read and write utf-8 format
  pub fn set_utf8(&mut self){
     self.utf8_flag = true;
  }
  
  /// Sets to automatically load any data stored in the file to volatile memory (RAM)
  pub fn set_auto(&mut self){
     self.auto_flag = true;
  }
  
  /// Sets to not load data  
  pub fn set_manual(&mut self){
      self.auto_flag = false;
  }
  
  /// Sets the limit on how large CompVector can be 
  pub fn set_memory_max(&mut self, bound: u64){
          self.memory_max = bound;
  }
  
  /// Number of elements stored in CompVector, this evaluates for CFile as well as CVec
  // FIXME only supports binary files
  pub fn len(&self) -> usize{
      match &self.file{
        Some(x) => (x.metadata().unwrap().len() as usize)/T::BYTE_LENGTH,
        None => self.elements.len(),
      }
  }
  
  pub fn satisfies_memory_bound(&self) -> bool{
      match &self.file{
       Some(x) => {
         let mut len = x.metadata().unwrap().len() as u64;
        
         if self.utf8_flag{
           len /=2;
         }
         
         if self.memory_max < len{
           return false;
         }
         
         return true
       }
       
       None => return true,
      }
  }
  
    /// Initialises from file 
  pub fn from_file(filename : &str) -> FResult<Self>{
	  
	       match std::fs::OpenOptions::new().read(true).write(true).open(filename){
             Ok(x)=> FResult::Value( Self{
				            file: Some(x),
							elements: Vec::<T>::new(),
						  memory_max: MEMORY_MAX,
							utf8_flag: UTF8_FLAG,
							auto_flag: AUTO_FLAG
							}
						  ),
             Err(file_error) => FResult::IOError(file_error) ,
           }
      }
      
  pub(crate) fn from_file_internal(f: std::fs::File, mm_flag: u64,u_flag: bool, a_flag:bool) -> Self{
       Self {
                file: Some(f),
            elements: Vec::<T>::new(),
          memory_max: mm_flag, 
           utf8_flag: u_flag,
           auto_flag: a_flag,
           }
  }
  
  pub (crate) fn from_vector_internal(comp: Vec<T>,memory_max: u64, utf8_flag: bool, auto_flag: bool) -> Self{
      Self{
           	  file: None, 
		  elements: comp,
	    memory_max: memory_max,
		 utf8_flag: utf8_flag,
		 auto_flag: auto_flag,
		 }

  }
  
  pub fn to_vector(&self) -> Vec<T>{
      self.elements.clone()
  }
	  
	/// Initialises from vector  
  pub fn from_vector(comp: Vec<T>) -> Self{
  
      Self::from_vector_internal(comp,MEMORY_MAX,UTF8_FLAG,AUTO_FLAG)
      }
      
 /// Writes to file returning a CompVector handling files   
  // FIXME Map file to file
  pub fn to_file(&self, filename: &str)  -> FResult<Self>{
     
     match std::fs::File::create_new(filename){
     
       Ok(x)=> {
  
           let mut wrtr = std::io::BufWriter::new(x.try_clone().unwrap());
           
         if self.utf8_flag == true{
       
           for i in self.elements.iter(){
              let out_str = i.to_string()+"\n"; 
        
              wrtr.write(out_str.as_bytes()).unwrap();
         }
       }
       
       if self.utf8_flag == false{
       
          for i in self.elements.iter(){
              wrtr.write(&i.to_bytes()[..]).unwrap();
          }
       }
       
       wrtr.flush().unwrap();
       
       return FResult::Value(Self{ 
                      file: Some(x.try_clone().unwrap()),
                  elements: Vec::<T>::new(),
                memory_max: self.memory_max,
                utf8_flag: self.utf8_flag,
                auto_flag: self.auto_flag
                })
     },
             Err(file_error) =>  FResult::IOError(file_error) ,
     }
  }
  
        /// loads file into RAM if possible, this is preferred as it allows much faster datahandling, including parallel evaluation
	   /// # MemoryExceeded
	   /// If file size is greater than 1 GiB, then returns None
	   /// # ReadError
	   /// Unable to read from file
	   /// # FileDNE
	   /// File does not exist
  pub fn load_to_memory(&self) -> FResult<Self>{
  
   match &self.file{
      Some(filey) => {
      
        if !self.satisfies_memory_bound(){
		  return FResult::MemoryExceeded;
	    }
	    
	    let mut r = std::io::BufReader::new(filey.try_clone().unwrap());
	    
	    let mut veccy : Vec<T> = Vec::new();
	    
	     if self.utf8_flag{
	     
	        for i in r.lines(){
	          match i {
	           Ok(x) => {
	               // FIXME handle unwrapping
	              veccy.push(T::from_str(&x).unwrap())
	             
	           }
	           Err(err_message) => return FResult::IOError(err_message),
	          }
	        }
	    
	        return FResult::Value(Self{
           	  file: None, 
		  elements: veccy,
	    memory_max: self.memory_max,
		 utf8_flag: self.utf8_flag,
		 auto_flag: self.auto_flag,
		 });

	     }
	     else{
	     
	       let mut interim = vec![0u8;T::BYTE_LENGTH];
	    
	       loop{
	       
	        match r.read(&mut interim[..]){
	           Ok(totalbytes) => {
	             if totalbytes == 0usize{
	                break;
	             }  
               let num = T::from_bytes(&interim);
               
               veccy.push(num);
	           }
	           Err(err_message) => return FResult::IOError(err_message),
	        }
        }

        return FResult::Value(Self{
           	  file: None, 
		  elements: veccy,
	    memory_max: self.memory_max,
		 utf8_flag: self.utf8_flag,
		 auto_flag: self.auto_flag,
		 });
      }
	  
      },
      None => FResult::FileDNE,
   }

  }    
  
    /// Clears all values from CompVector    
  pub fn clear_memory(&mut self){
      self.file = None;
      self.elements.resize(0,T::ZERO);
  }
  
  
    pub fn write_vector_internal(&mut self, mut output: &std::fs::File){
       use std::io::prelude::*;
       use std::io::SeekFrom;
       
       output.seek(SeekFrom::Start(0));
       let mut wrtr = std::io::BufWriter::new(output);
       
       if self.utf8_flag{
          for i in self.elements.iter(){
            let x = i.to_string()+"\n";
          
            wrtr.write(&x.as_bytes()[..]).unwrap();
          }
          wrtr.flush();
       }
       else if self.utf8_flag == false{
       
       for i in self.elements.iter(){
          wrtr.write(&i.to_bytes()[..]).unwrap();
       }
       wrtr.flush();
       }
  }
  
 
  
  pub fn mut_vector_op(&mut self, F: &dyn Fn(&mut [T]) -> ())-> FResult<T>{
       match &self.file{
         Some(x) => {
           match self.load_to_memory(){
             FResult::Value(mut interim) => {
            
               F(&mut interim.elements);
              
            interim.write_vector_internal(x);
              
               return FResult::Success;
             }
             FResult::MemoryExceeded => FResult::MemoryExceeded,
             _=> FResult::MemoryExceeded,
           }
         }
         None => {
           F(&mut self.elements);
           return FResult::Success;
         }
       }
  }
  
  // FIXME handle other Result values
  pub fn load_eval<K: Clone>(&self, func: &dyn Fn(Self) -> FResult<K>) -> FResult<K>{
      if self.auto_flag && !self.is_loaded(){
        match self.load_to_memory(){
        FResult::MemoryExceeded => FResult::MemoryExceeded,
         FResult::Value(x) => func(x),
         FResult::IOError(error_mssg) =>   FResult::IOError(error_mssg),
         _=> FResult::MemoryExceeded, 
        }
      }
      else if !self.auto_flag && !self.is_loaded(){
         return FResult::NotSupported
      }
      else{
         func(self.clone())
      }
  }
  
  // FIXME handle other Result values
  pub fn load_eval_ref<K: Clone>(&self, func: &dyn Fn(&Self) -> FResult<K>) -> FResult<K>{
      if self.auto_flag && !self.is_loaded(){
        match self.load_to_memory(){
        FResult::MemoryExceeded => FResult::MemoryExceeded,
         FResult::Value(x) => func(&x),
         FResult::IOError(error_mssg) =>   FResult::IOError(error_mssg),
         _=> FResult::MemoryExceeded, 
        }
      }
      else if !self.auto_flag && !self.is_loaded(){
         return FResult::NotSupported
      }
      else{
         func(self)
      }
  }
  
  
  
  // FIXME handle autoloading
  pub fn iter(&self) -> FResult<std::slice::Iter<T>>{
 
      match &self.file{
        Some(_) => FResult::NotSupported,
        None => FResult::Value(self.elements.iter())
      }
  }
  
  
  }

