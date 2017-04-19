use super::*;
use num::Float;
///A General Purpose Phase Generator
pub struct Phasor<T> {
    value: T,
    step: T,
    range: (T, T),
    domain: (T, T),
}

impl<T> Phasor<T>
    where T: Default + PartialEq + PartialOrd + Copy
{
    pub fn new(step: T, domain: (T, T), range: (T, T)) -> Self {
        Phasor {
            value: T::default(),
            step: step.clamp(&domain),
            range: range,
            domain: domain,
        }
    }
}

impl<T> Generator<T> for Phasor<T>
    where T: Float + Default
{
    fn sync_to(&mut self, value: T) {
        self.value = value.wrap(&self.range);
    }
    fn set_frequency(&mut self, value: T) {
        self.step = value.clamp(&self.domain);
    }
    fn get_frequency(&self) -> &T {
        &self.step
    }
}
impl<T> Range<T> for Phasor<T>
    where T: Float
{
    fn range(&self) -> &(T, T) {
        &self.range
    }
}
impl<T> Domain<T> for Phasor<T>
    where T: Float
{
    fn domain(&self) -> &(T, T) {
        &self.domain
    }
}
impl<T> Iterator for Phasor<T>
    where T: Float
{
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        //Compute the next value
        let value = (self.value + self.step).wrap(&self.range);
        //Exchange it with the cuurent and return the original
        Some(self.value.exchange(value))
    }
}
