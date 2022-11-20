use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T: Ord>(slice: &mut [T]) {

        for unsorted in 1..slice.len() {

            let mut i = unsorted;

            while i > 0 && slice[i - 1] > slice[i]{
                slice.swap(i - 1, i);
                i -= 1;
            }
        }

    }
}

#[test]
fn insertion_sort_works(){
    let mut things = vec![9, 8, 3 , 5, 0, 7];

    super::sort::<_, InsertionSort>(&mut things);
    assert_eq!(things, &[0, 3, 5, 7, 8, 9])

}