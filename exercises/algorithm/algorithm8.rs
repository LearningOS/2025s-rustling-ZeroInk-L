/*
    queue
    This question requires you to use queues to implement the functionality of the stac
*/
// I AM DONE HERE

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T> {
    //DONE
    q1: Queue<T>,
    q2: Queue<T>,
}
impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            //DONE
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }
    pub fn push(&mut self, elem: T) {
        //DONE
        self.q1.enqueue(elem)
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //DONE
        if self.is_empty() {
            return Err("Stack is empty");
        }
        while self.q1.size() > 1 {
            match self.q1.dequeue() {
                Ok(item) => self.q2.enqueue(item),
                Err(_) => return Err("Unexpected"),
            };
        }
        let result = match self.q1.dequeue() {
            Ok(item) => Ok(item),
            Err(_) => Err("Stack is empty"),
        };
        
        std::mem::swap(&mut self.q1, &mut self.q2);
        result
    }
    pub fn is_empty(&self) -> bool {
        //DONE
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
