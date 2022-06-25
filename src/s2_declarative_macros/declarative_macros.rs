// have to macro_export else $crate does not work
#[macro_export]
/// my good ass doc
macro_rules! my_vec {
    () => {
        Vec::new()
    };

    // expr is any expression
    // we're returning a block here as this is the thing that the macro will be expanded to

    // here means repeat whatever's inside the () separated by atleast 1 ','
    ($($element: expr),+ $(,)? /* 0 or 1 trailing comma */) => {{
        let mut vs = Vec::with_capacity($crate::my_vec![@COUNT; $($element),*]);

        // same as above, repeat whatever's inside $() the same number of times as the
        // above expression repeated, but this time separated by a ';'
        // how many times to repeat = check the identifier in the above declaration
        $(vs.push($element);)+
        // $(vs.push($element);)+ <-- adding this statement is totally fine. It'll just repeat the
        // repitition twice
        vs
    }};

    // for things like my_vec![420; 69];
    ($element: expr; $count: expr) => {{
        let count = $count;
        let mut vs = Vec::with_capacity(count);
        // to not do multiple pointer increments
        vs.extend(std::iter::repeat($element).take(count));

        // this will increment the pointer on every push (optimized above)
        //let x = $element;
        // for _ in 0..count {
        //     vs.push(x.clone());
        // }

        vs
    }};

    (@COUNT; $($element:expr),*) => {
        [$($crate::my_vec![@SUBST; $element]),*].len()
    };

    // this one won't use the element. It takes the argument but won't use it
    (@SUBST; $_element:expr) => { () }
}

pub fn tests() {
    println!("Testing declarative_macros");
    // when calling a macro we can use either (), [] or {}

    // single
    let x: Vec<u32> = my_vec!(69);
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);

    // double
    let x: Vec<u32> = my_vec![69, 420, 666, 13];
    assert_eq!(x.len(), 4);
    assert_eq!(x[0], 69);
    assert_eq!(x[1], 420);
    assert_eq!(x[2], 666);
    assert_eq!(x[3], 13);

    // trailing comma
    let _: Vec<&'static str> = my_vec![
        "hewrkjewlkrjwelrjwelrkjwer",
        "hlwerjweklrjwkrljewlrewkjrew",
        "klwerlwkjerlwkjwlrjwelrj",
    ];

    // semi colon
    let x: Vec<i32> = my_vec![69; 5];
    assert_eq!(x, vec![69, 69, 69, 69, 69]);
}

/*
pub trait MaxValue {
    fn max_value() -> Self;
}

//  get maximum for every numberic type
#[macro_export]
macro_rules! max_impl {
    ($t: ty) => {
        impl $crate::s2_declarative_macros::MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

max_impl!(u32);
max_impl!(i32);
max_impl!(i64);
max_impl!(u64);*/
