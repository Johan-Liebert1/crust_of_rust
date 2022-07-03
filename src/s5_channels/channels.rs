#![allow(unused_variables, dead_code, unused_mut)]

// Channel Flavours:
//
//  - Synchronous channels: Channel where send can block. Limited capacity
//
//  - Asynchronous channels: Channel where send cannot block. Unbounded
//
//  - Randezvous channels: Synchronous with capacity = 0. Won't let you send things, used for thread
//  synchronization
//
//  - Oneshot channels: Channles you only send at once. Any capacity, in practice only 1 call to
//  send()

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex}; // VecDeque = kinda like a ring buffer
                                      // Arc = Atomically reference counted type

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    // since we have only one receiver, we don't need to take the lock on every receive
    buffer: VecDeque<T>,
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
        // it might be that the VecDeque's size increases which means that send would take longer
        // and not necessarily that send will block
        inner.queue.push_back(t);
        drop(inner); // drop the lock

        // notify any waiting receiver when it sends
        self.shared.available.notify_one();
    }
}

impl<T> Receiver<T> {
    pub fn receive(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            // here we have something in the receiver's buffer that we copied from the VecDeque
            // during the last receive. Here we don't need to take the lock as we alrady have
            // soemthing that's not yet "received"
            return Some(t);
        }

        let mut inner = self.shared.inner.lock().unwrap();

        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    if !inner.queue.is_empty() {
                        // copy the contents of the queue into receiver's buffer so that the next
                        // size(self.buffer) receives won't take the lock
                        // we swap the empty buffer with the queue that has items to receive
                        //
                        // NOTE: The buffer will be empty here as we will always pop from the
                        // buffer if it's not empty
                        std::mem::swap(&mut self.buffer, &mut inner.queue);
                    }

                    return Some(t);
                }

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

// impementing Iterator for receiver which would be similar to Go's for item := channel {}
impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.receive()
    }
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
            buffer: VecDeque::default(),
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
    let (mut tx, mut rx) = channel();
    drop(rx);
    tx.send(42); // we should tell that channel has been closed as there are no receivers
}
