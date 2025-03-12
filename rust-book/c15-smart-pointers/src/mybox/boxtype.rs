use std::ops::{Deref, DerefMut};

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
  pub fn new(item: T) -> Self {
    MyBox(item)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.0
  }
}

impl<T> DerefMut for MyBox<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}