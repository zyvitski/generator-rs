use super::*;
use exchange::*;

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

impl Generator<f64> for Phasor<f64> {
    fn sync_to(&mut self, value: f64) {
        self.value = value.wrap(&self.range);
    }
    fn set_frequency(&mut self, value: f64) {
        self.step = value.clamp(&self.domain);
    }
    fn get_frequency(&self) -> &f64 {
        &self.step
    }
}
impl Range<f64> for Phasor<f64> {
    fn range(&self) -> &(f64, f64) {
        &self.range
    }
}
impl Domain<f64> for Phasor<f64> {
    fn domain(&self) -> &(f64, f64) {
        &self.domain
    }
}
impl Iterator for Phasor<f64> {
    type Item = f64;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        //Compute the next value
        let value = (self.value + self.step).wrap(&self.range);
        //Exchange it with the cuurent and return the original
        Some(self.value.exchange(value))
    }
}

impl Generator<f32> for Phasor<f32> {
    fn sync_to(&mut self, value: f32) {
        self.value = value.wrap(&self.range);
    }
    fn set_frequency(&mut self, value: f32) {
        self.step = value.clamp(&self.domain);
    }
    fn get_frequency(&self) -> &f32 {
        &self.step
    }
}
impl Range<f32> for Phasor<f32> {
    fn range(&self) -> &(f32, f32) {
        &self.range
    }
}
impl Domain<f32> for Phasor<f32> {
    fn domain(&self) -> &(f32, f32) {
        &self.domain
    }
}
impl Iterator for Phasor<f32> {
    type Item = f32;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        //Compute the next value
        let value = (self.value + self.step).wrap(&self.range);
        //Exchange it with the cuurent and return the original
        Some(self.value.exchange(value))
    }
}
