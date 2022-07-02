#![allow(unused_variables, dead_code, unused_mut)]

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex}; // VecDeque = kinda like a ring buffer
                                      // Arc = Atomically reference counted type

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        // keep track of how many senders there are. We do this to not keep a receiver waiting when
        // all the senders have been dropped
        inner.senders += 1;
        drop(inner);

        Self {
            // to tell Rust to clone the Arc, i.e. the reference counter instead of the shared thing
            // that Arc holds which is the VecDeque
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);

        if was_last {
            // last sender drop. Wake the receiver
            self.shared.available.notify_one();
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);
        drop(inner);

        // notify any waiting receiver when it sends
        self.shared.available.notify_one();
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    pub fn receive(&mut self) -> Option<T> {
        let mut inner = self.shared.inner.lock().unwrap();

        loop {
            match inner.queue.pop_front() {
                Some(t) => return Some(t),

                // if the sender count is 0, then just
                // return as the channel is empty forever
                None if inner.senders == 0 => return None,
                None => {
                    // this loop won't consume CPU cycles as it is put to sleep by the OS
                    // wait gives up the lock
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

struct Inner<T> {
    // things in the channel
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    // things in the channel
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        senders: 1,
    };

    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::new(),
    };

    let shared = Arc::new(shared);

    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
        },
    )
}

pub fn tests() {
    println!("Testing channels");

    // ping pong
    let (mut tx, mut rx) = channel();
    tx.send(42);
    assert_eq!(rx.receive(), Some(42));

    // closed tx
    let (mut tx, mut rx) = channel::<()>();
    drop(tx);
    // let _ = rx.receive(); // what do we even get here? this will just hang forever
    assert_eq!(rx.receive(), None);

    // closed rx
    let (mut tx, mut rx) = channel::<()>();
    drop(rx);
    tx.send(42); // we should tell that channel has been closed as there are no receivers
}
