use crate::natural::montcore::NTCore;
use crate::structures::store::Persistent;
use crate::{CompVector, FResult, Natural, Primes};

/*
   Preliminary structures to evaluate Fermat pseudoprimes

   Candidate cofactors are of the form k = 1 mod ord_b(p)

   Small values

     Calculate mul_ords  to the small primes

     Cofactors are of the form p_ord 1

     p_ord + p_ord^-1
*/

// Set of primes with a specific order to a base, the base is not stored
#[derive(Debug)]
pub struct SharedOrd {
    ord: u64,
    primes: Vec<u64>,
}

impl SharedOrd {
    fn new_ord(ord: u64) -> Self {
        Self {
            ord: ord,
            primes: Vec::new(),
        }
    }
    /// Append an integer that has a mulord of ord
    fn append(&mut self, x: u64) {
        self.primes.push(x);
    }

    fn is_empty(&self) -> bool {
        self.primes.len() == 0
    }
}

#[derive(Debug)]
pub struct SOSet {
    a: u64,
    bound: u64,
    elements: Vec<SharedOrd>,
}

/// orders of 32-bit integers
#[derive(Clone)]
pub struct SmallOrd {
    a: u64, // (ord,p) 32-bit
    bound: u64,
    pub elements: Vec<u64>,
}
// SmallOrd of integers with a specific signature
pub struct SigOrd{
   ord: SmallOrd,
   sig: u8,
}

impl SOSet {
    fn new(a: u64, bound: u64) -> Self {
        Self {
            a,
            bound,
            elements: vec![],
        }
    }

    fn append(&mut self, x: SharedOrd) {
        self.elements.push(x);
    }

    /*
       Todo Implement cyclotomic factorisations
    */
    /// Calculate the prime orders from the general mersenne numbers bounded to b^n-1 < 2^128
    pub fn from_general_mersenne(base: u64, bound: u64) -> (Self, u64) {
        let mut ord = 1;
        let mut values = SOSet::new(base, bound);
        let inf = bound.isqrt();
        loop {
            ord += 1;
            let mut plist = SharedOrd::new_ord(ord);
            let (res, flag) = (base as u128).overflowing_pow(ord as u32);
            if flag {
                break;
            }
            let f = res.wrapping_sub(1).factor().unwrap();
            for i in f.factors {
                if i > (inf as u128) && i < bound as u128 {
                    plist.append(i as u64);
                }
            }
            if !plist.is_empty() {
                values.append(plist);
            }
        }
        (values, ord)
    }

    /*
        Start from an order

    */
    
    /*
    
      Algorithm improvements 
      
      
    
    
    
    */
    // FIXME Iterate over the residue class of primes and ord
    pub fn from_interval(base: u64, mut start_ord: u64, bound: u64) -> Self {
        let mut res = SOSet::new(base, bound);
        let stop_ord = bound.isqrt();
        for ord in start_ord..stop_ord {
            let mut ordp = SharedOrd::new_ord(ord);
            // FIXME set start value to be around sqrt(N)
            let mut start: u64 = 1;
            for _ in 0..bound / (ord * ord) {
                start += ord;

                if start.is_prime() && start > stop_ord {
                    if base.exp_unit(ord, start) {
                        ordp.append(start);
                        // allocate as it is a prime that has order start_ord
                    }
                }
            }
            if !ordp.is_empty() {
                res.append(ordp);
            }
        }
        res
    }

    pub fn pseudoprimes(&self) -> Vec<u64> {
        let mut values = std::collections::HashSet::new();

        for i in self.elements.iter() {
            for p in i.primes.iter() {
                let mut start = 1;

                for _ in 0..self.bound / (p * i.ord) {
                    start += i.ord;

                    let n = *p * start;

                    if Natural::fermat(&n, self.a) {
                        values.insert(n);
                    }
                }
            }
        }
        values.drain().collect::<Vec<u64>>()
    }
}

impl SmallOrd {
    pub fn initialise_prime(a: u64, p_bound: u64) -> Self {
        let plist = Primes::generate_or_restore(p_bound as usize);
        let mut elements = vec![];

        for i in plist.iter() {
            if i.gcd(a) == 1 {
                elements.push((i << 32) + i.p_ord(a));
            } else {
                elements.push(i);
            }
        }
        Self {
            a: a,
            bound: p_bound,
            elements,
        }
    }

