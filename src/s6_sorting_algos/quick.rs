use super::orst::Sorter;

pub struct QuickSort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non empty");

    let (mut left, mut right) = (0 as i32, (rest.len() - 1) as i32);

    while left <= right && left < rest.len() as i32 && right >= 0 {
        if &rest[left as usize] <= pivot {
            // already on the correct side
            left += 1;
        } else if &rest[right as usize] > pivot {
            right -= 1;
        } else {
            rest.swap(left as usize, right as usize);
            left += 1;
            right -= 1;
        }
    }

    // placing the pivot at the correct place
    slice.swap(0, left as usize);

    let (left, right) = slice.split_at_mut(left as usize);
    quicksort(left);
    quicksort(&mut right[1..]);
}

impl<T> Sorter<T> for QuickSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        quicksort(slice);
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
fn quick_sort_works() {
    use super::orst::Sorter;

    let mut things = vec![2, 4, 1, 3, 5];
    QuickSort.sort(&mut things);

    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
