use std::mem;

pub struct List<T> {
  head: Link<T>,
}

enum Link<T> {
  Empty,
  More(Box<Node<T>>),
}

struct Node<T> {
  elem: T,
  next: Link<T>,
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut cur_link = mem::replace(&mut self.head, Link::Empty);
    while let Link::More(mut boxed_node) = cur_link {
      cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
    }
  }
}

impl<T> List<T> {
  pub fn new() -> Self {
    List::<T> { head: Link::<T>::Empty }
  }

  pub fn push(&mut self, elem: T) {
    let new_node = Box::new(
      Node {
        elem: elem,
        next: mem::replace(&mut self.head, Link::Empty),
      }
    );
    self.head = Link::More(new_node);
  }

  pub fn pop(&mut self) -> Option<T> {
    match mem::replace(&mut self.head, Link::Empty) {
      Link::Empty => None,
      Link::More(node) => {
        self.head = node.next;
        Some(node.elem)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basics() {
    let mut list = List::new();

    // Check empty list behaves right
    assert_eq!(list.pop(), None);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
  }
}

