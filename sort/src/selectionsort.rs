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
                .min_by_key(|tuple| tuple.1) //If the iterator is empty, None is returned
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

/*
The following is the complete explanation by ChatGPT of how min_by_key works:

    min_by_key is a method that is available on Rust's Iterator trait. It is
    used to find the element of an iterator that has the minimum value when
    compared using a specific key function. The key function is a closure
    that is applied to each element of the iterator, and the resulting value
    is used to compare the elements.

    Here is an example of how you might use min_by_key:

    '''
    use std::cmp::Ordering;

    fn main() {
        let numbers = vec![3, 5, 1, 2, 4];

        let min = numbers.iter().min_by_key(|x| *x);

        println!("The minimum number is {:?}", min);
    }

    '''

    In this example, the key function is |x| *x, which simply returns the value
    of x itself. This means that the minimum value will be found by comparing
    the elements of the iterator using their own values. The output of this
    program will be "The minimum number is Some(1)".

    You can also use a different key function if you want to find the minimum
    value based on some other criterion. For example, you might use a key
    function that returns the length of a string, in which case min_by_key
    would find the shortest string in the iterator.

    min_by_key returns an Option<T>, where T is the type of the element being
    iterated over. If the iterator is empty, min_by_key returns None. If the
    iterator is not empty, it returns Some(T) with the element that has the minimum
    value according to the key function.

*/