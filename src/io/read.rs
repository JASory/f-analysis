use crate::fermat::FInteger;

pub(crate) fn read_composite_newline<T: FInteger>(x: String) -> Option<Vec<T>> {
    let mut res = vec![];
    for i in x.split_whitespace() {
        match T::from_str(i) {
            Some(z) => {
                if !z.is_prime() {
                    res.push(z);
                }
            }
            None => return None,
        }
    }
    Some(res)
}

#[allow(dead_code)]
pub(crate) fn read_prime_newline<T: FInteger>(x: String) -> Option<Vec<T>> {
    let mut res = vec![];
    for i in x.split_whitespace() {
        match T::from_str(i) {
            Some(z) => {
                if z.is_prime() {
                    res.push(z);
                }
            }
            None => return None,
        }
    }
    Some(res)
}

pub(crate) fn read_newline<T: FInteger>(x: String) -> Option<Vec<T>> {
    let mut res = vec![];
    for i in x.split_whitespace() {
        match T::from_str(i) {
            Some(z) => res.push(z),
            None => return None,
        }
    }
    Some(res)
}

pub(crate) fn read_binary<T: FInteger>(x: Vec<u8>) -> Vec<T> {
    let mut res = vec![];
    let stride = T::byte_length();
    for i in 0..(x.len() / stride){
        res.push(T::from_bytes(&x[i * stride..(i + 1) * stride]))
    }
    res
}
