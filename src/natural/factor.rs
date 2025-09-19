use crate::natural::rand::drbg;
use crate::primes::SMALL_PRIMES;
use crate::Natural;
/*

*/

pub struct Factorization<T: Natural> {
    pub factors: Vec<T>,
    pub powers: Vec<u32>,
}

impl<T: Natural> Factorization<T> {
    fn new() -> Self {
        Self {
            factors: vec![],
            powers: vec![],
        }
    }

    pub fn pair_iter(&self) -> std::iter::Zip<std::slice::Iter<T>, std::slice::Iter<u32>> {
        self.factors.iter().zip(self.powers.iter())
    }
}

impl<T: Natural> std::fmt::Display for Factorization<T> {
    fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        let len = self.factors.len();
        if self.powers[0] != 1 {
            output += &(self.factors[0].to_string() + "^" + &self.powers[0].to_string());
        } else {
            output += &(self.factors[0].to_string());
        }
        if len > 1 {
            for i in 1..len {
                if self.powers[i] != 1u32 {
                    let pair = self.factors[i].to_string() + "^" + &self.powers[i].to_string();
                    output += &(" * ".to_owned() + &pair);
                } else {
                    let pair = self.factors[i].to_string();
                    output += &(" * ".to_owned() + &pair);
                }
            }
        }
        write!(format, "{}", output)
    }
}

const fn poly_eval(x: u64, subtrahend: u64, n: u64, npi: u64) -> u64 {
    machine_prime::mont_sub(machine_prime::mont_prod(x, x, n, npi), subtrahend, n)
}

fn pollard_brent(base: u64, inv: u64, subtrahend: u64, n: u64) -> Option<u64> {
    let m = 128;
    let mut r = 1;
    let mut q = 1;
    let mut g = 1;
    let mut ys = 1;
    let mut y = base;
    let mut x = y;
    let mut cycle = 0;

    while cycle < 17 {
        cycle += 1;
        x = y;

        let mut yloop = 0;

        while yloop < r {
            yloop += 1;
            y = poly_eval(y, subtrahend, n, inv);
        }

        let mut k = 0;

        loop {
            let mut i = 0;

            while i < m * cycle {
                if i >= r - k {
                    break;
                }

                y = poly_eval(y, subtrahend, n, inv);
                q = machine_prime::mont_prod(q, x.abs_diff(y), n, inv);
                i += 1;
            } // end loop

            ys = y;
            g = q.gcd(n);
            k += m;
            if k >= r || g != 1 {
                break;
            }
        }

        r <<= 1;
        if g != 1 {
            break;
        }
    }

    if g == n {
        while g == 1 {
            ys = poly_eval(ys, subtrahend, n, inv);
            g = x.abs_diff(ys).gcd(n);
        }
    }
    if g != 1 && g != n && machine_prime::is_prime_wc(g) {
        return Some(g);
    }
    None
}

/// Returns some prime factor of an 64-bit integer
///
/// This function uses the Pollard-rho algorithm and mostly exists for FFI
pub fn get_factor(n: u64) -> u64 {
    let inv = machine_prime::mul_inv2(n);
    let one = machine_prime::one_mont(n);
    let base = machine_prime::two_mont(one, n);

    match pollard_brent(base, inv, one, n) {
        Some(factor) => return factor,
        None => {
            // if x^2 -1 failed try x^2+1
            // No particular reason except to reuse some values
            // Coef here is the additive inverse of 1 over Z/nZ
            let coef = n.wrapping_sub(one);
            match pollard_brent(base, inv, coef, n) {
                Some(factor) => return factor,
                None => {
                    // Loop that has a roughly 0.5 probability of factoring each iteration
                    let mut param = drbg(n);
                    loop {
                        let rand_base = param % (n - 3) + 3;
                        match pollard_brent(rand_base, inv, one, n) {
                            Some(factor) => return factor,
                            None => param = drbg(param),
                        }
                    }
                }
            }
        }
    }
}

pub fn factorize(mut n: u64) -> Factorization<u64> {
    let mut t = Factorization::new();

    let mut idx = 0usize;

    if n == 0 {
        return t;
    }
    if n == 1 {
        t.factors.push(1);
        t.powers.push(1);
        return t;
    }

    let twofactor = n.trailing_zeros();

    if twofactor != 0 {
        t.factors.push(2u64);
        t.powers.push(twofactor);
        n >>= twofactor;
    }

    let mut i = 0usize;
    while i < 53 {
        let fctr = SMALL_PRIMES[i] as u64;
        // strips out small primes
        if n % fctr == 0 {
            t.factors.push(fctr);
            let mut count = 0u32;
            while n % fctr == 0 {
                count += 1;
                n /= fctr;
            }
            t.powers.push(count);
        }
        i += 1;
    }

    if n == 1 {
        return t;
    }

    if machine_prime::is_prime_wc(n) {
        t.factors.push(n);
        t.powers.push(1);
        return t;
    }
    while n != 1 {
        let k = get_factor(n);
        t.factors.push(k);
        let mut count = 0u32;
        while n % k == 0 {
            count += 1;
            n /= k;
        }
        t.powers.push(count);
        if n == 1 {
            return t;
        }
        if machine_prime::is_prime_wc(n) {
            t.factors.push(n);
            t.powers.push(1);
            return t;
        }
    }
    t
}

