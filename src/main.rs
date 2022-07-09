#![allow(
    unused_variables,
    unused_mut,
    dead_code,
    unused_assignments,
    unused_doc_comments
)]
#![feature(dropck_eyepatch)]

mod s1_lifetime_annotations;
mod s2_declarative_macros;
mod s3_iterators;
mod s4_smart_pointers_interior_mutability;
mod s5_channels;
mod s6_sorting_algos;
mod s7_drop_check;
mod s8_atomics_and_memory;
mod s9_dispatch_and_fat_pointers;

const TEST_CASE: i32 = 9;

fn main() {
    match TEST_CASE {
        1 => {
            s1_lifetime_annotations::lifetime_annotations::test_iterator();
            s1_lifetime_annotations::lifetime_annotations::test_until_character();
        }

        2 => s2_declarative_macros::declarative_macros::tests(),

        3 => s3_iterators::iterators::tests(),

        4 => {
            s4_smart_pointers_interior_mutability::cell::tests();
            s4_smart_pointers_interior_mutability::refcell::tests();
            s4_smart_pointers_interior_mutability::rc::tests();
        }

        5 => s5_channels::channels::tests(),

        6 => {
            s6_sorting_algos::orst::tests();
            // s6_sorting_algos::benches::benchmarks::run_benchmarks();
        }

        7 => s7_drop_check::drop_check::boks_main(),

        8 => s8_atomics_and_memory::atomics_and_memory::tests(),

        9 => s9_dispatch_and_fat_pointers::dispatch_and_fat_pointers::tests(),

        _ => println!("Invalid test case"),
    }
}
