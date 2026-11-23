use core::sync::atomic::*;
use core::cell::*;

const BUF_LEN: usize = 256;

struct ByteBuffer {
    data: UnsafeCell<[u8; BUF_LEN]>
}

impl ByteBuffer {
    pub const fn new() -> Self {
        Self { data: UnsafeCell::new([0; BUF_LEN]) }
    }
    pub unsafe fn get(&self, index: usize) -> u8 {
        debug_assert!(index < BUF_LEN);
        *self.data.get().cast::<u8>().add(index)
    }
    pub unsafe fn set(&self, index: usize, value: u8) {
        debug_assert!(index < BUF_LEN);
        *self.data.get().cast::<u8>().add(index) = value;
    }
}

pub struct RingBuffer {
    buffer: ByteBuffer,
    head: AtomicUsize,
    tail: AtomicUsize,
}

impl RingBuffer {

    pub const fn new() -> Self { // ensure staticness of RingBuffer
        Self { buffer: ByteBuffer::new(), head: AtomicUsize::new(0), tail: AtomicUsize::new(0)}
    }

    pub fn push(&self, b: u8) -> bool {
        let _head: usize = self.head.load(Ordering::Relaxed);
        let _tail: usize = self.tail.load(Ordering::Acquire);
        if _head - _tail == BUF_LEN { // full
            return false;
        }
        unsafe {
            self.buffer.set(_head % BUF_LEN, b);
        }
        self.head.fetch_add(1, Ordering::Release);
        return true;
    }

    pub fn pop(&self) -> Option<u8> {
        let _head: usize = self.head.load(Ordering::Acquire);
        let _tail: usize = self.tail.load(Ordering::Relaxed);
        if _head == _tail { // empty
            return None;
        }
        let mut val: u8 = 0;
        unsafe {
            val = self.buffer.get(_tail % BUF_LEN);
        }
        self.tail.fetch_add(1, Ordering::Release);
        return Some(val);
    }

}

unsafe impl Sync for RingBuffer {}
