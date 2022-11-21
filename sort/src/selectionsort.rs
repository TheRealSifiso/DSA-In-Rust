use crate::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T: Ord>(slice: &mut [T]) {
        for unsorted in 0..slice.len() {
            let mut smallest_in_rest = unsorted;

            for i in (unsorted + 1)..slice.len() {
                if slice[i] < slice[smallest_in_rest] {
                    smallest_in_rest = i;
                }
            }

            if smallest_in_rest != unsorted {
                slice.swap(smallest_in_rest, unsorted);
            }
        }
    }
}

#[test]
fn selection_sort_works(){
    let mut things = vec![9, 8, 3 , 5, 0, 7];

    super::sort::<_, SelectionSort>(&mut things);
    assert_eq!(things, &[0, 3, 5, 7, 8, 9])
}