use machine_prime::{mul_inv2,mul_inv2_128,u256prod};

// Interim trait until Wrapping stabilises
pub trait FiniteArith: Sized{

    fn finite_add(&self, other: Self) -> Self;
    
    fn finite_sub(&self, other: Self) -> Self;
    
    fn finite_neg(&self) -> Self;
    
    fn finite_mul(&self, other: Self) -> Self;
    
    fn overflow_add(&self, other: Self) -> (Self,bool);
    
    fn overflow_sub(&self, other: Self) -> (Self,bool);
    
    fn widening_mul(&self, other: Self) -> (Self,Self);
    
    fn invert(&self) -> Self;
    
    fn to_float(&self) -> f64;
}

impl FiniteArith for u64{

    fn finite_add(&self, other: Self) -> Self{
       self.wrapping_add(other)
    }
    
    fn finite_sub(&self, other: Self) -> Self{
       self.wrapping_sub(other)
    }
    
    fn finite_neg(&self) -> Self{
       self.wrapping_neg()
    }
    
    fn finite_mul(&self, other: Self) -> Self{
       self.wrapping_mul(other)
    }
    
    fn overflow_add(&self, other: Self) -> (Self,bool){
       self.overflowing_add(other)
    }
    
    fn overflow_sub(&self, other: Self) -> (Self,bool){
       self.overflowing_sub(other)
    }
    
    fn widening_mul(&self, other: Self) -> (Self,Self){
       let prod = *self as u128 * other as u128;
       (prod as u64, (prod>>64) as u64)
    }
    
    fn invert(&self) -> Self{
       mul_inv2(*self)
    }
    
    fn to_float(&self) -> f64{
       *self as f64
    }
}

impl FiniteArith for u128{

    fn finite_add(&self, other: Self) -> Self{
       self.wrapping_add(other)
    }
    
    fn finite_sub(&self, other: Self) -> Self{
       self.wrapping_sub(other)
    }
    
    fn finite_neg(&self) -> Self{
       self.wrapping_neg()
    }
    
    fn finite_mul(&self, other: Self) -> Self{
       self.wrapping_mul(other)
    }

    fn overflow_add(&self, other: Self) -> (Self,bool){
       self.overflowing_add(other)
    }
    
    fn overflow_sub(&self, other: Self) -> (Self,bool){
       self.overflowing_sub(other)
    }
    
    fn widening_mul(&self, other: Self) -> (Self,Self){
       u256prod(*self,other)
    }
    
    fn invert(&self) -> Self{
       mul_inv2_128(*self)
    }
    
    fn to_float(&self) -> f64{
       *self as f64
    }
}
