pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Post {
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(PostState::Draft)),
      content: String::new(),
    }
  }

  pub fn add_text(&mut self, text: &str) {
    if self.state.as_ref().unwrap().is_editable() {
      self.content.push_str(text);
    }
  }

  pub fn content(&self) -> &str {
    self.state.as_ref().unwrap().content(self)
  }

  pub fn request_review(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.request_review())
    }
  }

  pub fn approve(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve())
    }
  }

  pub fn reject(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.reject())
    }
  }
}

/*
* STATES: Draft, PendingReview, Published
* must implement the State trait
*/

trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn is_editable(&self) -> bool;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn reject(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self, _post: &'a Post) -> &'a str {
    ""
  }
}

#[derive(Debug)]
enum PostState {
  Draft,
  PendingReview,
  Published
}

impl State for PostState {
  fn is_editable(&self) -> bool {
    matches!(self, PostState::Draft)
  }

  fn request_review(self: Box<Self>) -> Box<dyn State> {
    match self.as_ref() {
      PostState::Draft => Box::new(PostState::PendingReview),
      _ => self,
    }
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    match self.as_ref() {
      PostState::PendingReview => Box::new(PostState::Published),
      _ => self,
    }
  }

  fn reject(self: Box<Self>) -> Box<dyn State> {
    match self.as_ref() {
      PostState::PendingReview => Box::new(PostState::Draft),
      _ => self
    }
  }

  fn content<'a>(&self, post: &'a Post) -> &'a str {
    match self {
      PostState::Published => &post.content,
      _ => ""
    }
  }
}