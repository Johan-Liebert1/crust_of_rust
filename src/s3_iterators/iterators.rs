#![allow(unused_variables)]
#![allow(dead_code)]

pub fn flatten<I>(iter: I) -> Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter)
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
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
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                // we have exhausted the inner iterator as next didn't give back any item
                self.inner = None;
            }

            let next_inner_iter = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_iter);
        }
    }
}

pub fn tests() {
    println!("Testing iterators");
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
