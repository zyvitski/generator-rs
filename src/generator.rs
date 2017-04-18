///Basic Signal Generator Trait
pub trait Generator<T>: Range<T> + Domain<T> + Iterator<Item = T>
    where T: Default
{
    ///Sync to Default value for T
    fn sync(&mut self) {
        self.sync_to(T::default())
    }
    ///Sync to Specified Value
    fn sync_to(&mut self, value: T);
    ///Set the Normalized Frequency
    fn set_frequency(&mut self, value: T);
    ///Get the Normalized Frequency
    fn get_frequency(&self) -> &T;
}

///Describes the Mathematical Range of a Type
pub trait Range<T> {
    fn range(&self) -> &(T, T);
}
///Describes the Mathematical Domain of a Type
pub trait Domain<T> {
    fn domain(&self) -> &(T, T);
}
