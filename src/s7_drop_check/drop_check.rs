use std::marker::PhantomData;
use std::ptr::NonNull;

// if we have some THING that's generic over T, the the compiler assumes that dropping the THING
// will access the T
pub struct Boks<T> {
    p: NonNull<T>,      // *mut T is invariant, while NonNull is covariant
    _t: PhantomData<T>, // this tells the compiler that we do drop the T when Boks drops
}

// the #may_dangle is to let the compiler know that we do not access T in our Boks' drop
// but we need to tell the compiler that we will drop the T, which is what PhantomData achieves
unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        let a = Box::new(32);
        unsafe { Box::from_raw(self.p.as_mut()) };
    }
}

impl<T> Boks<T> {
    pub fn new(t: T) -> Self {
        Boks {
            // SAFETY: We know that Box never creates a null pointer
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(t))) },
            _t: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't been freed, since self is alive
        unsafe { &*self.p.as_ref() }
    }
}

impl<T> std::ops::DerefMut for Boks<T> {
    // we can refer to Self::Target here as anything that implemets DerefMut must also implement
    // Deref
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't been freed, since self is alive
        // Also, since we have &mut self, no other mutable reference has been given out to p
        unsafe { &mut *self.p.as_mut() }
    }
}

use std::fmt::Debug;
struct Oops<T: Debug>(T);

impl<T: Debug> Drop for Oops<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

pub fn boks_main() {
    let x = 42;
    let b = Boks::new(x);

    // the *b goes into the Deref trait
    println!("{:?}", *b);

    let mut y = 42;
    let b = Boks::new(&mut y);
    println!("{:?}", y);

    let mut z = 42;
    // this is not okay due to the PhantomData
    // if we remove the PhantomData then this code will compile
    let b = Boks::new(Oops(&mut z));
    // println!("{:?}", z);

    let z = String::new();
    let mut b = Box::new(&*z);
}
