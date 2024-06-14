
// Maximum memory in bytes to be used by CompVector
pub const MEMORY_MAX : u64 = 1073741824;
// Configuration to automatically load memory
pub const AUTO_FLAG : bool = true;
// Read/write files in utf8. Default is binary
pub const UTF8_FLAG : bool = false;



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
