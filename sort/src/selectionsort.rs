use crate::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 0..slice.len(){
            let smallest_in_rest= slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|tuple| tuple.1)
                .map(|tuple| unsorted + tuple.0) // Maps an Option<T> to Option<U> by applying a function to a contained value.
                .expect("slice is non-empty");

                /*
                min_by_key is used to find the element of an iterator that has the
                minimum value when compared using a specific key function.

                The key function is a closure that is applied to each element of the
                iterator, and the resulting value (of the closure) is used to
                compare the elements.

                In the above case, we call the enumerate() method which consumes the
                iterator type 'Iter<T>' and returns an iterable that gives a tuple
                containing the current iteration count (the index) as well as the
                next value.
                */

            slice.swap(smallest_in_rest, unsorted)
        }
    }
}