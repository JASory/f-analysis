use crate::io::write::format_block;

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
}

impl std::fmt::Display for HashTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = (1u64 << (32 - self.dimen.trailing_zeros())).to_string();
        let m = self.multiplier.to_string();
        let q = format_block::<16, u64>(&self.table);

        write!(f, "divisor: {} multiplier: {} \n {}", d, m, q)
    }
}
