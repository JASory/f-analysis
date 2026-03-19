pub fn create_file(file: &str){
    std::fs::File::create(file).unwrap();
}

pub fn appender(data: &str, file: &str){
   use std::io::{Write,Seek,SeekFrom};
   let mut file = std::fs::OpenOptions::new().write(true).open(file).unwrap();
   file.seek(SeekFrom::End(0));
   file.write(&data.as_bytes());
}

pub fn write_bounds(x: u64, y: u64, file: &str){
   use std::io::Write;
   let mut fout = std::fs::File::create(file).unwrap();//std::fs::OpenOptions::new().write(true).open(file).unwrap();
   let data = x.to_string() + " " + &y.to_string();
   fout.write(&data.as_bytes());
}

pub fn write_single(x: u64, file: &str){
   use std::io::Write;
   let mut fout = std::fs::File::create(file).unwrap();//std::fs::OpenOptions::new().write(true).open(file).unwrap();
   let data = x.to_string();
   fout.write(&data.as_bytes());
}

//pub fn update_single


pub fn load_idx(input: &str) -> Option<u64>{
      let strin = std::fs::read_to_string(input).unwrap();
      let k = strin.trim();
      Some(k.parse::<u64>().unwrap())
}

// 
pub fn load_bounds(input: &str) -> Option<(u64,u64)>{
   let strin = std::fs::read_to_string(input).unwrap();
   
   let args = strin.split_whitespace().collect::<Vec<&str>>();
   match args.len(){
    1 => {
      match args[0].trim().parse::<u64>(){
        Ok(x) => return Some((0,x)),
        Err(_) => return None,
      }
    }
    2 => {
     match (args[0].trim().parse::<u64>(),args[1].trim().parse::<u64>()){
      (Ok(x),Ok(y)) => Some((x,y)),
      _=> None,
     }
    }
    _ => return None,
   }
}

