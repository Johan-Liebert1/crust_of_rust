mod s1_lifetime_annotations;
mod s2_declarative_macros;

const TEST_CASE: i32 = 2;

fn main() {
    match TEST_CASE {
        1 => {
            s1_lifetime_annotations::lifetime_annotations::test_iterator();
            s1_lifetime_annotations::lifetime_annotations::test_until_character();
        }

        2 => s2_declarative_macros::declarative_macros::tests(),

        _ => println!("Invalid test case"),
    }
}
