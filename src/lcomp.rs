use crate::fermat::FInteger;
use crate::CompVector;
use crate::filter::*;
use std::io::Read;

// Large Binary vector stored in file
struct LCompVector<T: FInteger>{
  filename: String,
  apparition: std::marker::PhantomData<T>,
}

impl<T: FInteger> LCompVector<T>{
   fn new(file: String) -> Self{
      Self{filename:file, apparition: std::marker::PhantomData::<T>}
   }

   fn filter_sprp<F: StrongFermat>(&self) -> CompVector<T>{
      let mut file = std::io::BufReader::new(std::fs::File::create(self.filename.as_str()).unwrap());
      let mut interim = vec![0u8;T::BYTE_LENGTH];
      let mut k = CompVector::<T>::new();
      loop{
       let totalbytes = file.read(&mut interim[..]).unwrap();
       if totalbytes == 0{
         break;
        }
       k.append(T::from_bytes(&interim));
      }
     k    
  }
}



