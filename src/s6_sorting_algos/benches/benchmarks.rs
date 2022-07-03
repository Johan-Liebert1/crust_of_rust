use super::super::bubble::BubbleSort;
use super::super::insertion::InsertionSort;
use super::super::orst::Sorter;
use super::super::quick::QuickSort;

// use bytify::bytify;

use rand::prelude::*;

use std::cell::Cell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Clone)]
pub struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.t == other.t
    }
}
impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}
impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmps.set(self.cmps.get() + 1);
        self.t.cmp(&other.t)
    }
}

// impl<T> Bytify for SortEvaluator<T>
// where
//     T: Bytify,
// {
//     fn bytify(&self, level: usize) -> Option<usize> {
//         return self.t.bytify(level);
//     }
// }

pub fn run_benchmarks() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));

    println!("algorithm n comparisons time");
    for &n in &[0, 1, 10, 100, 1000, 10000, 50000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: Rc::clone(&counter),
            });
        }

        for _ in 0..10 {
            values.shuffle(&mut rand);

            let took = bench(BubbleSort, &values, &counter);
            println!("{} {} {} {}", "bubble", n, took.0, took.1);
            let took = bench(InsertionSort, &values, &counter);
            println!("{} {} {} {}", "insertion-smart", n, took.0, took.1);
            let took = bench(QuickSort, &values, &counter);
            println!("{} {} {} {}", "quick", n, took.0, took.1);
        }
    }
}

pub fn bench<T: Ord + Clone, S: Sorter<SortEvaluator<T>>>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> (usize, f64) {
    let mut values: Vec<_> = values.to_vec();
    counter.set(0);

    let time = std::time::Instant::now();
    sorter.sort(&mut values);

    let took = time.elapsed();
    let count = counter.get();

    // assert!(values.is_sorted());
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }

    (count, took.as_secs_f64())
}
