use std::cell::UnsafeCell;

// Cell type allows us to modify a value through a shared reference (through API) and not directly
// with raw pointers
// Cells are usually used when we have number of flags that multiple things point to
// Cells can be used for anything, but usually it's only used with types that are cheap to copy
pub struct MyCell<T> {
    value: UnsafeCell<T>,
}

// negative trait bounds are not yet fully implemented; use marker types for now
// this is implied by UnsafeCell
// impl<T> !Sync for MyCell<T> {}

impl<T> MyCell<T> {
    pub fn new(value: T) -> Self {
        MyCell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value };
    }

    /// get always returns a copy and never a reference
    /// Reasoning
    ///
    /// ```
    /// let x = Cell::new(vec![42]);
    /// let first = &x.get()[0];
    ///
    /// // emptying the vector, now the vector is wiped out in memory and so is
    /// x.set(vec![]);
    ///
    /// println!("{}", first); // wrong! as the reference to first now just points to some random
    /// memory
    /// ```
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

pub fn tests() {}
