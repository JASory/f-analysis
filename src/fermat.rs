//! Integers that are used for Fermat bases and iterators that generate bases of interest
//!
//! Currently only u64 is supported however extensions into u128 and 1024 bit are likely
//!
//! FIterators are defined to have a length, some of them have a starting value.
//!
//! ```
//!
//! ```
//!
//!

mod fiterator;
mod ftrait;
mod eight_bytes;
mod sixteen_bytes;
mod epz;

pub use ftrait::{FInteger, FIterator,NTCore};

pub use fiterator::{CompRng, CompSeq, IntSeq,IntRng, PrimeRng, PrimeSeq};
