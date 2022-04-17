use std::collections::VecDeque;
use std::os::unix::thread;
use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {
    data: Arc<Data<T>>,
}
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut thread = self.data.thread.lock().unwrap();
        thread.senders_amount+=1;
        drop(thread);

        Sender { 
            data: Arc::clone(&self.data) 
        }
    }
}
impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut thread = self.data.thread.lock().unwrap();
        thread.queue.push_back(t);
        drop(thread);
        self.data.is_available.notify_all(); //Condvar "wakeup" to receiver
    }
}
impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut thread = self.data.thread.lock().unwrap();
        thread.senders_amount -= 1;
        let is_last = thread.senders_amount;
        drop(thread);
        if is_last == 0 {
            self.data.is_available.notify_all();
        }
    }
}
pub struct Receiver<T> {
    data: Arc<Data<T>>,
}
impl<T> Receiver<T> {
    pub fn receive(&mut self) -> Option<T> {
        let mut thread = self.data.thread.lock().unwrap();
        loop{
            match thread.queue.pop_front() {

                Some (t) => return Some(t),
                None if thread.senders_amount == 0 => return None,
                None => {
                    thread = self.data.is_available.wait(thread).unwrap()
                }
    
            }

        }
    }
}

struct Data<T> {
    thread : Mutex<LockedQueue<T>>,
    is_available :Condvar
}
struct LockedQueue<T> {
    queue : VecDeque<T>,
    senders_amount : usize
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let thread = LockedQueue {
        queue: VecDeque::default(),
        senders_amount: 1,
    };
    let data = Data {
        thread: Mutex::new(thread),
        is_available: Condvar::new(),
    };
    let arc_data = Arc::new(data);
    (
        Sender {
            data: arc_data.clone()
        },
        Receiver {
            data: arc_data.clone()
        },
    )
}

#[test]
fn test_send_receive() {
    let (mut tx,mut rx) = channel();
    tx.send(42);
    assert_eq!(rx.receive(), Some(42))
}
#[test]
fn test_drop_sender() {
    let (tx,mut rx) = channel::<()>();
    drop(tx);
    assert_eq!(rx.receive(), None);
}
fn test_drop_receiver() {
    let (mut tx, rx) = channel();
    drop(rx);
    tx.send(42);
}