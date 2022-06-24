macro_rules! my_vec {
    () => {
        Vec::new()
    };

    // expr is any expression
    // we're returning a block here as this is the thing that the macro will be expanded to

    // here means repeat whatever's inside the () separated by atleast 1 ','
    ($($element: expr),+) => {{
        let mut vs = Vec::new();

        // same as above, repeat whatever's inside $() the same number of times as the
        // above expression repeated, but this time separated by a ';'
        // how many times to repeat = check the identifier in the above declaration
        $(vs.push($element);)*
        vs
    }};
}

pub fn tests() {
    // when calling a macro we can use either (), [] or {}

    // single
    let x: Vec<u32> = my_vec!(69);
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    // double
    let x: Vec<u32> = my_vec![69, 420];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 69);
    assert_eq!(x[1], 420);
}
