use crate::Natural;
use crate::filter::*;
use std::io::Read;
use std::io::Write;
use std::io::BufRead;



pub(crate) fn filter_generic<T: Natural, F: GenericFilter>(file: std::fs::File, utf8_flag: bool, filter_flag: bool) -> Vec<T>{
      let mut file_in = std::io::BufReader::new(file.try_clone().unwrap());
      let mut k = Vec::<T>::new();
     
     if utf8_flag{
	     
	        for i in file_in.lines(){
	        
	          let val = T::from_str(&i.unwrap()).unwrap();
	          
	          if F::filter_check(val) == filter_flag{
	             k.push(val);
	          }
	          
	        }
	     }
	     else{
	     
	       let mut interim = vec![0u8;T::BYTE_LENGTH];
	    
	       loop{
              let totalbytes = file_in.read(&mut interim[..]).unwrap();
          
              if totalbytes == 0usize{
                  break;
              }
           
              let val = T::from_bytes(&interim);
     
            if F::filter_check(val)==filter_flag{
	             k.push(val);
	          }   
           }
        }
       k 
      }  
      

pub(crate) fn filter_generic_file<T: Natural, F: GenericFilter>(file: std::fs::File, out: std::fs::File, utf8_flag: bool, filter_flag: bool){
      let mut file_in = std::io::BufReader::new(file.try_clone().unwrap());
      let mut file_out = std::io::BufWriter::new(file.try_clone().unwrap());
     
     if utf8_flag{
	     
	        for i in file_in.lines(){
	        
	          let val = T::from_str(&i.unwrap()).unwrap();
	          
	          if F::filter_check(val) == filter_flag{
	             file_out.write(&val.to_bytes()[..]).unwrap();
	          }
	          
	        }
	     }
	     else{
	     
	       let mut interim = vec![0u8;T::BYTE_LENGTH];
	    
	       loop{
              let totalbytes = file_in.read(&mut interim[..]).unwrap();
          
              if totalbytes == 0usize{
                  break;
              }
           
              let val = T::from_bytes(&interim);
     
            if F::filter_check(val)==filter_flag{
	            file_out.write(&val.to_bytes()[..]).unwrap();;
	          }   
           }
        }
       //k 
      }
      
 pub(crate) fn filter_strong<T: Natural>(file: std::fs::File, utf8_flag: bool, filter_flag: bool,base: T) -> Vec<T>{
    let mut file_in = std::io::BufReader::new(file.try_clone().unwrap());
      let mut k = Vec::<T>::new();
     
     if utf8_flag{
	     
	        for i in file_in.lines(){
	        
	          let val = T::from_str(&i.unwrap()).unwrap();
	          
	          if val.sprp(base) == filter_flag{
	             k.push(val);
	          }
	          
	        }
	     }
	     else{
	     
	       let mut interim = vec![0u8;T::BYTE_LENGTH];
	    
	       loop{
              let totalbytes = file_in.read(&mut interim[..]).unwrap();
          
              if totalbytes == 0usize{
                  break;
              }
           
              let val = T::from_bytes(&interim);
     
            if val.sprp(base)==filter_flag{
	             k.push(val);
	          }   
           }
        }
       k 
      }
