/*
   Enum returning the result of a fused Fermat-Sprp test
   None if it is not a Fermat pseudoprime
   Fermat if it is only a Fermat pseudoprime
   Euler if it is both Euler-Jacobi and Fermat
   Strong if it is Fermat, EJ and Strong Pseudoprime
*/
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Pseudoprime {
    None,
    Fermat,
    Euler,
    Strong,
}

const INV_8: [u8; 128] = [
    0x01, 0xAB, 0xCD, 0xB7, 0x39, 0xA3, 0xC5, 0xEF, 0xF1, 0x1B, 0x3D, 0xA7, 0x29, 0x13, 0x35, 0xDF,
    0xE1, 0x8B, 0xAD, 0x97, 0x19, 0x83, 0xA5, 0xCF, 0xD1, 0xFB, 0x1D, 0x87, 0x09, 0xF3, 0x15, 0xBF,
    0xC1, 0x6B, 0x8D, 0x77, 0xF9, 0x63, 0x85, 0xAF, 0xB1, 0xDB, 0xFD, 0x67, 0xE9, 0xD3, 0xF5, 0x9F,
    0xA1, 0x4B, 0x6D, 0x57, 0xD9, 0x43, 0x65, 0x8F, 0x91, 0xBB, 0xDD, 0x47, 0xC9, 0xB3, 0xD5, 0x7F,
    0x81, 0x2B, 0x4D, 0x37, 0xB9, 0x23, 0x45, 0x6F, 0x71, 0x9B, 0xBD, 0x27, 0xA9, 0x93, 0xB5, 0x5F,
    0x61, 0x0B, 0x2D, 0x17, 0x99, 0x03, 0x25, 0x4F, 0x51, 0x7B, 0x9D, 0x07, 0x89, 0x73, 0x95, 0x3F,
    0x41, 0xEB, 0x0D, 0xF7, 0x79, 0xE3, 0x05, 0x2F, 0x31, 0x5B, 0x7D, 0xE7, 0x69, 0x53, 0x75, 0x1F,
    0x21, 0xCB, 0xED, 0xD7, 0x59, 0xC3, 0xE5, 0x0F, 0x11, 0x3B, 0x5D, 0xC7, 0x49, 0x33, 0x55, 0xFF,
];
/*
    In: Natural n := n < 2^64
    Out:
*/
pub(crate) fn mod_inv64(n: u64) -> u64 {
    // inverse of odd n in  2^64
    let mut est = INV_8[((n >> 1) & 0x7F) as usize] as u64;
    est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
    est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
    est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
    est.wrapping_neg()
}

/*
    In:
    Out:
*/
pub fn x64_modn(x: u64, n: u64) -> u64 {
    (((x as u128) << 64) % (n as u128)) as u64
}

/*
    In:
    Out:
*/
fn montprod_64(x: u64, y: u64, n: u64, npi: u64) -> u64 {
    let input = x as u128 * y as u128;
    let tm = (input as u64).wrapping_mul(npi);
    let (t, overflow) = input.overflowing_add((tm as u128) * (n as u128));
    let t = (t >> 64) as u64;

    if overflow {
        t + n.wrapping_neg()
    } else if t >= n {
        t - n
    } else {
        t
    }
}

/*
    In:
    Out:
*/
fn mpow_64(x: u64, p: u64, n: u64, npi: u64) -> u64 {
    let mut z = x64_modn(1u64, n);
    let mut base = x64_modn(x, n);
    let mut pow = p;

    while pow > 1 {
        if pow & 1 == 0 {
            base = montprod_64(base, base, n, npi);
            pow >>= 1;
        } else {
            z = montprod_64(base, z, n, npi);
            base = montprod_64(base, base, n, npi);
            pow = (pow - 1) >> 1
        }
    }
    montprod_64(base, z, n, npi)
}

fn odd_pow64(x: u64, p: u64, n: u64) -> u64 {
    let npi = mod_inv64(n);
    let interim = mpow_64(x, p, n, npi);
    montprod_64(1u64, interim, n, npi)
}

fn even_pow_64(x: u64, y: u64, reducer: u64) -> u64 {
    let mut z = 1u64;
    let mut base = x;

    let mut pow = y;
    if pow == 0 {
        return z;
    }

    while pow > 1 {
        if pow % 2 == 0 {
            base = base.wrapping_mul(base);
            pow >>= 1;
        } else {
            z = base.wrapping_mul(z);
            base = base.wrapping_mul(base);
            pow = (pow - 1) >> 1;
        }
    }

    base.wrapping_mul(z) & reducer
}

pub(crate) fn pow_64(x: u64, y: u64, n: u64) -> u64 {
    if n & 1 == 0 {
        let k = n.trailing_zeros() as u64;
        let s = n >> k;

        let reducer = (1 << k) - 1; // A shorthand for arithmetic over Z[2k]

        let k_rem = even_pow_64(x, y, reducer); //x.wrapping_pow(y as u32)&reducer;

        let s_rem = odd_pow64(x, y, s);

        let mut s_inv = s;

        for _ in 0..10 {
            // Multiplicative inverse over Z[2k]
            s_inv = 2u64.wrapping_sub(s_inv.wrapping_mul(s)).wrapping_mul(s_inv) & reducer;
        }

        let y = k_rem.wrapping_sub(s_rem).wrapping_mul(s_inv) & reducer;

        s_rem + s * y
    } else {
        odd_pow64(x, y, n)
    }
}

/*
    In:
    Out:
*/
pub(crate) fn fermat(p: u64, base: u64) -> bool {
    pow_64(base, p - 1, p) == 1
}

/*
    In:
    Out:
*/
pub(crate) fn sprp(p: u64, base: u64) -> bool {
    let p_minus = p - 1;
    let zeroes = p_minus.trailing_zeros();
    let d = p_minus >> zeroes;

    let npi = mod_inv64(p);
    let mut x = mpow_64(base, d, p, npi);
    let one = x64_modn(1, p);
    let oneinv = x64_modn(p_minus, p);
    if x == one || x == oneinv {
        return true;
    }
    for _ in 1..zeroes {
        x = montprod_64(x, x, p, npi);

        if x == oneinv {
            return true;
        }
    }
    false
}

/* Fix for even pseudoprime
    In:
    Out:
*/
// Fermat and Sprp test
pub(crate) fn fsprp(p: u64, base: u64) -> Pseudoprime {
   // let 
    let p_minus = p - 1;
    let zeroes = p_minus.trailing_zeros();
    let d = p_minus >> zeroes;

    let npi = mod_inv64(p);
   // println!("pminus {} npi {} zeroes{}",p_minus,npi,zeroes);
    let mut x = mpow_64(base, d, p, npi);
    let one = x64_modn(1, p);
    let oneinv = x64_modn(p_minus, p);
     //   println!("x {} one {} oneinv {}",x,one,oneinv);
    if x == one || x == oneinv {
        return Pseudoprime::Strong;
    }
    
    for _ in 1..zeroes {
        x = montprod_64(x, x, p, npi);

        if x == oneinv {
            return Pseudoprime::Strong;
        }
    }
   // println!("{} {}",zeroes,x);
    x = montprod_64(x, x, p, npi);
    if x == one {
        return Pseudoprime::Fermat;
    }
    Pseudoprime::None
}
