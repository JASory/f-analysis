#[derive(Clone, Copy)]
pub(crate) struct Signed<
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::Mul<Output = T>,
>(pub bool, pub T);

impl<
        T: Copy
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Rem<Output = T>
            + std::ops::Mul<Output = T>
            + std::cmp::PartialOrd,
    > Signed<T>
{
    pub(crate) fn sub(a: Self, b: Self) -> Self {
        match (a.0, b.0) {
            (false, false) => {
                if b.1 >= a.1 {
                    Signed(true, b.1 - a.1)
                } else {
                    Signed(false, a.1 - b.1)
                }
            }
            (false, true) => Signed(false, a.1 + b.1), //Sign(,a.1-b.1)
            (true, true) => {
                if b.1 >= a.1 {
                    Signed(false, b.1 - a.1)
                } else {
                    Signed(true, a.1 - b.1)
                }
            }
            (true, false) => Signed(true, a.1 + b.1),
        }
    }

    pub(crate) fn prod(a: Self, quo: T) -> Self {
        Self(a.0, a.1 * quo)
    }

    pub(crate) fn residue(a: Self, res: T) -> T {
        if !a.0 {
            let k = a.1 % res;
            res - k
        } else {
            a.1 % res
        }
    }
}
