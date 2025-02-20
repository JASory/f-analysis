use std::sync::Arc;


trait PrimeValuation{
  const BOOLEAN : bool
}

impl PrimeValuation for Prime{
   const BOOLEAN : bool = true;
}


impl PrimeValuation for Composite{
   const BOOLEAN : bool = false;
}

/* 
  Primes
*/
//  Only works for perfect powers of two 2^{2n}
fn mul_promoter(p: u64, seg: u64) -> u64{
         if seg == 0{
           return p*p;
         }
         let num = (seg-p)/(p*2);
         ((num+1)*(p*2)+p)%(seg)
}

fn sub_sieve(primes: Arc<Vec<u64>>, seg_size: u64,start: u64, stop: u64, func : &'static (dyn Fn(u64) -> bool + Sync), flag: bool)-> Vec<u64>{
    let mut sieve = Vec::<bool>::with_capacity(seg_size as usize);
    let mut veccy = vec![];
   let mut count = 0u64;  
   for seg in start..stop{
   
   let low = seg*seg_size;
   let mut n = low;
   if n&1 == 0{
     n+=1;
   }
   //if low == 0{
   // n=3;
   //}
   
     sieve.truncate(0);
     sieve.resize(seg_size as usize,true); // allocate with true values
     
     let mut high : u64 = low + seg_size -1;
     let max_prime = (high as f64).sqrt() as u64;

     
     for i in primes.iter(){// sieve current segment
         if *i > max_prime{
            break;
         }
         let mut j = mul_promoter(*i,low);
         let k = *i*2;     
         loop {
           if j >= seg_size{
             break;
           }
           sieve[j as usize] = false;
           j+=k;
         }
         
     }
     
     loop{
       if n > high{
         break;
       }
       
       if sieve[(n-low) as usize]==flag{
         if func(n){
            veccy.push(n);
         }
       }
       n+=2;
     }
     
   }
   veccy
}

fn para_sieve(sup: u64) -> Vec<u64>{
   let tc = 2;
   let plist : Arc<Vec<u64>> = Arc::new(segmented_sieve((sup as f64).sqrt() as u64));
   
   let isqrt = (sup as f64).sqrt() as u64;
   
   let segment_size = std::cmp::max(isqrt,CACHE);
   
   let mut thread_vec : Vec<std::thread::JoinHandle::<Vec<u64>>> = Vec::new();

   let seg_count = sup/segment_size;
   
   for i in 0..tc{
      let p_i = Arc::clone(&plist);
      let start = (seg_count/tc)*i;
      let stop = (seg_count/tc)*(i+1);
      thread_vec.push(std::thread::spawn(move||{
       sub_sieve(p_i,segment_size,start,stop,&sqr_fermat,false)
       }
      ));
      }

   let mut veccy = vec![];
   for j in thread_vec{
      let mut k = j.join().unwrap();
      veccy.extend_from_slice(&mut k[..]);
   }
   return veccy;
}

// Generates either primes or odd composites
struct PCGenerator<T: PrimeValuation>{
  // List of primes up to sqrt
  sqrt_p : Arc<Vec<u64>>,
  sup: u64,
  c_flag: T, 
}

impl<T: PrimeValuation> PCGenerator<T>{

// Initialises by primality testing as
 pub fn initialise(sup: u64) -> Self{
    let max = sup.isqrt();
    for i in 3..max{
     if i.is_prime(){
        veccy.push(i);
     }
    }
    Self{ sqrt_p: Arc::new(veccy), sup: sup}
 }
 
 fn func_eval(&self) -> Vec<u64>{
 
 }
 // Parallel evaluation of all generated composites 
 pub fn composite_eval(&self, func: dyn)-> Vec<u64>{
 
 }
 // Parallel evaluation of all generated primes
 pub fn prime_eval(&self,func: dyn)-> Vec<u64>{
 
 }
 
}
