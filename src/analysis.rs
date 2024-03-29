use crate::Interval;
use std::path::Path;

/// Unified Analysis of Fermat bases
///
/// This is the preferred method of research as it handles data automatically
/// However some functionality may require using more primitive functions
pub struct FAnalysis {
    inf: usize,
    sup: usize,
}

impl FAnalysis {
    /// Initialise with lower bound of search, upper bound of search
    pub fn new(inf: usize, sup: usize) -> Self {
        Self { inf, sup }
    }

    /// Write preliminary data of the semiprimes of the form (k+1)(ak+1) with the interval
    pub fn init_semiprime_k(&self, a: usize) {
        let filename = "ce/".to_string()
            + &self.inf.to_string()
            + "_"
            + &self.sup.to_string()
            + "-"
            + &a.to_string();

        if Path::new("ce").exists() {
            if Path::new(&filename).exists() {
                () // do nothing
            } else {
                // create file
                let mut k_ce = Interval::new(self.inf, self.sup).gen(a as u64);
                k_ce.sort();
                k_ce.write_binary(&filename).unwrap();
            }
        } else {
            std::fs::create_dir("ce").unwrap();
            let mut k_ce = Interval::new(self.inf, self.sup).gen(a as u64);
            k_ce.sort();
            k_ce.write_binary(&filename).unwrap();
        }
    }
    
    //pub fn init_fermat_ce
    /*
    pub fn iter_search(&self, bound: u64) -> Vec<u64>{

       let filename = "ce/".to_string() + &self.inf.to_string()+"_"+&self.sup.to_string()+"-"+&self.a.to_string();
       let z = CompVector::read_binary_unchecked(&filename).unwrap();
       z.iter_sprp_search(bound)
    }
    */
    /*
    pub fn is_init_ce() -> bool{

    }

    fn is_init() -> bool{

    }

    pub fn init(&self,){

    }

    // Returns the counterexamples that are shared by the
    pub fn shared_all() -> CompVector

    // Returns the counterexamples that are shared by the
    pub fn delta_all() -> CompVector

    // Finds the sets of fermat bases that are deterministic
    pub fn sprp_vector() -> Vec<u64>
    */
}
