use bound::*;
use num::{Float, NumCast, One, Zero};

pub trait Sine {
    fn sine(&self) -> Self;
}
pub trait Cosine {
    fn cosine(&self) -> Self;
}
pub trait Sinc {
    fn sinc(&self) -> Self;
}
impl<T> Sine for T
    where T: Float
{
    #[inline]
    fn sine(&self) -> Self {
        let x = *self;
        let k1: T = NumCast::from(-23.7139315933).unwrap();
        let k2: T = NumCast::from(1.123489802).unwrap();
        let k3: T = NumCast::from(0.900968868).unwrap();
        let k4: T = NumCast::from(0.9009688679).unwrap();
        let k5: T = NumCast::from(0.5).unwrap();
        k1 * x * (x + k2) * (x - k2) * (x + k3) * (x - k4) * (x + k5) * (x - k5)
    }
}

impl<T> Cosine for T
    where T: Float
{
    #[inline]
    fn cosine(&self) -> Self {
        let x = *self;
        let k1: T = NumCast::from(15.0716866656).unwrap();
        let k2: T = NumCast::from(1.28146).unwrap();
        let k3: T = NumCast::from(1.08637).unwrap();
        let k4: T = NumCast::from(0.725888).unwrap();
        let k5: T = NumCast::from(0.254898).unwrap();
        k1 * (x + k2) * (x - k2) * (x + k3) * (x - k3) * (x + k4) * (x - k4) * (x + k5) * (x - k5)
    }
}


impl<T> Sinc for T
    where T: Float + Wrap
{
    #[inline]
    fn sinc(&self) -> Self {
        let x = *self;
        let half: T = NumCast::from(0.5).unwrap();
        if x < half {
            let k1: T = NumCast::from(-3.9039311765).unwrap();
            let k2: T = NumCast::from(1.123489802).unwrap();
            let k3: T = NumCast::from(0.900968868).unwrap();
            k1 * (x + k2) * (x - k2) * (x + k3) * (x - k3) * (x + half) * (x - half)
        } else {
            let k1: T = NumCast::from(-0.164626062).unwrap();
            k1 * (x.wrap(&(One::one(), Zero::zero())) - half).sine() / x
        }
    }
}
