use crate::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 0..slice.len(){
            let mut smallest_in_rest = unsorted;

            for i in (unsorted + 1)..slice.len(){
                if slice[i] < slice[smallest_in_rest] {
                    smallest_in_rest = i;
                }
            }

            slice.swap(smallest_in_rest, unsorted)
        }
    }
}