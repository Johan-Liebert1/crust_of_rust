use std::cell::UnsafeCell;

use super::cell::MyCell;

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct MyRefCell<T> {
    value: UnsafeCell<T>,
    state: MyCell<RefState>,
}

impl<T> MyRefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: MyCell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<MyRef<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                // SAFETY: no exclusive references have been given out, since state would be
                // exclusive
                Some(MyRef { refcell: self })
            }

            RefState::Shared(n) => unsafe {
                // SAFETY: no exclusive references have been given out, since state would be
                // exclusive
                self.state.set(RefState::Shared(n + 1));
                Some(MyRef { refcell: self })
            },

            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<MyRefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            // SAFETY: no other state has been given out since sate would be Shared or
            // Exclusive
            self.state.set(RefState::Exclusive);
            Some(MyRefMut { refcell: self })
        } else {
            None
        }
    }
}

pub struct MyRef<'refcell, T> {
    refcell: &'refcell MyRefCell<T>,
}

impl<T> std::ops::Deref for MyRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: a Ref is only created if no exclusive references have been given out.
        // once it is given out, state is set to shared so no exclusive references are given out
        // so dereferencing into a shared reference is fine
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Drop for MyRef<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => {
                unreachable!();
            }
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            }
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1));
            }
        }
    }
}

pub struct MyRefMut<'refcell, T> {
    refcell: &'refcell MyRefCell<T>,
}

impl<T> std::ops::Deref for MyRefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY:
        // see safety for DerefMut
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for MyRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: a RefMut is only created if no other references have been given out.
        // once it is given out, state is set to exclusive so no future references are given out
        // so we have an exclusive lease on the inner value so,
        // so mutably dereferencing into a shared reference is fine
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for MyRefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => {
                unreachable!();
            }
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared);
            }
        }
    }
}

pub fn tests() {
    println!("testing ref cell");
}
