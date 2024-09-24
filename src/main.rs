fn main() {
  use cons::List::{Cons, Nil};
  let list = Cons(String::from("a"), Box::new(Cons(String::from("b"), Box::new(Cons(String::from("c"), Box::new(Cons(String::from("d"), Box::new(Cons(String::from("e"), Box::new(Nil))))))))));

  for l in list {

    println!("{l:?}");
  }
}