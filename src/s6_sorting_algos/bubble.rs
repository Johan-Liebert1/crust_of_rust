use super::orst::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;

        while swapped {
            swapped = false;

            // this will panic on an empty slice, but YOLO
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}
#[test]
fn bubble_sort_works() {
    let mut things = vec![4, 2, 3, 1];
    super::orst::sort::<_, BubbleSort>(&mut things);

    assert_eq!(things, &[1, 2, 3, 4]);
}
