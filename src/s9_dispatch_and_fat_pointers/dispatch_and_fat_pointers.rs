use std::marker::PhantomData;

pub trait Hello {
    fn hello(&self);
}

impl Hello for &str {
    fn hello(&self) {
        println!("Hello, {}", self);
    }
}

impl Hello for String {
    fn hello(&self) {
        println!("Hello, {}", self);
    }
}

/// NOTE Static Dispatch
/// at compile time multiple of this methods will be created for all the Types that this method is
/// called with in the code
///
/// function below is equivalent to
///
/// ```
/// pub fn bar<H: Hello> (h: H) {
///  h.hello();
/// }
///
/// ```
///
/// NOTE this does not work if we want to make this function generic over things that implement
/// Hello trait. For example we cannot take a vector of things that implement Hello. We can take
/// Vec<&str> and Vec<String> but not Vec<&str | String>
pub fn bar(h: impl Hello) {
    h.hello();
}

// T: ?Sized = does not need to be sized
struct StdBox<T: ?Sized> {
    _t: PhantomData<T>,
}

/// &dyn Hello
/// stored in reference &
///  1. a pointer to the actual, concrete, implementing type
///  2. a pointer to a vtable for the referenced trait  
///
/// what is a vtable?
///
/// dyn Hello, vtable:
///
/// ```
///     struct HelloVtable {
///         hello: *mut Fn(*mut ())
///     }
/// ```
///
/// &str -> &dyn Hello
///
/// 1. Pointer to the &str (pointer to the str reference)
///
/// one such vtable for every type that gets converted to a Hello trait object
///
/// 2.
/// ```
///     HelloVtable {
///         // Key : member for each method of the string
///         // Value : value is the pointer to the implementation method of the concrete type
///         hello: &<str as Hello>::hello // Line 9
///     }  
/// ```
///
pub fn say_hello(s: &dyn Hello) {
    s.hello(); // what is generated as the machine code for this line?
               // the above line is akin to s.vtable.hello(s.pointer);
}

pub fn strlen<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().len()
}

pub fn tests() {
    "Wow".hello();
    bar("From bar");
}
