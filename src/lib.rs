/// Stack Data Structure
/// It has a head, that points to the top of the stack
/// It has a size, updated on every push and pop
#[derive(Debug)]
pub struct Stack<T> {
    pub head: Option<Box<Tile<T>>>,
    pub size: usize,
}

impl<T> Stack<T> {
    /// Initialize a new stack with its head pointing to None and with zero size
    ///
    /// Example
    /// ```rust
    /// let new_stack = Stack::<i32>::new(); // Stack<i32> { head: None, size: 0 }
    /// ```
    pub fn new() -> Self {
        Stack {
            head: None,
            size: 0,
        }
    }
    /// Get a reference to data in the head of the Stack
    /// Returns None if the stack is empty
    ///
    /// Example
    /// ```rust
    /// let head_data: Option<&i32> = new_stack.peek();
    /// ```
    pub fn peek(&self) -> Option<&T> {
        match self.head {
            Some(ref v) => Some(&v.as_ref().value),
            None => None,
        }
    }
    /// Pops the top off the stack and returns the data it contains
    /// Returns None if the stack is empty
    ///
    /// Reduces the size of stack by 1 unit
    ///
    /// Example
    /// ```rust
    /// let top_data: Option<i32> = new_stack.pop();
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(v) => {
                self.head = v.next;
                self.size -= 1;
                Some(v.value)
            }
            None => None,
        }
    }
    /// Pushes a new tile with the desired data onto the stack
    ///
    /// Increases the size of stack by 1 unit
    ///
    /// Example
    /// ```rust
    /// new_stack.push(5);
    /// ```
    pub fn push(&mut self, data: T) {
        self.size += 1;
        match self.head.take() {
            Some(v) => {
                self.head = Some(Box::new(Tile {
                    value: data,
                    next: Some(v),
                }));
            }
            None => {
                self.head = Some(Box::new(Tile {
                    value: data,
                    next: None,
                }));
            }
        }
    }
}

impl<T: PartialEq> Stack<T> {
    /// Searches for data in the whole stack [O(n) operation]
    /// Returns None is stack is empty
    ///
    /// Example
    /// ```rust
    /// let result: Option<usize> = new_stack.search(3);
    /// ```
    pub fn search(&self, data: T) -> Option<usize> {
        let mut ptr = self.head.as_ref();
        let mut pos = self.size;
        while let Some(ref p) = ptr {
            if p.as_ref().value == data {
                return Some(pos);
            }
            ptr = p.next.as_ref();
            pos -= 1;
        }
        return None;
    }
}

#[derive(Debug)]
pub struct Tile<T> {
    value: T,
    next: Option<Box<Tile<T>>>,
}

impl<T> Tile<T> {
    pub fn new(value: T) -> Self {
        Tile { value, next: None }
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;
    #[test]
    fn basics() {
        let mut stack = Stack::<u8>::new();
        stack.push(3);
        stack.push(6);
        stack.push(9);
        assert_eq!(3, stack.size);
        assert_eq!(Some(&9), stack.peek());
        assert_eq!(Some(9), stack.pop());
        assert_eq!(Some(2), stack.search(6));
        assert_eq!(2, stack.size);
        assert_eq!(Some(6), stack.pop());
        assert_eq!(Some(&3), stack.peek());
        assert_eq!(3, stack.head.unwrap().as_ref().value);
    }
}
