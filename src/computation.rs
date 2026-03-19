/*
   Large scale computation
   
   These involve computations that can take months and must be able to recover from a prior state
*/

mod psi;
mod compio;
mod wieferich;
pub use psi::PsiEval;
pub use wieferich::WieferichEval;

/*
computation 

 Monier Rabin Bound
   Storage
    Residues
    the pseudoprimes
    the residue being evaluated
  
  Recovery start from the residue
  
  
  Fermat pseudoprimes
  
  Stored values 
  
    Mulords
    
    Cunningham factorisations
    
    Large mul ord search {store the ord being searched, for recovery}
    
    Weiferich primes
    
    
    
  Weiferich prime search 
  
  Stored
    Primes
    Index
    
  Recovery
   Retrieve index
   */
