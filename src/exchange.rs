///Exchanges two values
pub trait Exchange {
    fn exchange(&mut self, value: Self) -> Self;
}

impl<T> Exchange for T
    where T: Copy
{
    #[inline]
    fn exchange(&mut self, value: Self) -> Self {
        let out = *self;
        *self = value;
        out
    }
}
