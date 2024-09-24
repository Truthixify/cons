use List::{Cons, Nil};

#[derive(Debug, PartialEq, Clone)]
pub enum List<T> {
  Cons(T, Box<List<T>>),
  Nil,
}

impl<T: Clone> List<T> {
  pub fn new() -> Self {
    Nil
  }

  pub fn is_empty(&self) -> bool {
    match self {
        Nil => true,
        _ => false
    }
  }

  pub fn prepend(&mut self, element: T) {
    *self = Cons(element, Box::new(self.clone()));
  }

  pub fn len(&self) -> usize {
    match self {
        Nil => 0,
        Cons(_, next) => 1 + next.len(),
    }
  }

  pub fn head(&self) -> Option<&T> {
    match self {
        Cons(value, _) => Some(value),
        Nil => None,
    }
  }

  pub fn tail(&self) -> Option<&List<T>> {
    match self {
      Cons(_, next) => Some(next),
      Nil => None,
    }
  }

  pub fn pop(&mut self) -> Option<T> {
    match self.clone() {
        Cons(value, next) => {
          *self = *next;
          Some(value)
        },
        Nil => None,
    }
  }

  pub fn to_vec(&self) -> Vec<T> {
    let mut vec_from_list = Vec::new();
    self.traverse_through_list(&mut vec_from_list);

    vec_from_list
  } 

  pub fn reverse(&mut self) -> List<T> {
    let vec_from_list = self.to_vec();
    let mut new_list = List::new();

    for v in vec_from_list {
      new_list.prepend(v);
    }

    new_list
  }

  // Helper function only used here to traverse through the list and store each value in an array
  fn traverse_through_list(&self, value_vec: &mut Vec<T>) -> Option<List<T>> {
    match self {
        Cons(value, next) => {
          value_vec.push(value.clone());
          next.traverse_through_list(value_vec)
        },
        Nil => None
    }
  }
}

impl<T: Clone> Iterator for List<T> {
  type Item = List<T>;
  fn next(&mut self) -> Option<Self::Item> {
    match self.clone() {
      Cons(value, n) => {
        *self = *n.clone();
        Some(Cons(value, Box::new(*n)))
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
    let list: List<i32> = List::new();

    assert!(list.is_empty());
  }

  #[test]
  fn test_is_not_empty() {
    let list = Cons(String::from("a"), Box::new(Cons(String::from("b"), Box::new(Cons(String::from("c"), Box::new(Cons(String::from("d"), Box::new(Cons(String::from("e"), Box::new(Nil))))))))));

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
    let list = Cons(String::from("a"), Box::new(Cons(String::from("b"), Box::new(Cons(String::from("c"), Box::new(Cons(String::from("d"), Box::new(Cons(String::from("e"), Box::new(Nil))))))))));

    assert_eq!(list.head(), Some(&String::from("a")));
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
    let list_i32 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))))));
    let expected_result_i32 = [1, 2, 3, 4, 5];
    let result_i32 = list_i32.to_vec();

    let list_string = Cons(String::from("a"), Box::new(Cons(String::from("b"), Box::new(Cons(String::from("c"), Box::new(Cons(String::from("d"), Box::new(Cons(String::from("e"), Box::new(Nil))))))))));
    let expected_result_string = [String::from("a"), String::from("b"), String::from("c"), String::from("d"), String::from("e")];
    let result_string = list_string.to_vec();

    assert_eq!(result_i32, expected_result_i32);
    assert_eq!(result_string, expected_result_string);
  }

  #[test]
  fn test_reverse_list() {
    let mut list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))))));
    let expected_result = Cons(5, Box::new(Cons(4, Box::new(Cons(3, Box::new(Cons(2, Box::new(Cons(1, Box::new(Nil))))))))));
    let result = list.reverse();

    assert_eq!(result, expected_result);
  }
}