    pub fn initialise(a: u64, sup: u64, bound: u64) -> (Self, Vec<u64>) {
        let mut elements: Vec<u64> = Vec::new();
        let maxprime = bound.isqrt();
        let mut pseudos = vec![];

        for i in 2..sup {
            let primeflag = i.is_prime();

            if !primeflag {
                if Natural::fermat(&i, a) {
                    pseudos.push(i);
                }
            }

            if primeflag && i > maxprime {
                continue;
            }

            match a.ord(i) {
                Some(x) => {
                    if x.gcd(i) == 1 {
                        elements.push((i << 32) + x);
                    }
                }
                None => (),
            }
        }
        (Self { a, bound, elements }, pseudos)
    }
    // Initialise Z
    pub fn initialise_integer(a: u64, sup: u64) -> (Self,Vec<u64>){
        let mut elements: Vec<u64> = Vec::new();
        let mut pseudos = vec![];

        for i in 2..sup {
        
            let primeflag = i.is_prime();

            if !primeflag {
                if Natural::fermat(&i, a) {
                    pseudos.push(i);
                }
            }

            match a.ord(i) {
                Some(x) => {
                    if x.gcd(i) == 1 {
                        elements.push((i << 32) + x);
                    }
                }
                None => (),
            }
        }
        (Self { a, bound: sup, elements }, pseudos)
    }
    // Depth 2 search
    pub fn pseudoprime_shallow(&self) -> Vec<u64>{
                 const MASK: u64 = 0xFFFFFFFF;
    
        let mut values = std::collections::HashSet::new();
        
        for (idx, i) in self.elements.iter().enumerate() {
            let (integer, iord) = (i >> 32, i & MASK);
            let inv = integer.mul_inverse(iord).unwrap();

            for j in self.elements[idx..].iter() {
                let (jnteger, jord) = (j >> 32, j & MASK);

                let n = jnteger.clone() * integer.clone();

                if n > self.bound {
                    break;
                }

                if jnteger % iord == inv {
                    let fullord = jord.lcm(iord).unwrap();
                    if (n - 1) % fullord == 0 {
                        values.insert(n);
                    }
                }
            }
        }
        let mut res = values
            .drain()
            .filter(|v| Natural::fermat(v, self.a))
            .collect::<Vec<u64>>();

        res

    }
    
    /*
       3 depth search
       
       for i in P
         if gcd(i,j) != 1 skip
         for j in P[i..]
           let ord = lcm(iord,jord)
           let r = (i*j)^1 ord
           for k in P[j..]
             if lcm(ord,kord) | i*j*k
              insert into hashtable
         
    */
    /*
    pub fn pseudoprime_experimental(&self) -> Vec<u64>{
        let mut values = std::collections::HashSet::new();
         const MASK: u64 = 0xFFFFFFFF;
         let psup = (sup as f64).powf(2.0/3.0).ceil() as u64;
         let (ord, ps) = Self::initialise_integer(a,sup.isqrt());
         
        for i in ps {
            values.insert(i);
        }
        
        for (idx, i) in ord.elements.iter().enumerate() {
            let (integer, iord) = (i >> 32, i & MASK);
            let inv = integer.mul_inverse(iord).unwrap();

            for (jdx,j) in ord.elements[..].iter().enumerate() {
                let (jnteger, jord) = (j >> 32, j & MASK);
                if jnteger%iord != inv{
                   continue;
                }
                // If j and i share a factor then j*i is not square free
                //if jnteger.gcd(integer) != 1{
                //   continue;
                //}
                //if integer == 349{
                //   println!("j can {}",jnteger);
                // }
                let partialprod = jnteger.clone() * integer.clone();
                
                //if partialprod > psup{
                //   println!("Broke at {} = {}*{}",partialprod,integer,jnteger);
                //   continue;
                  //break;
                //}
                
                let partialord = jord.lcm(iord).unwrap();
                //if partialprod == 182527{
                //   println!("j {} {} i {} {} ord {}",integer,iord,jnteger,jord, partialord);
               // }
                
                if (partialprod-1)%partialord == 0{
                  //if partialprod == 182527{
                  //   println!("Inserted!");
                  //}
                  values.insert(partialprod);
                }
                
                //if partialprod > psup{
                //   println!("Broke at {} = {}*{}",partialprod,integer,jnteger);
                //   continue;
                  //break;
                //}
                /**/
                //if partialprod > psup{
                //   println!("Broke at {} = {}*{}",partialprod,integer,jnteger);
                //   continue;
               // }
                
