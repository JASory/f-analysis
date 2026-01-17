use crate::natural::extended::inlineops::{divide3by2, fuse, mul_acc, split};
use crate::natural::extended::sliceops::{
    add_slice, leading_digit, leading_idx, remove_lead_zeros,
};

/*
   FIXME possibly remove the split_at_mut
*/

/*
   Multiplication is simply the classical algorithm storing the values in and accumulator

   This is sufficient for integers less than 2^2048, which is f-analysis` target,
*/

// Multiply X_n by a scalar y and accumulate the product in acc
pub(crate) fn mac_scalar(x: &[u64], y: u64, acc: &mut [u64]) {
    // if y is zero then do nothing
    // Possible improvements
    // If y == 1 then map x to acc

    if y == 0 {
        return;
    }

    let mut carry = 0;
    
    if x.len() < acc.len() {
    
    let (a_lo, a_hi) = acc.split_at_mut(x.len());

    for (a, &x) in a_lo.iter_mut().zip(x) {
        *a = mul_acc(*a, x, y, &mut carry);
    }

    let (carry_hi, carry_lo) = split(carry);

    if carry_hi == 0 {
        add_slice(a_hi, &[carry_lo], 0u8);
    } else {
        add_slice(a_hi, &[carry_hi, carry_lo], 0u8);
    }
    
    }
    // Simulates finite ring arithmetic which discards the upper elements
    else{
      for (a, &x) in acc.iter_mut().zip(x) {
        *a = mul_acc(*a, x, y, &mut carry);
      }
    }
}

pub(crate) fn mul_slice(mut x: &[u64], mut y: &[u64], mut acc: &mut [u64]) {
    let (x, y) = if leading_idx(x) < leading_idx(y) {
        (x, y)
    } else {
        (y, x)
    };

    for (i, xi) in x.iter().enumerate() {
        mac_scalar(y, *xi, &mut acc[i..]);
    }
}

/*
pub(crate) fn sub_sign(mut a: &[u64], mut b: &[u64]) -> (Sign, Vec<u64>) {
    if let Some(&0) = a.last() {
        a = &a[..a.iter().rposition(|&x| x != 0).map_or(0, |i| i + 1)];
    }
    if let Some(&0) = b.last() {
        b = &b[..b.iter().rposition(|&x| x != 0).map_or(0, |i| i + 1)];
    }

    match cmp_slice(a, b) {
        Ordering::Greater => {
            let mut a = a.to_vec();
            sub_slice(&mut a[..], b);
            (Sign::Positive, a)
        }
        Ordering::Less => {
            let mut b = b.to_vec();
            sub_slice(&mut b[..], a);
            (Sign::Negative, b)
        }
        Ordering::Equal => (Sign::Positive, vec![0]),
    }
}
*/

// scale and subtract y from x
fn sub_mul(x: &mut [u64], y: &[u64], z: u64) -> u64 {
    let mut carry = u64::MAX;

    for (i, j) in x.iter_mut().zip(y) {
        let fused = fuse(u64::MAX, *i);
        let result = fused - u64::MAX as u128 + carry as u128 - *j as u128 * z as u128;

        let (new_x, new_carry) = split(result);

        carry = new_carry;
        *i = new_x;
    }
    u64::MAX - carry
}

/*
   leading digit
*/

pub(crate) fn rem_slice2(mut a: &mut [u64], b: &[u64]) {
    let mut a0 = 0;
    let bidx = leading_idx(b);
    let mut aidx = leading_idx(a);
    let b0 = b[bidx];
    let b1 = b[bidx - 1];
    let quo_len = aidx - bidx + 1;

    for j in (0..quo_len).rev() {
        let a1 = a[aidx];
        let a2 = a[aidx - 1];

        let mut q0 = divide3by2(a0, a1, a2, b0, b1);

        let borrow = sub_mul(&mut a[j..], b, q0);

        if borrow > a0 {
            //  q0 -= 1;
            add_slice(&mut a[j..], b, 0u8);
        }

        a0 = a[aidx];
        a[aidx] = 0;
        aidx -= 1;
    }
}

pub(crate) fn rem_slice(a: &mut Vec<u64>, b: &[u64]) {
    let mut a0 = 0;
    let b0 = *b.last().unwrap();
    let b1 = b[b.len() - 2];
    
    if a.len() < b.len() {
        return;
    }

    let quo_len = a.len() - b.len() + 1;
    //let mut quo = vec![0; quo_len];

    for j in (0..quo_len).rev() {
        let a1 = *a.last().unwrap();
        let a2 = a[a.len() - 2];

        let mut q0 = divide3by2(a0, a1, a2, b0, b1);

        let borrow = sub_mul(&mut a[j..], b, q0);

        if borrow > a0 {
            q0 -= 1;
            add_slice(&mut a[j..], b, 0);
        }

        //  quo[j] = q0;

        a0 = a.pop().unwrap();

    }

    a.push(a0);
}

