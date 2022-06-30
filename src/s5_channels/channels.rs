#![allow(unused_variables, dead_code, unused_mut)]

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex}; // VecDeque = kinda like a ring buffer
                                      // Arc = Atomically reference counted type

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            // to tell Rust to clone the Arc, i.e. the reference counter instead of the inner thing
            // that Arc holds which is the VecDeque
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);
        drop(queue);
        // notify any waiting receiver when it sends
        self.inner.available.notify_one();
    }
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Receiver<T> {
    pub fn receive(&mut self) -> T {
        let mut queue = self.inner.queue.lock().unwrap();

        loop {
            match queue.pop_front() {
                Some(t) => return t,
                None => {
                    // this loop won't consume CPU cycles as it is put to sleep by the OS
                    // wait gives up the lock
                    queue = self.inner.available.wait(queue).unwrap();
                }
            }
        }
    }
}

struct Inner<T> {
    // things in the channel
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::default(),
        available: Condvar::new(),
    };

    let inner = Arc::new(inner);

    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
        },
    )
}

pub fn tests() {
    println!("Testing channels");

    // ping pong
    let (mut tx, mut rx) = channel();
    tx.send(42);
    assert_eq!(rx.receive(), 42);
}