const fn poly_eval_128(x: u128, subtrahend: u128, n: u128, npi: u128) -> u128 {
    machine_prime::mont_sub_128(machine_prime::mont_sqr_128(x, n, npi), subtrahend, n)
}

fn pollard_brent_128(base: u128, inv: u128, subtrahend: u128, n: u128) -> Option<u128> {
    let m = 512;
    let mut r = 1;
    let mut q = 1;
    let mut g = 1;
    let mut ys = 1;
    let mut y = base;
    let mut x = y;
    let mut cycle = 0;

    while cycle < 33 {
        cycle += 1;
        x = y;

        let mut yloop = 0;

        while yloop < r {
            yloop += 1;
            y = poly_eval_128(y, subtrahend, n, inv);
        }

        let mut k = 0;

        loop {
            let mut i = 0;

            while i < m * cycle {
                if i >= r - k {
                    break;
                }

                y = poly_eval_128(y, subtrahend, n, inv);
                q = machine_prime::mont_prod_128(q, x.abs_diff(y), n, inv);
                i += 1;
            } // end loop

            ys = y;
            g = q.gcd(n);
            k += m;
            if k >= r || g != 1 {
                break;
            }
        }

        r <<= 1;
        if g != 1 {
            break;
        }
    }

    if g == n {
        while g == 1 {
            ys = poly_eval_128(ys, subtrahend, n, inv);
            g = x.abs_diff(ys).gcd(n);
        }
    }
    if g != 1 && g != n && machine_prime::is_prime_wc_128(g) {
        return Some(g);
    }
    None
}

/// Returns some prime factor of an 128-bit integer
pub fn get_factor_128(n: u128) -> u128 {
    // Possible optimisations
    // base and one don't have to be computed they just need to be less than n
    // Not sure if initialisation is a bottleneck in easy composites

    // Alternate base selection methods could be selecting a jacobi(a^2 -1,n) == 1
    let inv = machine_prime::mul_inv2_128(n);
    let one = machine_prime::one_mont_128(n);
    let base = machine_prime::two_mont_128(one, n);

    match pollard_brent_128(base, inv, one, n) {
        Some(factor) => return factor,
        None => {
            // if x^2 -1 failed try x^2+1
            // No particular reason except to reuse some values
            let coef = n.wrapping_sub(one);
            match pollard_brent_128(base, inv, coef, n) {
                Some(factor) => return factor,
                None => {
                    // Loop that has a roughly 0.5 probability of factoring each iteration
                    // The probability of being unable to factor a composite is 1/2^period and the period is likely very large
                    // So the risk of error is vanishingly small
                    let mut param = drbg(n as u64);
                    loop {
                        let rand_base = (param as u128) % (n - 3) + 3;
                        match pollard_brent_128(rand_base, inv, one, n) {
                            Some(factor) => return factor,
                            None => param = drbg(param),
                        }
                    }
                }
            }
        }
    }
}

pub fn factorize_128(mut n: u128) -> Factorization<u128> {
    let mut t = Factorization::new();

    let mut idx = 0usize;

    if n == 0 {
        return t;
    }
    if n == 1 {
        t.factors.push(1);
        t.powers.push(1);
        return t;
    }

    let twofactor = n.trailing_zeros();

    if twofactor != 0 {
        t.factors.push(2u128);
        t.powers.push(twofactor);
        n >>= twofactor;
    }

    let mut i = 0usize;
    while i < 53 {
        let fctr = SMALL_PRIMES[i] as u128;
        // strips out small primes
        if n % fctr == 0 {
            t.factors.push(fctr);
            let mut count = 0u32;
            while n % fctr == 0 {
                count += 1;
                n /= fctr;
            }
            t.powers.push(count);
        }
        i += 1;
    }

    if n == 1 {
        return t;
    }

    if machine_prime::is_prime_wc_128(n) {
        t.factors.push(n);
        t.powers.push(1);
        return t;
    }
    while n != 1 {
        let k = get_factor_128(n);
        t.factors.push(k);
        let mut count = 0u32;
        while n % k == 0 {
            count += 1;
            n /= k;
        }
        t.powers.push(count);
        if n == 1 {
            return t;
        }
        if machine_prime::is_prime_wc_128(n) {
            t.factors.push(n);
            t.powers.push(1);
            return t;
        }
    }
    t
}
