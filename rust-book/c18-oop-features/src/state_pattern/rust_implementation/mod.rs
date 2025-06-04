mod types;

use types::Post;

pub fn blog_post_rust() {
  println!("### Using different Post structs that implement different 'content', 'request_review', 'reject' and 'approve' methods, which consume the struct if they return a new state");

  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");

  let post = post.request_review();
  let post = post.approve();

  assert_eq!("I ate a salad for lunch today", post.content());
}