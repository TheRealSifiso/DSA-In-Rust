use super::Sorter;
pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;

        //Quit iterating after the last swop
        while swapped {
            swapped = false;

            //Iterate over the collection and swop consecutive items
            for i in 0..slice.len() - 1 {
                if slice[i + 1] < slice[i] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}

/*
Bubble Sort:
    -> Iterate over the collection
	-> Compare consecutive items, then swap them!
		
		A sorted partition will then form at the end of the collection,
		one item at a time.
	
	-> Swap
*/