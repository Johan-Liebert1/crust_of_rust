// Reference counted pointer
use super::cell::MyCell;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    refcount: MyCell<usize>,
}

pub struct MyRc<T> {
    // T is stored on the heap. It needs to be on the heap as multiple functions can be pointing to
    // T so T cannot be on the stack frame of any particular function
    inner: NonNull<RcInner<T>>,
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

            MyRc { inner: self.inner }
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
