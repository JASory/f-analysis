use crate::structures::{Primes,Point,BaseSeq,CompVector};
use crate::fermat::{FInteger,IntSeq,FIterator};

use crate::{HashTable};
use crate::filter::{StrongFermat,Coprime,GenericFilter};
use std::io::{Read,BufRead,Write};
use crate::compeval::{vector::*,file::*};
use crate::search::{hash_search,unary_ht_par,strip_pseudo_par,strip_pseudo_st,binary_evo_st,binary_det_iter_st,binary_evo_par,binary_evo_st_rand_partial, unary_strongest_st,unary_strongest_par,unary_strongest_rand_par,exhaustive_par,exhaustive_rand_par};
use crate::io::write::format_block;
use crate::result::FResult;
use crate::compconfig::{Search,AUTO_FLAG,UTF8_FLAG,MEMORY_MAX};
use crate::Constructor;

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

impl<T: FInteger> std::convert::From<Vec<T>> for CompVector<T>{

      fn from(x: Vec<T>) -> Self{
         Self::from_vector_internal(x,MEMORY_MAX,UTF8_FLAG,AUTO_FLAG)
      }
}


impl<T : FInteger> FromIterator<T> for CompVector<T> {

    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut out = Self::new();
        
        for i in iter{
            out.push(i)
         }
         out 
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
  
