mod s1_lifetime_annotations;
mod s2_declarative_macros;
mod s3_iterators;
mod s4_smart_pointers_interior_mutability;
mod s5_channels;

const TEST_CASE: i32 = 5;

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

        _ => println!("Invalid test case"),
    }
}
