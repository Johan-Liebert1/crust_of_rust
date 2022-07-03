use super::orst::Sorter;

pub struct InsertionSort;

impl<T> Sorter<T> for InsertionSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted
            // take slice[unsorted] and place in sorted locationin slice[..=unsorted]

            let mut i = unsorted;

            while i > 0 && slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);
                i -= 1;
            }
        }
    }
}

#[test]
fn insertion_sort_works() {
    use super::orst::Sorter;

    let mut things = vec![4, 2, 3, 1];
    InsertionSort.sort(&mut things);

    assert_eq!(things, &[1, 2, 3, 4]);
}