  pub fn constructor() -> Constructor<T>{
      Constructor::<T>::new()
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
  
  
  pub fn set_file(&mut self,locale: &str) -> FResult<T>{
     if self.is_assigned(){
       return FResult::Err("Already assigned a value");
     }
    self.file = Some(std::fs::OpenOptions::new().read(true).write(true).open(locale).unwrap()); 
    FResult::Success
  }
  
  
 /// Set  CompVector to be equal to vector
  pub fn set_vector(&mut self, el: Vec<T>) -> FResult<T>{
     if self.is_assigned(){
       return FResult::Err("Already assigned a value");
     }
      self.elements = el;
      
      FResult::Success
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
  
  pub fn push(&mut self, el: T){
      self.elements.push(el);
  }
  
  /// Add from a collection that implements IntoIterator
  pub fn append_collection<F: IntoIterator<Item=T>>(&mut self, otra: F){
         for i in otra.into_iter(){
            self.push(i);
         }
  }
  
  /// Appends all elements to self  
  pub fn append(&mut self, otra: &mut Self){
      self.elements.append(&mut otra.elements)
  }
  
  pub fn satisfies_memory_bound(&self) -> FResult<T>{
      match &self.file{
       Some(x) => {
         let mut len = x.metadata().unwrap().len() as u64;
        
         if self.utf8_flag{
           len /=2;
         }
         
         if self.memory_max < len{
           return FResult::MemoryExceeded(len as usize);
         }
         
         return FResult::Success;
       }
       
       None => return FResult::Success,
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
	  
	/// Initialises from vector slightly faster than std::convert::From  
  pub fn from_vector(comp: Vec<T>) -> Self{  
      Self::from_vector_internal(comp,MEMORY_MAX,UTF8_FLAG,AUTO_FLAG)
      }
      
 /// Writes to file returning a CompVector handling files   
  // FIXME Map file to file
  pub fn to_file(&self, filename: &str)  -> FResult<Self>{
     
     match std::fs::File::create(filename){
     
       Ok(x)=> {
  
           let mut wrtr = std::io::BufWriter::new(x.try_clone().unwrap());
           
         if self.utf8_flag == true{
       
           for i in self.elements.iter(){
              let out_str = i.to_string()+"\n"; 
        
              match wrtr.write(out_str.as_bytes()){
                Ok(_) => (),
                Err(message) => return FResult::IOError(message),
              }
         }
       }
       
       if self.utf8_flag == false{
       
          for i in self.elements.iter(){
          
             match wrtr.write(&i.to_bytes()[..]){
               Ok(_) => (),
               Err(message) => return FResult::IOError(message),
             }
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
  
        /// Loads file into RAM if possible, this is preferred as it allows much faster datahandling, including parallel evaluation
	   /// # MemoryExceeded
	   /// If file size is greater than 1 GiB, then returns None
	   /// # ReadError
	   /// Unable to read from file
	   /// # FileDNE
	   /// File does not exist
  pub fn load_to_memory(&self) -> FResult<Self>{
  
   match &self.file{
      Some(filey) => {
      
      
        match self.satisfies_memory_bound(){
           FResult::MemoryExceeded(mem) => {
                   return FResult::MemoryExceeded(mem)
                   },
           _ => (),
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
  
  
pub(crate) fn write_vector_internal(&mut self, mut output: &std::fs::File){
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
  
 
  
  pub fn mut_vector_op(&mut self, op: &dyn Fn(&mut [T]) -> ())-> FResult<T>{
       match &self.file{
         Some(x) => {
           match self.load_to_memory(){
             FResult::Value(mut interim) => {
            
               op(&mut interim.elements);
              
            interim.write_vector_internal(x);
              
               return FResult::Success;
             }
             FResult::MemoryExceeded(mem) => FResult::MemoryExceeded(mem),
             _=> FResult::Critical,
           }
         }
         None => {
           op(&mut self.elements);
           return FResult::Success;
         }
       }
  }
  
  pub fn sort(&mut self) -> FResult<T>{
      self.mut_vector_op(&<[T]>::sort)
  }
  
  // FIXME handle other Result values
  pub fn load_eval<K: Clone>(&self, func: &dyn Fn(Self) -> FResult<K>) -> FResult<K>{
      if self.auto_flag && !self.is_loaded(){
      
        match self.load_to_memory(){
        
        FResult::MemoryExceeded(mem) => FResult::MemoryExceeded(mem),
         FResult::Value(x) => func(x),
         FResult::IOError(error_mssg) =>   FResult::IOError(error_mssg),
         _=> FResult::Critical, 
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
        
         FResult::MemoryExceeded(mem) => FResult::MemoryExceeded(mem),
         FResult::Value(x) => func(&x),
         FResult::IOError(error_mssg) =>   FResult::IOError(error_mssg),
         _=> FResult::Critical, 
        }
      }
      else if !self.auto_flag && !self.is_loaded(){
         return FResult::NotSupported
      }
      else{
         func(self)
      }
  }
  
  pub fn set_union(&self, otra: &Self) -> FResult<Self>{
      match (&self.file,&otra.file){
      (&None,&None) => {
         let mut hash_union = std::collections::HashSet::<T>::new();
         
         for i in self.elements.iter(){
             hash_union.insert(*i);
         }
         for j in otra.elements.iter(){
           hash_union.insert(*j);
         }
         let interim = hash_union.drain().collect::<Vec<T>>();
         FResult::Value(Self::from_vector_internal(interim,self.memory_max,self.utf8_flag,self.auto_flag))
      }
      _=> FResult::NotSupported,
      }
  }
  
  pub fn make_set(&self) -> FResult<Self>{
      match &self.file{
        &None => {
          let mut hash_union = std::collections::HashSet::<T>::new();
         
         for i in self.elements.iter(){
             hash_union.insert(*i);
         }
         let interim = hash_union.drain().collect::<Vec<T>>();
         FResult::Value(Self::from_vector_internal(interim,self.memory_max,self.utf8_flag,self.auto_flag))
        }
        _=> FResult::NotSupported,
      }
  }
  
  // FIXME handle autoloading
  pub fn iter(&self) -> FResult<std::slice::Iter<T>>{
 
      match &self.file{
        Some(_) => FResult::NotSupported,
        None => FResult::Value(self.elements.iter())
      }
  }
  
  // FIXME handle autoloading
  pub fn into_iter(&self) -> FResult<std::vec::IntoIter<T>>{
 
      match &self.file{
        Some(_) => FResult::NotSupported,
        None => FResult::Value(self.elements.clone().into_iter())
      }
  }
  
  pub fn reducible<F: FInteger>(&self) -> bool{
  
       let maxbits = T::BYTE_LENGTH*8;
       
      for i in self.iter().unwrap(){
         if i.msb() > maxbits{
            return false;
         }
      }
      return true;
  }
  
  }

