// ring_buffer_deque

#![allow(dead_code)]
#![allow(unused)]

pub struct RingBufferDeque<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    head: usize,
    tail: usize,
    count: usize,
}

impl<T> RingBufferDeque<T> {
    pub fn new() -> Self {
        Self::with_capacity(4)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = if capacity == 0 { 4 } else { capacity };
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(None);
        }

        RingBufferDeque {
            buffer,
            capacity,
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn push_back(&mut self, item: T) -> Result<(), &'static str> {
        if self.is_full() {
            self.increase_capacity();
        }

        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;
        self.count += 1;
        Ok(())
    }

    pub fn push_front(&mut self, item: T) -> Result<(), &'static str> {
        if self.is_full() {
            self.increase_capacity();
        }

        self.head = (self.head + self.capacity - 1) % self.capacity;
        self.buffer[self.head] = Some(item);
        self.count += 1;
        Ok(())
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.count -= 1;

        item
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.tail = (self.tail + self.capacity - 1) % self.capacity;
        let item = self.buffer[self.tail].take();
        self.count -= 1;

        item
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
pub mod ring_buffer_deque_tests {
    use super::*;

    #[test]
    fn test_pop_both_ends() {
        let mut rb = RingBufferDeque::with_capacity(3);

        rb.push_back("A").unwrap();
        rb.push_back("B").unwrap();
        rb.push_back("C").unwrap(); 
        rb.push_front("D").unwrap(); // Buffer is now full [D, A, B, C]

        // Pop from front (head) and back (tail)
        assert_eq!(rb.pop_front(), Some("D")); // Removes oldest ("D")
        assert_eq!(rb.pop_front(), Some("A")); // Removes oldest ("A")
        assert_eq!(rb.pop_back(), Some("C"));  // Removes newest ("C")
        assert_eq!(rb.pop_front(), Some("B")); // Removes remaining ("B")
        
        assert!(rb.is_empty());
    }
}