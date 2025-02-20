
// Maximum memory in bytes to be used by CompVector
pub const MEMORY_MAX : u64 = 1073741824;
// Configuration to automatically load memory
pub const AUTO_FLAG : bool = true;
// Read/write files in utf8. Default is binary
pub const UTF8_FLAG : bool = false;


/// Result of calculations and possible failures
pub enum FResult<T: Clone>{
  /// Search mode not supported for current configuration
  NotSupported,
  /// Unable to run without exceeding some assigned bound
  MemoryExceeded(usize),
  /// Generic failure of some operation
  Failure,
  /// Unable to find sufficient candidates 
  InsufficientCandidates(usize),
  /// No Candidate found
  NoCandidate,
  /// Partial Solution with number of counterexamples left 
  Partial(T,usize),
  /// Input output Error
  IOError(std::io::Error),
  /// File Does Not Exist
  FileDNE,
  /// All other errors
  Err(&'static str),
  /// Successful execution of function that returns unit-type ()
  Success,
  /// Critical Error occurred, a last resort catch-all. This should never be returned
  Critical,
  /// Result of operation
  Value(T),
  }
  
  impl<T: std::fmt::Display + Clone> std::fmt::Display for FResult<T>{
  
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self{
        FResult::Success => write!(f,"Operation Succeeded"),
        FResult::NotSupported => write!(f,"Not supported"),
        FResult::MemoryExceeded(mem) => write!(f,"Required memory {} bytes exceeds the MemoryMax",mem),
        FResult::Failure => write!(f,"Failed"),
        FResult::InsufficientCandidates(x) => write!(f,"{} candidates exist",x),
        FResult::IOError(message) => write!(f,"{}",message),
        FResult::FileDNE => write!(f,"File not found"),
        FResult::NoCandidate => write!(f,"No candidate exists"),
        FResult::Partial(x,y) => write!(f,"{} {}",x,y),
        FResult::Critical => write!(f,"Unknown Critical error occurred, file issue immediately to https://github.com/JASory/f-analysis"),
        FResult::Err(message) => write!(f,"{}",message),
        FResult::Value(x) => write!(f,"{}",x),
       }
     }
     
  }
  

  impl<T: Clone> FResult<T>{
     
  pub fn unwrap(&self) -> T{
        match self{
          FResult::Partial(x,_) => x.clone(), 
          FResult::Value(x) => x.clone(),
          FResult::NotSupported => panic!("Requested operation not supported"),
          FResult::NoCandidate => panic!("No candidate exists"),
          FResult::InsufficientCandidates(x) => panic!("Insufficient Candidates: {}",x),
          FResult::MemoryExceeded(mem) => panic!("Required memory {} bytes exceeds the MemoryMax",mem),
          FResult::IOError(message) => panic!("{}",message),
          FResult::Err(message) => panic!("{}",message),
          FResult::Critical => panic!("Critical error occurred, file issue immediately to https://github.com/JASory/f-analysis"),
          _=> panic!("Value does not exist"),
        }
     }
  } 
  
#[derive(Copy,Clone,Debug,PartialEq,PartialOrd,Eq,Ord)]
pub enum Pseudoprime {
    Composite=0,
    Fermat=1,
    Euler=2,
    EulerJacobi=3,
    Strong=4,
    Prime=5,
}

/// Enum of search variants
/// WeakHeuristic, StrongHeuristic, and Deterministic are successively supersets of the previous one
/// i.e all the values tested by the WeakHeuristic will be tested by the StrongHeuristic
/// Other variants if implemented will have no guarantee of overlap
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Search{
	   // Use a weak heuristic; very fast but only an estimate
	   /// # Accuracy
	   /// Frequently correct on calculating first counterexamples to bases.
	   /// Performs poorly at proving a base set eliminates all composites
	   /// Almost certainly fails at generating all pseudoprimes to a set of bases
	   /// # Forms
	   // Currently the semiprimes of the form (2x+1)(4x+1)
	   WeakHeuristic,
	   // Use a strong heuristic, slower but much more likely to be correct
	   /// # Accuracy
	   /// Same as WeakHeuristic but stronger in all cases, and over large intervals
	   /// appears to frequently correctly show that the base sets eliminate all composites
	   /// # Forms
	   /// Currently the WeakHeuristic and semiprimes of the form (ak+1)(k+1) where a ranges from 2 to 63 inclusive. 
	   StrongHeuristic,
	   // Very slow but provides guaranteed correctness
	   Deterministic,
	   
	   // Use the candidates for the Monier-Rabin bound
	  // MRHeuristic,
	  // Use the composites of the form (ak+1)(k+1) where ak+1 and k+1 are prime
	  // SPKHeuristic(usize),
	  // MRSPKAHeuristic(usize),
	  
    }
