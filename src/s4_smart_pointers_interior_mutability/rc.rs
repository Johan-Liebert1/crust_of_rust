#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_variables)]
// Reference counted pointer
use super::cell::MyCell;
use std::marker::PhantomData;
use std::ptr::NonNull;

// ============================ THE DROP CHECK =========================================
struct Foo<'a, T: Default> {
    v: &'a mut T,
}

impl<T: Default> Drop for Foo<'_, T> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        std::mem::replace(self.v, T::default());
    }
}

fn bad_things_happen() {
    // t is dropped before foo as Rust maintains a stack of variables to be dropped
    let foo: Foo<String>;
    let mut t;
    t = String::from("hello");
    // foo = MyRc::new(Foo { v: &mut t });
} // here foo is dropped and dropping a struct is counted as dropping every single value of that
  // struct. So Rust will call the drop implementation of Foo, but in Foo we're accessing the dropped
  // string t, and the code won't compile
  //
  // But, with a MyRef, Rust does not know about the "T" that MyRef contains and if we surround the
  // declaration of Foo by MyRef, i.e. MyRc::new(Foo{ v: &mut t }), it will compile the code as the
  // compiler does not know about the t inside of Foo inside of MyRc

// ============================ THE DROP CHECK =========================================

struct RcInner<T> {
    value: T,
    refcount: MyCell<usize>,
}

pub struct MyRc<T> {
    // T is stored on the heap. It needs to be on the heap as multiple functions can be pointing to
    // T so T cannot be on the stack frame of any particular function
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>, // treat as if MyRc contains T
}

impl<T> MyRc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: MyCell::new(1),
        });

        unsafe {
            MyRc {
                // SAFETY: Box does not give us a null pointer
                inner: NonNull::new_unchecked(Box::into_raw(inner)),
                _marker: PhantomData,
            }
        }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        // we're only increasing the reference count here and not cloning the actual data

        unsafe {
            let inner = self.inner.as_ref();
            let c = inner.refcount.get();
            inner.refcount.set(c + 1);

            MyRc {
                inner: self.inner,
                _marker: PhantomData,
            }
        }
    }
}

impl<T> std::ops::Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            let inner = self.inner.as_ref();
            let c = inner.refcount.get();

            if c == 1 {
                // SAFETY: We're the __only__ Rc left, and we are being dropped.
                // thus, after us, there will be no Rc's and no references to T

                drop(inner);
                // _ = dropping the pointer
                // Box::from_raw gives us a raw pointer
                let _ = Box::from_raw(self.inner.as_ptr());
            } else {
                // SAFETY: There are other Rc's so don't drop the Box because other things are
                // gonna need it
                inner.refcount.set(c - 1);
            }
        }
    }
}

pub fn tests() {}
