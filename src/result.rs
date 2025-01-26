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
  //FBase { bases: [42135, 61334, 14625, 51169, 448435, 1], mode: WeakHeuristic }
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
