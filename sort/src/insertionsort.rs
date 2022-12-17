use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            let mut i = unsorted;

            while i > 0 && slice[i - 1] > slice[i] {
                slice.swap(i, i - 1);
                i -= 1;
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
