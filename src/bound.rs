///Wraps a value into bounds
use num::Float;
pub trait Wrap: Sized {
    fn wrap(&self, bounds: &(Self, Self)) -> Self;
}

impl<T> Wrap for T
    where T: Float
{
    #[inline]
    fn wrap(&self, bounds: &(Self, Self)) -> Self {
        let diff = bounds.1 - bounds.0;
        let mut value = *self;
        while value > bounds.1 {
            value = value - diff;
        }
        while value < bounds.0 {
            value = value + diff;
        }
        value
    }
}
///Clamps a value into bounds
pub trait Clamp: Sized {
    fn clamp(&self, bounds: &(Self, Self)) -> Self;
}

impl<T> Clamp for T
    where T: PartialOrd + PartialEq + Copy
{
    #[inline]
    fn clamp(&self, bounds: &(Self, Self)) -> Self {
        let value = *self;
        let (lower, upper) = *bounds;
        if value > upper {
            upper
        } else if value < lower {
            lower
        } else {
            value
        }
    }
}
