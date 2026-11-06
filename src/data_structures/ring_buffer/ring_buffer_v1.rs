// ring_buffer_v1

#![allow(dead_code)]
#![allow(unused)]

pub struct RingBufferV1<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    head: usize,
    tail: usize,
    count: usize,
}

impl<T> RingBufferV1<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(None);
        }

        RingBufferV1 {
            buffer,
            capacity,
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn enqueue(&mut self, item: T) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Buffer is full");
        }

        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;
        self.count += 1;
        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.count -= 1;

        item
    }

    pub fn is_full(&self) -> bool {
        self.count == self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn len(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
pub mod ring_buffer_v1_tests {
    use super::*;
    #[test]
    fn create_data_structure() {
        let mut rb = RingBufferV1::new(3);

        rb.enqueue("A").unwrap();
        rb.enqueue("B").unwrap();
        rb.enqueue("C").unwrap();

        assert!(rb.enqueue("D").is_err());

        assert_eq!(rb.dequeue(),Some("A"));
        assert_eq!(rb.dequeue(),Some("B"));
        
        rb.enqueue("D").unwrap(); 
        
        assert_eq!(rb.dequeue(),Some("C"));
        assert_eq!(rb.dequeue(),Some("D"));
    }
}
