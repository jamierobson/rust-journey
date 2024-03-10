pub type Vector<T> = Vec<T>;


pub trait Iteratable<T> {
    fn iterate(&self) -> std::slice::Iter<'_, T>;
}

impl<T>Iteratable<T> for [T] {
    fn iterate(&self) -> std::slice::Iter<'_, T> {
        self.iter()
    }
}