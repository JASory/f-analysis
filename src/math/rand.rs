use machine_prime::is_prime;

#[inline(always)]
fn default_xor() -> u64 {
    let mut x = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    x ^= x.wrapping_shr(12);
    x ^= x.wrapping_shl(25);
    x ^= x.wrapping_shr(27);
    x.wrapping_mul(0x2545F4914F6CDD1D)
}

#[allow(unreachable_code)]
pub(crate) fn rand() -> u64 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("rdrand") {
            // USE RDRAND if possible
            let mut x: u64 = 0;
            let flag = unsafe { core::arch::x86_64::_rdrand64_step(&mut x) };
            // Return the XORShift if RDRAND failed
            if flag == 0 {
                return default_xor();
            }

            return x;
        } // If processor is x86 and does not support RDRAND use xor shift
        return default_xor();
    }

    {
        // All other architectures use xor shift
        default_xor()
    }
}

pub fn gen_k(k: u64)-> Option<u64>{
    if k > 64 {
        return None;
    }
    if k < 1 {
        return None;
    }
    if k == 1 {
        return Some(2);
    }

    let form = 1 << (k - 1);
    let bitlength = form - 1;
    
    Some((rand() & bitlength) | form)
}

// Random k-bit Natural guaranteed to be composite
pub fn comp_gen_k(k: u64) -> Option<u64> {
    if k > 64 {
        return None;
    }
    if k < 1 {
        return None;
    }
    if k == 1 {
        return Some(2);
    }

    let form = 1 << (k - 1);
    let bitlength = form - 1;

    loop {
        let p = rand();
        if !is_prime((p & bitlength) | form) {
            return Some((p & bitlength) | form);
        }
    }
}

// Random k-bit Natural guaranteed to be prime
pub fn prime_gen_k(k: u64) -> Option<u64> {
    if k > 64 {
        return None;
    }
    if k < 1 {
        return None;
    }
    if k == 1 {
        return Some(2);
    }

    let form = (1 << (k - 1)) + 1;
    let bitlength = form - 2;

    loop {
        let p = rand();
        if is_prime((p & bitlength) | form) {
            return Some((p & bitlength) | form);
        }
    }
}
