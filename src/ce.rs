use crate::io::read::read_binary;
use crate::io::write::write_binary;

/// A bitvector that stores information on counterexamples to some reference set
///
/// Primarily for internal use.
pub struct CounterExamples {
    idx: Vec<u64>,
}

impl CounterExamples {
    pub fn new(size: usize) -> Self {
        let mut len = size / 64;

        if size % 64 != 0 {
            len += 1;
        }
        let idx = vec![0u64; len];
        Self { idx }
    }

    pub fn from_vector(idx: Vec<u64>) -> Self {
        Self { idx }
    }
    
    /// Read from binary
    pub fn from_file(locale: &str) -> Option<Self> {
        match std::fs::read(locale) {
            Ok(data) => Some(Self::from_vector(read_binary(data))),
            Err(_) => None,
        }
    }

    // Writes the vector to binary file
    pub fn write(&self, locale: &str) -> Option<()> {
        use std::fs::File;
        use std::io::Write;

        match File::create(locale) {
            Ok(mut out) => {
                let res = write_binary(&self.idx);
                match out.write_all(&res) {
                    Ok(_) => Some(()),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    // Set idx
    pub fn set(&mut self, pos: usize) {
        self.idx[pos >> 6] |= 1 << (pos & 63);
    }

    // AND operator
    pub fn shared(&mut self, other: &Self) {
        for (i, j) in self.idx.iter_mut().zip(other.idx.iter()) {
            *i &= j
        }
    }

    // XOR
    pub fn delta(&mut self, other: &Self) {
        for (i, j) in self.idx.iter_mut().zip(other.idx.iter()) {
            *i ^= j
        }
    }
    
    /// Number of counterexamples detected
    pub fn count(&self) -> u64 {
        let mut sum = 0u64;
        for i in self.idx.iter() {
            sum += i.count_ones() as u64;
        }
        sum
    }

    pub fn jaccard_c(&self, other: &Self) -> f64 {
        let mut union_count = 0u32;
        let mut intersection_count = 0u32;

        for (i, j) in self.idx.iter().zip(other.idx.iter()) {
            union_count += (*i & *j).count_ones();
            intersection_count += (*i | *j).count_ones();
        }
        union_count as f64 / intersection_count as f64
    }
}
