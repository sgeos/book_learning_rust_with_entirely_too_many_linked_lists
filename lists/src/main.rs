fn main() {
  println!("Hello world!");
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn main_ok() {
    main();
  }
}

