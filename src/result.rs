/// Result of calculations and possible failures
pub enum FResult<T: Clone>{
  /// Search mode not supported for current configuration
  NotSupported,
  /// Unable to run without exceeding some assigned bound
  MemoryExceeded,
  /// Some proof of correctness failed
  ProofFailed,
  /// Some proof of correctness succeeded
  Verified,
  /// Unable to find sufficient candidates 
  InsufficientCandidates(usize),
  /// No Candidate found
  NoCandidate,
  /// Full Solution
  Exhaustive(T),
  /// Partial Solution with number of counterexamples left 
  Partial(T,usize),
  /// Input output Error
  IOError(std::io::Error),
  /// File Does Not Exist
  FileDNE,
  /// Successful execution of function that returns unit-type ()
  Success,
  /// Non-solution result
  Value(T),
  }
  
  
  impl<T: std::fmt::Display + Clone> std::fmt::Display for FResult<T>{
  
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self{
        FResult::Success => write!(f,"Operation Succeeded"),
        FResult::NotSupported => write!(f,"Not supported"),
        FResult::MemoryExceeded => write!(f,"Unable to access sufficient memory"),
        FResult::ProofFailed => write!(f,"Failed Proof"),
        FResult::Verified => write!(f,"Verified"),
        FResult::InsufficientCandidates(x) => write!(f,"{} candidates exist",x),
        FResult::IOError(message) => write!(f,"{}",message),
        FResult::FileDNE => write!(f,"File not found"),
        FResult::NoCandidate => write!(f,"No candidate exists"),
        FResult::Exhaustive(x) => write!(f,"{}",x),
        FResult::Partial(x,y) => write!(f,"{} {}",x,y),
        FResult::Value(x) => write!(f,"{}",x),
       }
     }
     
  }
  //FBase { bases: [42135, 61334, 14625, 51169, 448435, 1], mode: WeakHeuristic }
  impl<T: Clone> FResult<T>{
     
  pub fn unwrap(&self) -> T{
        match self{
          FResult::Exhaustive(x) =>x.clone(),
          FResult::Partial(x,_) => x.clone(), 
          FResult::Value(x) => x.clone(),
          FResult::NotSupported => panic!("Requested operation not supported"),
          FResult::NoCandidate => panic!("No candidate exists"),
          FResult::InsufficientCandidates(x) => panic!("Insufficient Candidates: {}",x),
          FResult::MemoryExceeded => panic!("Unable to access sufficient memory"),
          _=> panic!("Value does not exist"),
        }
     }
  } 
