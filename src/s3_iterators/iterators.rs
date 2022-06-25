#![allow(unused_variables)]
#![allow(dead_code)]

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            front_iter: None,
            back_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,           // outer thing implements iterator
    O::Item: IntoIterator, // items of the outer type implement IntoIterator
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.front_iter {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                // we have exhausted the front_iter iterator as next didn't give back any item
                self.front_iter = None;
            }

            if let Some(next_inner) = self.outer.next() {
                self.front_iter = Some(next_inner.into_iter())
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator, // outer thing implements iterator + DoubleEndedIterator
    O::Item: IntoIterator,  // items of the outer type implement IntoIterator
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator, // items of the outer type implement DoubleEndedIterator
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_iter) = self.back_iter {
                if let Some(i) = back_iter.next_back() {
                    return Some(i);
                }
                // we have exhausted the front_iter iterator as next didn't give back any item
                self.front_iter = None;
            }

            if let Some(next_inner) = self.outer.next_back() {
                self.back_iter = Some(next_inner.into_iter())
            } else {
                return self.front_iter.as_mut()?.next_back();
            }
        }
    }
}

pub fn tests() {
    println!("Testing iterators");

    // empty
    assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);

    // empty_wide
    assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0);

    // one
    assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);

    // two
    assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);

    // two_wide
    assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);

    // reverse
    assert_eq!(
        flatten(std::iter::once(vec!["a", "b"]))
            .rev()
            .collect::<Vec<_>>(),
        vec!["b", "a"]
    );

    // reverse_wide
    assert_eq!(
        flatten(vec![vec!["a"], vec!["b"]])
            .rev()
            .collect::<Vec<_>>(),
        vec!["b", "a"]
    );

    // both ends
    let mut iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
    assert_eq!(iter.next_back(), Some("b3"));
    assert_eq!(iter.next(), Some("a1"));
    assert_eq!(iter.next(), Some("a2"));
    assert_eq!(iter.next(), Some("a3"));
    assert_eq!(iter.next_back(), Some("b2"));
    assert_eq!(iter.next_back(), Some("b1"));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);

    // infinitely flatten
    let mut iter = flatten((0..).map(|i| 0..i));
    // 0 => 0..0 => empty
    // 1 => 0..1 => [0]
    // 2 => 0..2 => [0, 1]
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
}

fn iterator_examples() {
    let v = vec![1, 2, 3];

    for _val in v.iter() {
        // gives us a reference over v. Does not move the value
    }

    for _val in &v {
        // simiarl to v.iter()
    }

    for _val in v {
        // consudmes v and we own val
    }

    // now v has been dropped
}
