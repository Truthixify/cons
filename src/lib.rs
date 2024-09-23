use List::{Cons, Nil};

#[derive(Debug, PartialEq, Clone)]
pub enum List {
  Cons(i32, Box<List>),
  Nil,
}

impl List {
  pub fn new() -> Self {
    Nil
  }

  pub fn is_empty(&self) -> bool {
    match self {
        Nil => true,
        _ => false
    }
  }

  pub fn prepend(&mut self, element: i32) {
    *self = Cons(element, Box::new(self.clone()));
  }

  pub fn len(&self) -> usize {
    match self {
        Nil => 0,
        Cons(_, next) => 1 + next.len(),
    }
  }

  pub fn head(&self) -> Option<&i32> {
    match self {
        Cons(value, _) => Some(value),
        Nil => None,
    }
  }

  pub fn tail(&self) -> Option<&List> {
    match self {
      Cons(_, next) => Some(next),
      Nil => None,
    }
  }

  pub fn pop(&mut self) -> Option<i32> {
    match self.clone() {
        Cons(value, next) => {
          *self = *next;
          Some(value)
        },
        Nil => None,
    }
  }

  pub fn to_vec(&self) -> Vec<i32> {
    let mut vec_from_list = Vec::new();
    self.traverse_through_list(&mut vec_from_list);

    vec_from_list
  } 

  pub fn reverse(&mut self) -> List {
    let vec_from_list = self.to_vec();
    let mut new_list = List::new();

    for v in vec_from_list {
      new_list.prepend(v);
    }

    new_list
  }

  // Helper function only used here to traverse through the list and store each value in an array
  fn traverse_through_list(&self, value_vec: &mut Vec<i32>) -> Option<List> {
    match self {
        Cons(value, next) => {
          value_vec.push(*value);
          next.traverse_through_list(value_vec)
        },
        Nil => None
    }
  }
}

impl Iterator for List {
  type Item = List;
  fn next(&mut self) -> Option<Self::Item> {
    match self.clone() {
      Cons(_value, n) => {
        *self = *n.clone();
        Some(*n)
      }
      Nil => None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_empty() {
    let list = List::new();

    assert!(list.is_empty());
  }

  #[test]
  fn test_is_not_empty() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))))));

    assert!(!list.is_empty());
  }

  #[test]
  fn test_prepend() {
    let mut list = Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))));
    list.prepend(1);

    let expected_result = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))))));

    assert_eq!(list, expected_result);
  }

  #[test]
  fn test_len() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))))));

    assert_eq!(list.len(), 5);
  }

  #[test]
  fn test_head() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))))));

    assert_eq!(list.head(), Some(&1));
  }

  #[test]
  fn test_tail() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))))));
    let expected_result = Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))));

    assert_eq!(list.tail(), Some(&expected_result));
  }

  #[test]
  fn test_pop() {
    let mut list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))))));

    let new_list = Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))));

    let expected_value = list.pop();

    assert_eq!(list, new_list);
    assert_eq!(expected_value.unwrap(), 1);
  }

  #[test]
  fn test_to_vec() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))))));
    let expected_result = [1, 2, 3, 4, 5];
    let result = list.to_vec();

    assert_eq!(result, expected_result);
  }

  #[test]
  fn test_reverse_list() {
    let mut list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))))));
    let expected_result = Cons(5, Box::new(Cons(4, Box::new(Cons(3, Box::new(Cons(2, Box::new(Cons(1, Box::new(Nil))))))))));
    let result = list.reverse();

    assert_eq!(result, expected_result);
  }
}