                //if partialprod.gcd(partialord) != 1{
                //   continue;
                //}
                
               // if partialprod > psup{
                //   println!("Broke at {} = {}*{}",partialprod,integer,jnteger);
               //    continue;
               // }
                /**/
                let mut inv : u64 = 0;
                match (partialprod).mul_inverse(partialord){
                   Some(x) => inv=x,
                   None => (),
                };
                
                if inv == 0{
                   continue;
                }
                //if partialprod == 4453{
                //  println!("It's a partialprod with inv {} {}",inv, partialord);
                //}
                  for k in ord.elements[jdx..].iter(){
                    let (knteger, kord) = (k >> 32, k & MASK);
                    //if partialprod == 4453{
                    //   println!("{} maximum {}",knteger, sup);
                    //}
                      if knteger%partialord == inv{
                         let fullord = partialord.lcm(kord).unwrap();
                         let n = partialprod*knteger;
                //         if partialprod == 4453{
                //           println!("k {} n {} ordn {}",knteger,n,fullord);
                //         }
                         if n > sup {
                          break;
                         }
                         
                         if (n-1)% fullord == 0{
                           values.insert(n);
                         }
                      }
                  }
                  /*
                if jnteger % iord == inv {
                    let fullord = jord.lcm(iord).unwrap();
                    if (n - 1) % fullord == 0 {
                        values.insert(n);
                    }
                }
                */
            }
        }
        values
            .drain()
            .filter(|v| Natural::fermat(v, a))
            .collect::<Vec<u64>>()
  
    }
    */
    
    /*
    pub fn depth_three(a: u64, sup: u64) -> Vec<u64>{
        let mut values = std::collections::HashSet::new();
        const MASK: u64 = 0xFFFFFFFF;
         let psup = (sup as f64).powf(2.0/3.0).ceil() as u64;
         let (ord, ps) = Self::initialise_integer(a,sup.isqrt());
    }
    */
    // Depth 3 search
    pub fn pseudoprimes2(a: u64,sup: u64) -> Vec<u64>{
         let mut values = std::collections::HashSet::new();
         const MASK: u64 = 0xFFFFFFFF;
         let psup = (sup as f64).powf(2.0/3.0).ceil() as u64;
         let (ord, ps) = Self::initialise_integer(a,sup.isqrt());
         
        for i in ps {
            values.insert(i);
        }
        
        for (idx, i) in ord.elements.iter().enumerate() {
            let (integer, iord) = (i >> 32, i & MASK);
           // let inv = integer.mul_inverse(iord).unwrap();

            for (jdx,j) in ord.elements[idx..].iter().enumerate() {
                let (jnteger, jord) = (j >> 32, j & MASK);
    //            if jnteger%iord != inv{
    //               continue;
    //            }
                // If j and i share a factor then j*i is not square free
                //if jnteger.gcd(integer) != 1{
                //   continue;
                //}
                //if integer == 349{
                //   println!("j can {}",jnteger);
                // }
                let partialprod = jnteger.clone() * integer.clone();
                
                //if partialprod > psup{
                //   println!("Broke at {} = {}*{}",partialprod,integer,jnteger);
                //   continue;
                  //break;
                //}
                
                let partialord = jord.lcm(iord).unwrap();
                //if partialprod == 182527{
                //   println!("j {} {} i {} {} ord {}",integer,iord,jnteger,jord, partialord);
               // }
                
                if (partialprod-1)%partialord == 0{
                  //if partialprod == 182527{
                  //   println!("Inserted!");
                  //}
                  values.insert(partialprod);
                }
                
                if partialprod > psup{
                   //println!("Broke at {} = {}*{}",partialprod,integer,jnteger);
                   continue;
                 // break;
                }
                /**/
                //if partialprod > psup{
                //   println!("Broke at {} = {}*{}",partialprod,integer,jnteger);
                //   continue;
               // }
                
                //if partialprod.gcd(partialord) != 1{
                //   continue;
                //}
                
               // if partialprod > psup{
                //   println!("Broke at {} = {}*{}",partialprod,integer,jnteger);
               //    continue;
               // }
                /**/
                let mut inv : u64 = 0;
                match (partialprod).mul_inverse(partialord){
                   Some(x) => inv=x,
                   None => (),
                };
                
                if inv == 0{
                   continue;
                }
                //if partialprod == 4453{
                //  println!("It's a partialprod with inv {} {}",inv, partialord);
                //}
                  for k in ord.elements[jdx..].iter(){
                    let (knteger, kord) = (k >> 32, k & MASK);
                    //if partialprod == 4453{
                    //   println!("{} maximum {}",knteger, sup);
                    //}
                      if knteger%partialord == inv{
                         let fullord = partialord.lcm(kord).unwrap();
                         let n = partialprod*knteger;
                //         if partialprod == 4453{
                //           println!("k {} n {} ordn {}",knteger,n,fullord);
                //         }
                         if n > sup {
                          break;
                         }
                         
                         if (n-1)% fullord == 0{
                           values.insert(n);
                         }
                      }
                  }
                  /*
                if jnteger % iord == inv {
                    let fullord = jord.lcm(iord).unwrap();
                    if (n - 1) % fullord == 0 {
                        values.insert(n);
                    }
                }
                */
            }
        }
        values
            .drain()
            .filter(|v| Natural::fermat(v, a))
            .collect::<Vec<u64>>()
        
    }

    pub fn pseudoprimes(a: u64, sup: u64) -> Vec<u64> {
        let mut values = std::collections::HashSet::new();
        const MASK: u64 = 0xFFFFFFFF;
        let (ord, ps) = Self::initialise(a, (sup as f64).powf(2.0 / 3.0).ceil() as u64, sup);

        for i in ps {
            values.insert(i);
        }

        for (idx, i) in ord.elements.iter().enumerate() {
            let (integer, iord) = (i >> 32, i & MASK);
            let inv = integer.mul_inverse(iord).unwrap();

            for j in ord.elements[idx..].iter() {
                let (jnteger, jord) = (j >> 32, j & MASK);

                let n = jnteger.clone() * integer.clone();

                if n > sup {
                    break;
                }

                if jnteger % iord == inv {
                    let fullord = jord.lcm(iord).unwrap();
                    if (n - 1) % fullord == 0 {
                        values.insert(n);
                    }
                }
            }
        }
        let mut res = values
            .drain()
            .filter(|v| Natural::fermat(v, a))
            .collect::<Vec<u64>>();

        res
    }

    pub fn iter(&self) -> std::slice::Iter<'_, u64> {
        self.elements.iter()
    }
}
/*
 impl std::iter::Iterator for SmallOrd{
       type Item = (u64,u64);
   fn next(&mut self) -> Option<Self::Item>{
      self.elements[..].iter().next().copied().map(|x| (x&0xFFFFFFFF,x>>32))
   }

 }
*/
impl Persistent for SmallOrd {
    fn to_persistent(&self, locale: &str) -> FResult<()> {
        use std::io::Write;

        match std::fs::File::create(locale) {
            Ok(mut x) => {
                // Write element
                match x.write(&self.a.to_bytes()[..]) {
                    Ok(_) => (),
                    Err(message) => return FResult::IOError(message),
                }
                // Write bound
                match x.write(&self.bound.to_bytes()[..]) {
                    Ok(_) => (),
                    Err(message) => return FResult::IOError(message),
                }

                // write values
                for i in self.elements.iter() {
                    match x.write(&i.to_bytes()[..]) {
                        Ok(_) => (),
                        Err(message) => return FResult::IOError(message),
                    };
                }
                //x.write()
            }
            Err(message) => return FResult::IOError(message),
        }

        FResult::Success
    }

    fn from_persistent(locale: &str) -> FResult<Self> {
        use std::io::Read;

        match std::fs::File::open(locale) {
            Ok(x) => {
                let mut interim = vec![0u8; u64::BYTE_LENGTH];
                let mut r = std::io::BufReader::new(x);

                // FIXME handle read errors;
                r.read(&mut interim[..]);
                let a = u64::from_bytes(&interim);
                r.read(&mut interim[..]);
                let bound = u64::from_bytes(&interim);

                let mut elements = Vec::new();

                loop {
                    match r.read(&mut interim[..]) {
                        Ok(totalbytes) => {
                            if totalbytes == 0usize {
                                break;
                            }
                            let num = u64::from_bytes(&interim);

                            elements.push(num);
                        }
                        Err(err_message) => return FResult::IOError(err_message),
                    }
                }
                return FResult::Value(Self { a, bound, elements });
            }
            Err(mess) => FResult::IOError(mess),
        }
    }
}

/*
fn calculate_large_primes() -> OrdPrimes{
   //
   let bound =
   //
}

fn calculate_small_primes() -> PrimeOrd{

}
*/
