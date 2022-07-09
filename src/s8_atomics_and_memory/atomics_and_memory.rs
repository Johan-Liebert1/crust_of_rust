use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct MyMutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for MyMutex<T> where T: Send {}

impl<T> MyMutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }

    // takes a closure function
    pub fn with_bad_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self.locked.load(Ordering::Relaxed) != UNLOCKED {}
        // maybe another thread runs here...

        self.locked.store(LOCKED, Ordering::Relaxed);
        // SAFETY: We hold the lock therefore we can create a mutable reference
        let ret = f(unsafe { &mut *self.v.get() });

        self.locked.store(UNLOCKED, Ordering::Relaxed);

        ret
    }

    // takes a closure function
    pub fn with_good_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .locked
            .compare_exchange(UNLOCKED, LOCKED, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            // MESI protocol
        }
        // maybe another thread runs here...

        // SAFETY: We hold the lock therefore we can create a mutable reference
        let ret = f(unsafe { &mut *self.v.get() });

        self.locked.store(UNLOCKED, Ordering::Relaxed);

        ret
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::thread::spawn;

    #[test]
    fn test_bad_mutex() {
        let l: &'static _ = Box::leak(Box::new(MyMutex::new(0)));

        let handles: Vec<_> = (0..100)
            .map(|_| {
                spawn(move || {
                    for _ in 0..1000 {
                        l.with_bad_lock(|v| {
                            *v += 1;
                        })
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(l.with_bad_lock(|v| *v), 100 * 1000);
    }
}

pub fn tests() {
    println!("Runing atomic tests");
}
