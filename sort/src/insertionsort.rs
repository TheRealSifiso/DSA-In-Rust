use super::Sorter;

pub struct InsertionSort {
    pub smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {

            if !self.smart {
                let mut i = unsorted;

                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i, i - 1);
                    i -= 1;
                }
            } else {
                let index = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(index) => index,
                    Err(index) => index,
                };

                //unsorted = the index of the element we want to insert.
                slice[index..=unsorted].rotate_right(1); //Rotating this chosen subslice swops elements at index 'i' and index 'unsorted'
                
                //rotate_right(k) shifts all elements to the 'right' by 'k' number of steps whilst wrapping arround
                //e.g. [1, 2, 3, 4, 5].rotate_right(1) -> [5, 1, 2, 3, 4]
                //e.g. [1, 2, 3, 4, 5].rotate_right(2) -> [4, 5, 1, 2, 3]
                //e.g. [1, 2, 3, 4, 5][..=2].rotate_right(1) -> [3, 1, 2, 4, 5]
            }
        }
    }
}

/*
Insertion Sort:
    -> Iterate over the collection
    -> Compare each item to all items before it
    -> If the item is in the wrong position, move it back to insert it into
        the correct position

        A sorted partition will then form at the start of the collection,
        one item at a time
*/
