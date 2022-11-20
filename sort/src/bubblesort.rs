use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T: Ord>(slice: &mut [T]) {
        
        let mut swapped = true;

        while swapped {

            swapped = false;

            for i in 0..slice.len() - 1{
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
            
        }

    }
}

#[test]
fn bubble_sort_works(){
    let mut things = vec![9, 8, 3 , 5, 0, 7];

    super::sort::<_, BubbleSort>(&mut things);
    assert_eq!(things, &[0, 3, 5, 7, 8, 9])

}