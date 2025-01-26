use crate::fermat::FInteger;
use crate::result::FResult;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub struct Point<T: FInteger> {
    base: T,
    value: u64,
}

impl<T: FInteger> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "B: {} V: {}",
            self.base.to_string(),
            self.value.to_string()
        )
    }
}

impl<T: FInteger> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: FInteger> Eq for Point<T> {}

impl<T: FInteger> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl<T: FInteger> Ord for Point<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}


impl<T: FInteger> Point<T> {
    pub fn new(base: T, value: u64) -> Self {
        Self { base, value }
    }

    pub fn base(&self) -> T {
        self.base
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

/// Data Vector for Fermat base statistics
pub struct DataVector<T: FInteger> {
    sort_flag : bool,
    data: Vec<Point<T>>,
}

impl<T: FInteger> std::fmt::Display for DataVector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self
            .data
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let q = v.join("\n");
        write!(f, "{}", q)
    }
}

impl<T: FInteger> DataVector<T> {

    /// Initialise from vector of Points
    pub fn new(data: Vec<Point<T>>) -> Self {
        Self { sort_flag: false, data }
    }
    /// Length of vector
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Sorts by value
    pub fn sort(&mut self) {
        if !self.sort_flag{
           self.data.sort();
           self.sort_flag = true;
        }
    }

    /// Extracts the highest datapoints
    pub fn upper_interval(&self, range: usize) -> Option<Self> {
        if range > self.len() || self.sort_flag==false{
            return None;
        }
        let start = self.len() - range;
        return Some(Self::new(self.data[start..].to_vec()));
    }

    /// Extracts the lowest datapoints
    pub fn lower_interval(&self, range: usize) -> Option<Self> {
        if range > self.len() || self.sort_flag == false{
            return None;
        }
        return Some(Self::new(self.data[..range].to_vec()));
    }

    /// Arithmetic mean of values. Maximum total sum is 2^64-1. Unlikely to ever be achieved in practice
    pub fn arith_mean(&self) -> f64 {
        (self.data.iter().map(|x| x.value).sum::<u64>() as f64) / (self.len() as f64)
    }

    /// Standard deviation from the arithmetic mean
    pub fn standard_deviation(&self) -> f64 {
        let am = self.arith_mean();
        let mut sum = 0f64;

        for i in self.data.iter() {
            sum += ((i.value as f64) - am).powi(2);
        }
        (sum / (self.len() as f64)).sqrt()
    }

    // Strip out the fermat bases
    pub fn base(&self) -> Vec<T> {
        self.data.iter().map(|x| x.base()).collect::<Vec<T>>()
    }
     /// Algorithm improvement, weight the stronger bases more
    pub fn sequence_stat(&self, otra: Self) -> FResult<u64>{
        if self.data.len() != otra.data.len(){
          return FResult::Err("Data vectors are of different lengths");
        }
        if self.sort_flag==false || otra.sort_flag == false{
          return FResult::Err("Both Fdata structures need to be sorted");
        }
        let mut delta = 0u64;
        for (idx,el) in self.data.iter().enumerate(){
           for (jidx,jel) in otra.data.iter().enumerate(){
               if el.base == jel.base{
                  delta +=idx.abs_diff(jidx) as u64;
               }
           }
        }
        FResult::Value(delta)
    }
}
