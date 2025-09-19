/*!
   Selection of existing functions to eliminate composites of certain forms. All functions return true if the condition is met

     Example of usage

   ```
   use f_analysis::CompVector;
   use f_analysis::filter::{Base,NQR,PFB};


      let ce = CompVector::<u64>::from_vector(vec![52,341,561,2047]);
      // Base 2 fermat test removes 52
      assert_eq!(ce.filter_fermat::<Base<2>>(),CompVector::from_vector(vec![341,561,2047]));
      // Base 2 Strong Fermat test removes all but the strong pseudoprime 2047
      assert_eq!(ce.filter_sprp::<Base<2>>(),CompVector::from_vector(vec![2047]));
      // The Carmichael number 561 remains, even after a special base selection
       assert_eq!(ce.filter_fermat::<NQR>(),CompVector::from_vector(vec![561]));
      // Strength the test with the same selection method, and there are no composites left
       assert_eq!(ce.filter_sprp::<NQR>(),CompVector::from_vector(vec![]));
      // Likewise use the first 2 prime bases 2,3. Carmichael remains in the weak test
      assert_eq!(ce.filter_fermat::<PFB<2>>(),CompVector::from_vector(vec![561]));
      // But is eliminated in the strong version
      assert_eq!(ce.filter_sprp::<PFB<2>>(),CompVector::from_vector(vec![]));

   ```
*/
pub(crate) mod fermat;
pub(crate) mod filtertype;
pub(crate) mod form;
pub(crate) mod ftraits;
pub(crate) mod gfilter;

pub use filtertype::*;
pub use ftraits::{Coprime, EulerFermat, FormCheck, GenericFilter, StrongFermat, WeakFermat};
