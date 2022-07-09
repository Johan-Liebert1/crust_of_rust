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

    // NOT_TOO_GOOD: Checkout test too_relaxed for more info
    pub fn with_okayish_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            // MESI protocol

            // as compare_exchange is quite expensive due to the CPU needing an exclusive reference
            // to the memory location, we will just read the value until it changes to our desired
            // value. Reading only needs shared access to the memory location
            while self.locked.load(Ordering::Relaxed) == LOCKED {}
        }
        // maybe another thread runs here...

        // SAFETY: We hold the lock therefore we can create a mutable reference
        let ret = f(unsafe { &mut *self.v.get() });

        self.locked.store(UNLOCKED, Ordering::Relaxed);

        ret
    }

    pub fn with_good_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // MESI protocol

            // as compare_exchange is quite expensive due to the CPU needing an exclusive reference
            // to the memory location, we will just read the value until it changes to our desired
            // value. Reading only needs shared access to the memory location
            while self.locked.load(Ordering::Relaxed) == LOCKED {}
        }
        // maybe another thread runs here...

        // SAFETY: We hold the lock therefore we can create a mutable reference
        let ret = f(unsafe { &mut *self.v.get() });

        // Ordering::Release makes sure to not reorder the statement while executing
        self.locked.store(UNLOCKED, Ordering::Release);

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

        // assert_eq!(l.with_bad_lock(|v| *v), 100 * 1000);
    }

    #[test]
    fn too_relaxed() {
        use std::sync::atomic::AtomicUsize;

        let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
        let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

        let t1 = spawn(move || {
            let r1 = y.load(Ordering::Relaxed);
            x.store(r1, Ordering::Relaxed);
            r1
        });
        let t2 = spawn(move || {
            let r2 = x.load(Ordering::Relaxed);

            // this line might execute before the previous line due to CPU / compiler optimizing things
            // which is allowed as the following line does not depend on anything that the previous
            // line does as the following line does not use r2 and neither x
            //
            // NOTE: Odering::Relaxed allows this to happen
            y.store(42, Ordering::Relaxed);
            r2
        });

        let r1 = t1.join().unwrap();
        let r2 = t2.join().unwrap();

        // it is possible for the following to happen
        // r1 == r2 == 42
    }

    #[test]
    fn sync_test() {
        use std::sync::atomic::AtomicUsize;

        let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
        let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
        let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

        let _tx = spawn(move || {
            x.store(true, Ordering::Release);
        });

        let _ty = spawn(move || {
            y.store(true, Ordering::Release);
        });

        let t1 = spawn(move || {
            while !x.load(Ordering::Acquire) {}

            if y.load(Ordering::Acquire) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });

        let t2 = spawn(move || {
            while !y.load(Ordering::Acquire) {}

            if x.load(Ordering::Acquire) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();

        let z = z.load(Ordering::SeqCst);

        eprintln!("z = {}", z);

        // What are the possible values for z?
        //
        // - Is 2 possible?
        // Yes if they run in this order, tx, ty, t1, t2
        //
        // - Is 1 possible?
        // Yes: tx, t1, ty, t2
        //
        // - Is 0 possible?
        //
        // Restrictions: we know that t1 must run after tx as t1 has a waiting loop on x
        // we know that t2 must run after ty as t2 has a waiting loop on y
        //
        // ty t2 tx t1 -> t1 will increment z
        //
        // ty tx (ty <ty can also go here>) t2 t1 -> t1 & t2 will increment z
        //
        // tx t1 ty t2 -> t2 will increment z
        //
        // Seems impossible ...
        //
        // Modification Order for x = false (then) true
        // Modification Order for y = false (then) true
        //
        // t1 is allowed to see either of the values for y, i.e. it's allowed to see true and false
        // this is because in an Acquire / Release setup, if you observe a value from Acquire,
        // you will see all the operations before the corresponding release store
        //
        // corresponding release for tx's Acquire is in tx
        //
        // t1 is bound to get true from the Modification order of x, but it can see either true /
        // false from the Modification order of y. This is because there are no Modifications of y
        // before the corresponding Release (there could be if t2 runs but otherwise there are none)
        // so we can get true if t2 runs else false
        //
        // Same is true for t2, t2 is see true for y but is
        // allowed to see either true / false from the Modification order of x
        //
        // NOTE: If we make all orderings SeqCst then 0 is no longer possible as SeqCst will make
        // sure that every thread will see everything happen in the same order
    }
}

pub fn tests() {
    println!("Runing atomic tests");
}
