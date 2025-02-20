use machine_prime::is_prime;

#[derive(Clone)]
pub struct Primes{
   bitvector : Vec<u32>,
}

impl Primes{

  pub fn init(sup: usize) -> Self{

    let ndxlmt = (sup - 3) / 2 + 1;
    let bfsz = ((sup - 3) / 2) / 32 + 1;
    let mut cmpsts = vec![0u32; bfsz];
    let sqrtndxlmt = ((sup as f64).sqrt() as usize - 3) / 2 + 1;

    for ndx in 0..sqrtndxlmt {
        if (cmpsts[ndx >> 5] & (1u32 << (ndx & 31))) == 0 {
            let p = ndx + ndx + 3;
            let mut cullpos = (p * p - 3) / 2;
            while cullpos < ndxlmt {
                unsafe { // avoids array bounds check, which is already done above
	            let cptr = cmpsts.get_unchecked_mut(cullpos >> 5);
	            *cptr |= 1u32 << (cullpos & 31);
                }
                cullpos += p;
            }
        }
    }
    Self{bitvector: cmpsts}
    //Primes {bitvector: cmpsts}
     
  }

  pub fn write(&self,locale: &str) -> Option<()> {
    
    use std::fs::File;
    use std::io::Write;   

     match File::create(locale) {
       Ok(mut out) => {
         let res = self.bitvector.iter().map(|z| z.to_le_bytes()).flatten().collect::<Vec<u8>>();
           match out.write_all(&res) {
              Ok(_) => Some(()),
              Err(_) => None,
             }
          }
        Err(_) => None,
     }    

  }

// Needs to know file length
  pub fn restore(locale: &str) -> Option<Self>{
    let from_bytes = |x : &[u8]| -> u32 {u32::from_le_bytes([x[0],x[1],x[2],x[3]])};
     match std::fs::read(locale) {
            Ok(data) => {
               let mut res = vec![];
               let stride = 4;

               for i in 0..(data.len() / stride){
                  res.push(from_bytes(&data[i * stride..(i + 1) * stride]));
               }
             return Some(Primes{bitvector: res});
             } 
            Err(_) => None,
        }

  }
  
  
  pub fn generate_or_restore(sup: usize) -> Self{
	  
	  let filename = "primes_".to_string() + &sup.to_string();
	  
	  let mut plist : Primes;
	  
	  match std::fs::File::open(filename.clone()){
	        Ok(_) => plist = Primes::restore(&filename.clone()).unwrap(),
            Err(_)	=> {
				plist = Primes::init(sup);
			    plist.write(&filename).unwrap(); 
			}		
	  }
	  plist
  }


  pub fn check(&self, x: u64) -> bool{
      // 
     //if x > (self.bitvector.len()/64) as u64{
       return is_prime(x)
     //}
     /*
     let odd_reduce = (x - 1) >> 1;
     let idx = odd_reduce / 32;
     let shift = odd_reduce & 31;

    ((self.bitvector[idx as usize] >> shift) & 1) == 1
*/
  }
  
  pub fn to_vector(&self) -> Vec<u64>{
         let ndxlmt = self.bitvector.len()*32 - 1;
         
         (0..ndxlmt as isize)
        .into_iter()
        .filter_map(move |i| {
           /* if i < 0 {
                Some(2)
            } else */ if self.bitvector[i as usize >> 5] & (1u32 << (i & 31)) == 0 {
                Some((i + i + 3) as u64)
            } else {
                None
            }
        }).collect::<Vec<u64>>()
         
  }

  pub fn iter(&self) -> Box<dyn Iterator<Item=u64> + '_>{
  
         let ndxlmt : isize = (self.bitvector.len()*32) as isize-1;
          Box::new((0 .. ndxlmt).into_iter().filter_map(move |i| {
                //if i < 0 { Some(3) } else {
                    if self.bitvector[i as usize >> 5] & (1u32 << (i & 31)) == 0 {
                        Some((i + i + 3) as u64) } else { None } //}
    }))
  }

 
}

