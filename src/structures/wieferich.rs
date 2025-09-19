use crate::natural::ftrait::Natural;

/// Set of generalised Wieferich primes. Primes such that a^(p-1) mod p^2 = 1
#[derive(Debug, Clone)]
pub struct WieferichPrime {
    base: u64,
    primes: Vec<u64>,
}

impl WieferichPrime {
    pub(crate) fn new(base: u64, primes: Vec<u64>) -> Self {
        Self { base, primes }
    }

    pub fn base(&self) -> u64 {
        self.base
    }

    pub fn prime_set(&self) -> Vec<u64> {
        self.primes.clone()
    }

    pub fn iter(&self) -> std::slice::Iter<u64> {
        self.primes.iter()
    }

    pub fn cardinality(&self) -> usize {
        self.primes.len()
    }

    pub fn pseudoprimes(&self, b: u64) -> Vec<u64> {
        let mut res = vec![];
        for i in self.primes.iter() {
            let psqr = i * i;
            let mut start = 1;
            for j in 0..b / psqr {
                start += (i - 1);
                let n = start * psqr;
                if n > b {
                    break;
                }
                if self.base.exp_residue(n - 1, n) == 1 {
                    res.push(n);
                }
            }
        }
        res
    }
}

impl std::fmt::Display for WieferichPrime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = self.base.to_string() + " p: ";
        let pstring = self
            .primes
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let interim = pstring.join(",");
        let res = b + &interim + " ";
        write!(f, "{}", res)
    }
}
