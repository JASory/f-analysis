
pub(crate) fn gcd(x: u64, y: u64) -> u64 {
    let mut a = x;
    let mut b = y;
    if b == 0 {
        return a;
    } else if a == 0 {
        return b;
    }

    let self_two_factor = a.trailing_zeros();
    let other_two_factor = b.trailing_zeros();
    let min_two_factor = std::cmp::min(self_two_factor, other_two_factor);
    a >>= self_two_factor;
    b >>= other_two_factor;
    loop {
        if b > a {
            std::mem::swap(&mut b, &mut a);
        }
        a -= b;

        if a == 0 {
            return b << min_two_factor;
        }
        a >>= a.trailing_zeros();
    }
}

pub(crate) fn jacobi(x: u64, k: u64) -> i8 {
    let mut n = x;
    let mut p = k;
    let mut t = 1i8;
    n %= p;

    while n != 0 {
        let zeros = n.trailing_zeros();
        n >>= zeros;

        if (p % 8 == 3 || p % 8 == 5) && (zeros % 2 == 1) {
            t = -t
        }

        std::mem::swap(&mut n, &mut p);
        if n % 4 == 3 && p % 4 == 3 {
            t = -t;
        }
        n %= p;
    }

    if p == 1 {
        t
    } else {
        0
    }
}

pub(crate) fn isqrt(x: u64) -> u64 {
    let mut est = (x as f64).sqrt() as u64 + 1;

    loop {
        let s = est;
        let t = s + x / s;
        est = t >> 1;
        if est >= s {
            return s;
        }
    }
}
