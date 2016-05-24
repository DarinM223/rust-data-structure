/// Queue implementation with two vectors
/// enqueue values by pushing onto stack1
/// dequeue by popping from stack2 and moving the reverse
/// of stack1 onto stack2 whenever stack2 becomes empty
pub struct Queue<T> {
    stack1: Vec<T>,
    stack2: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            stack1: vec![],
            stack2: vec![],
        }
    }

    pub fn enqueue(&mut self, data: T) {
        self.stack1.push(data);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.stack2.len() == 0 {
            if self.stack1.len() == 0 {
                return None;
            }

            // Reverse stack1 onto stack2
            while self.stack1.len() > 0 {
                self.stack2.push(self.stack1.pop().unwrap());
            }
        }
        return self.stack2.pop();
    }
}

#[test]
fn test_queue() {
    let mut queue: Queue<i32> = Queue::new();
    queue.enqueue(2);
    queue.enqueue(3);
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.dequeue(), Some(3));
    queue.enqueue(5);
    queue.enqueue(6);
    assert_eq!(queue.dequeue(), Some(5));
    assert_eq!(queue.dequeue(), Some(6));
}
