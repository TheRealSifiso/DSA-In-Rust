use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for i in 0..slice.len()-1 {
            let mut j = i + 1;
            while j != 0 && slice[j] < slice[j - 1] {
                slice.swap(j, j - 1);
                j -= 1;
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
