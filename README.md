# stack-rs

Implementation of the Stack Data-Structure in [rust-lang](https://rust-lang.org)

## Usage

```rust
use stack::Stack;

fn main() {
  let mut my_stack = Stack::<i32>::new();
  my_stack.push(2);
  my_stack.push(4);
  my_stack.push(7);
  assert_eq!(3, my_stack.size);
  assert_eq!(Some(&7), my_stack.peek());
  assert_eq!(Some(7), my_stack.pop());
  assert_eq!(Some(1), my_stack.search(2));
}
```