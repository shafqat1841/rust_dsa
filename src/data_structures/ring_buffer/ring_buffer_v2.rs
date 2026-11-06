// ring_buffer_v2

#![allow(dead_code)]
#![allow(unused)]

pub struct RingBufferV2<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    head: usize,
    tail: usize,
    count: usize,
}

impl<T> RingBufferV2<T> {
    pub fn new() -> Self {
        Self::with_capacity(4)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        
        let capacity = if capacity == 0 { 4 } else { capacity };
        
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(None);
        }

        RingBufferV2 {
            buffer,
            capacity,
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn enqueue(&mut self, item: T) -> Result<(), &'static str> {
        if self.is_full() {
            // return Err("Buffer is full");
            self.increase_capacity()
        }

        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;
        self.count += 1;
        Ok(())
    }

    pub fn increase_capacity(&mut self) {
          let new_capacity = self.capacity * 2;
        let mut new_buffer = Vec::with_capacity(new_capacity);
        
        for _ in 0..new_capacity {
            new_buffer.push(None);
        }

        let mut current_idx = self.head;
        for new_tail in 0..self.count {
            new_buffer[new_tail] = self.buffer[current_idx].take();
            current_idx = (current_idx + 1) % self.capacity;
        }

        self.head = 0;
        self.tail = self.count;
        self.buffer = new_buffer;
        self.capacity = new_capacity;
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
pub mod ring_buffer_v2_tests {
    use super::*;
    #[test]
    fn create_data_structure() {
        let mut rb = RingBufferV2::with_capacity(3);

        rb.enqueue("A").unwrap();
        rb.enqueue("B").unwrap();
        rb.enqueue("C").unwrap();
        rb.enqueue("D").unwrap();
        rb.enqueue("E").unwrap();

        // assert!(rb.enqueue("D").is_err());

        assert_eq!(rb.dequeue(),Some("A"));
        assert_eq!(rb.dequeue(),Some("B"));
        
        // rb.enqueue("D").unwrap(); 
        
        assert_eq!(rb.dequeue(),Some("C"));
        assert_eq!(rb.dequeue(),Some("D"));

        assert_eq!(rb.capacity, 6);
    }
}
