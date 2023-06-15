use crate::fermat::FInteger;
/*
   In: Vector of 64-bit integers
   Out: Vector of 8-bit integers, converted from little-endian
*/

pub(crate) fn write_binary<T: FInteger>(x: &Vec<T>) -> Vec<u8> {
   x.iter().map(|z| z.to_bytes()).flatten().collect::<Vec<u8>>()
}
/*
   In: Vector of 64-bit integers
   Out: String of integers separated by newline
*/
#[allow(dead_code)]
pub(crate) fn format_newline<T: FInteger>(x: Vec<T>) -> String {
    let strout = x.iter().map(|k| k.to_string()).collect::<Vec<String>>();
    strout.join("\n")
}
/*
   In: An integer
   Out: UTF-8 string representation of integer
*/

fn format<T: FInteger>(x: T) -> String {
    " ".to_owned() + &x.to_string()
}

/*
Note that this may also be used to write to csv

  In: A vector of 64-bit integers
  Out: A string of comma-separated integers with rows of length S
*/

pub(crate) fn format_block<const S: usize, T: FInteger>(x: &Vec<T>) -> String {
    let mut stringvec = vec![];

    for (idx, el) in x.iter().enumerate() {
        if idx % S == 0 && idx & 1 == 0 {
            stringvec.push("\n".to_string());
            stringvec.push("\t".to_string());
            stringvec.push(format(*el));
            stringvec.push(",".to_string());
        } else if idx % S == S - 1 {
            stringvec.push(format(*el));
            stringvec.push(",".to_string());
        } else if idx % S != S - 1 {
            stringvec.push(format(*el));
            stringvec.push(",".to_string());
        }
    }
    stringvec.join("")
}
