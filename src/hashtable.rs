use crate::io::write::format_block;
use crate::fermat::FInteger;
use crate::CompVector;

pub struct HashTable {
    dimen: usize,
    multiplier: u32,
    table: Vec<u64>,
}

impl HashTable {
    pub fn new(table: Vec<u64>, dimen: usize, multiplier: u32) -> Self {
        HashTable {
            table,
            dimen,
            multiplier,
        }
    }

    pub fn write_utf8(&self, locale: &str) -> Option<()> {
        use std::fs::File;
        use std::io::Write;

        match File::create(locale) {
            Ok(mut out) => {
                let res = self.to_string();
                match out.write_all(res.as_bytes()) {
                    Ok(_) => Some(()),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }
    /// Evaluates primality for an integer, utilizing the hashtable computed
    pub fn primality<T: FInteger>(&self, x: T) -> bool{
       let hash = x.hash_shift((32-self.dimen.trailing_zeros()) as usize, self.multiplier);
       x.sprp(T::from_u64(self.table[hash]))
    }
    
    /// Checks that the hashtable eliminates all composites from the vector
    pub fn prove<T: FInteger>(&self, cvec: &CompVector<T>) -> bool{
         for i in cvec.iter(){
            if self.primality(*i){
              return false
            }
         }
         return true
    }
}

impl std::fmt::Display for HashTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = (1u64 << (32 - self.dimen.trailing_zeros())).to_string();
        let m = self.multiplier.to_string();
        let q = format_block::<16, u64>(&self.table);

        write!(f, "divisor: {} multiplier: {} \n hash(x)  = (x as u32).wrapping_mul({})/{} \n {}", d, m,m,d, q)
    }
}
