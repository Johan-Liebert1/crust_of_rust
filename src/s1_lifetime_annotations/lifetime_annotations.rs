#[derive(Debug, PartialEq)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

// lifetime of StrSplit has to be the same as the strings as str split contains both strings
// so if we ever destroy haystack or the delimiter then we also destroy StrSplit
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

// here the final return lifetime will be of x as we are givinf y some generic lifetime
// so the only unambiguous lifetime here is that of x
// fn foo(x: &str, y: &'_ str) -> &'_ str {"hello"}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // we use 'ref mut' here to only get a mutable reference to self.remainder and not move it
        // out
        // we cannot write Some(&mut remainder), then '&mut remainder' would be the pattern being
        // matched when we need to match remainer
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delimiter) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delimiter];
                *remainder = &remainder[(next_delimiter + self.delimiter.len())..];
                Some(until_delimiter)
            } else {
                // impl<T> Option<T> { fn take(&mut self) -> Option<T> }
                // if T is None then it returns None, else it consumes from the Some and sets the
                // Option to none
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

fn until_char(s: &str, c: char) -> &str {}

pub fn test_iterator() {
    println!("testing iterator");
    let haystack = "a b c d e";
    let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

pub fn test_until_character() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}
