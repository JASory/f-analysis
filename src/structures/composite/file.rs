use crate::filter::*;
use crate::Natural;
use std::io::BufRead;
use std::io::Read;
use std::io::Write;

pub(crate) fn filter_generic<T: Natural, F: GenericFilter>(
    file: std::fs::File,
    utf8_flag: bool,
    filter_flag: bool,
) -> Vec<T> {


    let mut file_in = std::io::BufReader::new(file.try_clone().unwrap());
    let mut k = Vec::<T>::new();
     // FIXME propagate error
    if utf8_flag {
        for i in file_in.lines() {
          match i{
            Ok(validline) => {
               match T::from_str(&validline){
                 Ok(val) => {
                  if F::filter_check(val) == filter_flag{
                    k.push(val);
                    }
                 }
                 Err(_) => (),
               }
            } 
            Err(_) => (),
          }
        }
    } else {
        let mut interim = vec![0u8; T::BYTE_LENGTH];

        loop {
            let totalbytes = file_in.read(&mut interim[..]).unwrap();

            if totalbytes == 0usize {
                break;
            }

            let val = T::from_bytes(&interim);

            if F::filter_check(val) == filter_flag {
                k.push(val);
            }
        }
    }
    k
}

// Parallelised file read and filter for non-UTF-8 values
pub(crate) fn filter_generic_par<T: Natural, F: GenericFilter>(
    file: std::fs::File,
    filter_flag: bool,
) -> Vec<T> {
    let read_eval = |mut f: std::fs::File, start: u64, stride: u64, flag: bool| -> Vec<T> {
        use std::io::Seek;
        let point = std::io::SeekFrom::Start(start * (T::BYTE_LENGTH as u64));
        f.seek(point);
        println!("Sought to {}", start * (T::BYTE_LENGTH as u64));
        let mut res = vec![];

        let mut interim = vec![0u8; T::BYTE_LENGTH];

        for i in 0..stride {
            let totalbytes = f.read(&mut interim[..]).unwrap();

            if totalbytes == 0usize {
                break; // possibly return Error as you should never get this
            }
            //             println!("{:?} ", interim);

            let val = T::from_bytes(&interim);
            //              println!("{}",val);
            if F::filter_check(val) == flag {
                res.push(val);
            }
        }
        res
    };

    let el_count = ((file.metadata().unwrap().len() as usize) / T::BYTE_LENGTH) as u64;
    let tc = crate::search::thread_count() as u64;
    let stride = (el_count / tc);

    println!("elements {} threads {} stride {} ", el_count, tc, stride);
    let mut thread_vec: Vec<std::thread::JoinHandle<Vec<T>>> = Vec::new();

    for i in 0..tc - 1 {
        let start = i * stride;
        let f = file.try_clone().unwrap();
        let inner_flag = filter_flag.clone();
        thread_vec.push(std::thread::spawn(move || {
            read_eval(f, start, stride.clone(), inner_flag)
        }));
    }

    thread_vec.push(std::thread::spawn(move || {
        read_eval(
            file,
            stride * (tc - 1),
            el_count - (stride * (tc - 1)),
            filter_flag,
        )
    }));

    let mut total = vec![];

    for handle in thread_vec {
        total.extend_from_slice(&handle.join().unwrap()[..]);
    }
    total
}

pub(crate) fn filter_generic_file<T: Natural, F: GenericFilter>(
    file: std::fs::File,
    out: std::fs::File,
    utf8_flag: bool,
    filter_flag: bool,
) {
    let mut file_in = std::io::BufReader::new(file.try_clone().unwrap());
    let mut file_out = std::io::BufWriter::new(file.try_clone().unwrap());

   if utf8_flag {
        for i in file_in.lines() {
          match i{
            Ok(validline) => {
               match T::from_str(&validline){
                 Ok(val) => {
                  if F::filter_check(val) == filter_flag{
                   file_out.write(&val.to_bytes()[..]).unwrap();
                    }
                 }
                 Err(_) => (),
               }
            } 
            Err(_) => (),
          }
        }
    } else {
        let mut interim = vec![0u8; T::BYTE_LENGTH];

        loop {
            let totalbytes = file_in.read(&mut interim[..]).unwrap();

            if totalbytes == 0usize {
                break;
            }

            let val = T::from_bytes(&interim);

            if F::filter_check(val) == filter_flag {
                file_out.write(&val.to_bytes()[..]).unwrap();
            }
        }
    }
    //k
}

pub(crate) fn filter_strong<T: Natural>(
    file: std::fs::File,
    utf8_flag: bool,
    filter_flag: bool,
    base: T,
) -> Vec<T> {
    let mut file_in = std::io::BufReader::new(file.try_clone().unwrap());
    let mut k = Vec::<T>::new();
      if utf8_flag {
        for i in file_in.lines() {
          match i{
            Ok(validline) => {
               match T::from_str(&validline){
                 Ok(val) => {
                  if val.sprp(base) == filter_flag{
                    k.push(val);
                    }
                 }
                 Err(_) => (),
               }
            } 
            Err(_) => (),
          }
        }

    } else {
        let mut interim = vec![0u8; T::BYTE_LENGTH];

        loop {
            let totalbytes = file_in.read(&mut interim[..]).unwrap();

            if totalbytes == 0usize {
                break;
            }

            let val = T::from_bytes(&interim);

            if val.sprp(base) == filter_flag {
                k.push(val);
            }
        }
    }
    k
}
