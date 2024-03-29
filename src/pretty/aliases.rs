pub type Vector<T> = Vec<T>;
pub type StringSlice<'a> = &'a str;

pub trait Iteratable<T> {
    fn iterate(&self) -> std::slice::Iter<'_, T>;
}

impl<T>Iteratable<T> for [T] {
    fn iterate(&self) -> std::slice::Iter<'_, T> {
        self.iter()
    }
}