use crate::fermat::FInteger;

/// Trait implementing some check that returns a boolean, and the negation of that check
pub trait GenericFilter{
   fn filter_check<T: FInteger>(x: T) -> bool;
}

/// Trait implementing the a^(n-1)=1 (mod n) Fermat test
pub trait WeakFermat {
    fn fermat<T: FInteger>(x: T) -> bool;
}

/// Trait implementing the a^(n-1)=1 (mod n) Fermat test
pub trait EulerFermat: WeakFermat {
    fn efermat<T: FInteger>(x: T) -> bool;
}

/// Trait implementing the strong variant
pub trait StrongFermat: EulerFermat + GenericFilter {
    fn sprp<T: FInteger>(x: T) -> bool;
}

/// Trait implementing checks for integers coprime to some set of integers
pub trait Coprime: GenericFilter {
    fn coprime<T: FInteger>(x: T) -> bool;
}

/// Trait implementing checks for integers of a certain form, true means that the integer is of that form
pub trait FormCheck: GenericFilter {
    fn is_form<T: FInteger>(x: T) -> bool;
}
