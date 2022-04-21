use std::collections::VecDeque;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};

#[derive(Debug, Clone)]
pub struct ChannelData<T> {
    pub channel_name : char,
    pub tx : Sender<T>,
    pub rx : Receiver<T>
}

impl<T> ChannelData<T> {
    pub fn new(name: char) -> ChannelData<T>{
        let (tx,rx) = channel::<T>();
        ChannelData {
            channel_name: name,
            tx,
            rx
        } 
    }
    pub fn default() -> Self {
        let (tx,rx) = channel::<T>();
        ChannelData {
            channel_name: 'c',
            tx,
            rx
        } 
    }
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
#[derive(Debug)]
pub struct Sender<T> {
    data: Arc<Data<T>>,
}
// using Mutex lock we are increase senders and drop inner thread. Thread-safe data clone to Sender struct
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
//  unlocked Arc(data) pushes to back of queue
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
            //Using CondVar to notify Receiver to "wake up"
            //You can add one more CondVar for Receiver implementation to notify Sender
            //If Sender push data faster than Receiver takes it, CondVar can says "wake up" to Sender
            self.data.is_available.notify_all();
        }
    }
}
#[derive(Debug, Clone)]
pub struct Receiver<T> {
    data: Arc<Data<T>>,
}
// unlocked Arc(data) taked from front of queue
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
#[derive(Debug)]
struct Data<T> {
    thread : Mutex<LockedQueue<T>>,
    is_available :Condvar
}
#[derive(Debug)]
struct LockedQueue<T> {
    //you can have a conflict if you push/pop data to Vector. So, we are using double-ended queue
    queue : VecDeque<T>,
    senders_amount : usize
}


#[test]
fn test_send_receive() {
    let (mut tx,mut rx) = channel();
    tx.send(42);
    assert_eq!(rx.receive(), Some(42))
}
#[test]
fn test_channel_data() {
    let mut channel = ChannelData::new('c');
    channel.tx.send(3);
    assert_eq!(channel.rx.receive(), Some(3))
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