pub(crate) fn quo_rem_slice(a: &mut Vec<u64>, b: &[u64], q: &mut [u64]){
    let mut a0 = 0;
    println!("plimbs : {:?} qlimbs : {:?}",a,b);
    let b0 = *b.last().unwrap();
    let b1 = b[b.len() - 2];
    println!("Initialised");
    if a.len() < b.len() {
        return;
    }

    let quo_len = a.len() - b.len() + 1;
    //let mut quo = vec![0; quo_len];
    println!("qlen {}",quo_len);
    for j in (0..quo_len).rev() {
        let a1 = *a.last().unwrap();
        let a2 = a[a.len() - 2];
        println!("q0 : ({} {} {}) {} {}",a0,a1,a2,b0,b1);
        let mut q0 = divide3by2(a0, a1, a2, b0, b1);

        let borrow = sub_mul(&mut a[j..], b, q0);

        if borrow > a0 {
            q0 -= 1;
            add_slice(&mut a[j..], b, 0);
        }

          q[j] = q0;

        a0 = a.pop().unwrap();

    }

    a.push(a0);
}

/*
pub(crate) fn rem_slice(a: &mut Vec<u64>, b: &[u64]){
    let mut a0 = 0;
    let b0 = *b.last().unwrap();
    let b1 = b[b.len() - 2];

    let quo_len = a.len() - b.len() + 1;

    for j in (0..quo_len).rev() {
        let a1 = *a.last().unwrap();
        let a2 = a[a.len() - 2];

        let mut q0 = divide3by2(a0, a1, a2, b0, b1);

        let borrow = sub_mul(&mut a[j..], b, q0);

        if borrow > a0 {
            q0 -= 1;
            add_slice(&mut a[j..], b,0u8);
        }

        a0 = a.pop().unwrap();
    }
    //if a0 != 0{
    a.push(a0);
    //}
    //remove_lead_zeros(a);

}
*/
/*
pub(crate) fn euclidean_slice(a: &mut [u64], b: &[u64], q: &mut[u64]){
    let mut a0 = 0;

    let bidx = leading_idx(b);
    let mut aidx = leading_idx(a);

    let b0 = b[bidx];
    let b1 = b[bidx-1];

    let quo_len = aidx - bidx + 1;
    //let mut a0 = a[aidx];
    for j in (0..quo_len).rev(){

        let a1 = a[aidx];
        let a2 = a[aidx-1];

        let mut q0 = divide3by2(a0, a1, a2, b0, b1);

        let borrow = sub_mul(&mut a[j..], b, q0);

        if borrow > a0 {
            q0 -= 1;
            add_slice(&mut a[j..], b,0u8);
        }


        a0 = a[aidx];//a.pop().unwrap();
        a[aidx]=0;
       // if
        aidx=//leading_idx(a);
    }
}
*/
/*
pub(crate) fn rem3(quotient: Vec<u64>, b: &[u64]) -> Vec<u64>{
  let shift = leading_digit(b);
  let mut rem = quotient.clone();

   if shift ==0{
  //    let rem = quotient.clone();
      euclidean_slice2(&mut rem,b);
      return rem;
   }
   shl(rem,shift);
   shl()
 //  quotient.clone();
 //     euclidean_slice(&mut rem);
      return rem;
    euclidean_slice2()
}

pub(crate) fn euclidean_slice2(a: &mut Vec<u64>, b: &[u64]) -> Vec<u64> {
    let mut a0 = 0;

    let b0 = *b.last().unwrap();
    let b1 = b[b.len() - 2];

    let quo_len = a.len() - b.len() + 1;
    let mut quo = vec![0; quo_len];

    for j in (0..quo_len).rev() {
        let a1 = *a.last().unwrap();
        let a2 = a[a.len() - 2];

        let mut q0 = divide3by2(a0, a1, a2, b0, b1);

        let borrow = sub_mul(&mut a[j..], b, q0);

        if borrow > a0 {
            q0 -= 1;
            add_slice(&mut a[j..], b);
        }

        //quo[j] = q0;

        a0 = a.pop().unwrap();

    }

    a.push(a0);
    //let mut remainder = a.to_vec();
    //let mut quotient = quo.to_vec();
    remove_lead_zeros(&mut a);

    //remove_lead_zeros(&mut quotient);
    //if remainder.is_empty() {
    //    remainder.push(0u64)
    //}

    //if quotient.is_empty() {
    //    quotient.push(0u64)
    //}
    // remainder
}

*/
/*
pub(crate) fn euclidean_slice(a: &mut Vec<u64>, b: &[u64]) -> (Vec<u64>, Vec<u64>) {
    let mut a0 = 0;

    let b0 = *b.last().unwrap();
    let b1 = b[b.len() - 2];

    let quo_len = a.len() - b.len() + 1;
    let mut quo = vec![0; quo_len];

    for j in (0..quo_len).rev() {
        let a1 = *a.last().unwrap();
        let a2 = a[a.len() - 2];

        let mut q0 = divide3by2(a0, a1, a2, b0, b1);

        let borrow = sub_mul(&mut a[j..], b, q0);

        if borrow > a0 {
            q0 -= 1;
            add_slice(&mut a[j..], b);
        }

        quo[j] = q0;

        a0 = a.pop().unwrap();
    }

    a.push(a0);
    let mut remainder = a.to_vec();
    let mut quotient = quo.to_vec();
    remove_lead_zeros(&mut remainder);

    remove_lead_zeros(&mut quotient);
    if remainder.is_empty() {
        remainder.push(0u64)
    }

    if quotient.is_empty() {
        quotient.push(0u64)
    }
    (quotient, remainder)
}
*/
// Arbitrary precision squaring routine

// (aR^2 + bR + c)^2

// a^2R^4 + abR^3 + acR^2 +
//
