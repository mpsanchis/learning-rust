mod types;

use types::Post;

pub fn blog_post_oop() {
  println!("### Using a Post object with State objects inside that implement the 'content', 'request_review' and 'approve' functions, and delegating behaviour to them");
  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");
  assert_eq!("", post.content());

  post.request_review();
  assert_eq!("", post.content());

  post.approve();
  assert_eq!("I ate a salad for lunch today", post.content());
}