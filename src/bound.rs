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

pub trait PredicateElse<F>
    where F: Fn(&Self) -> bool
{
    fn predicate_else(&self, predicate: F, alternative: Self) -> Self;
}
impl<T, F> PredicateElse<F> for T
    where T: Copy,
          F: Fn(&Self) -> bool
{
    fn predicate_else(&self, predicate: F, alternative: Self) -> Self {
        if predicate(self) { *self } else { alternative }
    }
}

pub trait IsNormalElse: Float {
    fn is_normal_else(&self, value: Self) -> Self;
}

impl<T> IsNormalElse for T
    where T: Float
{
    fn is_normal_else(&self, value: Self) -> Self {
        self.predicate_else(move |x| x.is_normal(), value)
    }
}
