use super::aliases::Iteratable;

pub trait SubsetComparable<T> {
    fn collection_equals(&self, other: &[T]) -> bool  where T: PartialEq;
    fn is_subset_of(&self, other: &[T]) -> bool  where T: PartialEq;
    fn is_superset_of(&self, other: &[T]) -> bool  where T: PartialEq;
}

impl<T>SubsetComparable<T> for [T] {
    fn collection_equals(&self, other: &[T]) -> bool  where T: PartialEq {

        return self.is_subset_of(other) && other.is_subset_of(self);
    }

    fn is_subset_of(&self, other: &[T]) -> bool  where T: PartialEq {
        return self.iterate().all(|element| other.contains(&element));
    }

    fn is_superset_of(&self, other: &[T]) -> bool  where T: PartialEq {
        return other.iterate().all(|element| self.contains(&element));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_superset_of_be_true_when_this_contains_all_of_other() {
        
        let is_superset = vec![1,2,3,4].is_superset_of(&vec![1,2,3]);
        assert!(is_superset);
    }

    #[test]
    fn is_superset_of_be_true_when_this_contains_exactly_same_elements_as_other() {

        let is_superset = vec![1,2,3].is_superset_of(&vec![1,2,3]);
        assert!(is_superset);
    }

    #[test]
    fn is_subset_of_be_true_when_other_contains_all_of_this() {

        let is_subset = vec![1,3,2].is_subset_of(&vec![2,4,3,1]);
        assert!(is_subset);
    }

    #[test]
    fn is_superset_of_be_true_when_other_contains_exactly_same_elements_as_this() {

        let is_subset = vec![1,2,3,4].is_subset_of(&vec![1,3,2,4]);
        assert!(is_subset);
    }

    #[test]
    fn collection_equals_be_true_when_contains_same_elements() {
        
        let is_subset = vec![1,2,5].collection_equals(&vec![2,5,1]);
        assert!(is_subset);
    }

    #[test]
    fn collection_equals_be_false_when_any_element_different() {
        
        let is_subset = vec![1,2,4].collection_equals(&vec![1,2,5]);
        assert!(!is_subset);
    }
}
