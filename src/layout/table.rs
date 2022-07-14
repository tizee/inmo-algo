/// Table layout
struct Table;

#[cfg(test)]
mod test_layout_table {
  use super::*;
  #[test]
  fn test_add_row() {
  }
  #[test]
  #[should_panic(expected= "hello")]
  fn test_bar() {
    panic!("hello");
  }
  // you can't use Reulst<T,E> with #[should_panic] annotation
}
