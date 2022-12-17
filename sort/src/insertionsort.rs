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
