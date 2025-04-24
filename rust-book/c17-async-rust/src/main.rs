use trpl::Either;

mod futures_async_syntax;
mod applying_concurrency;
mod any_num_of_futures;
mod streams;
mod async_traits;

fn main() {
  println!("# Futures and async syntax (uncommenting requires args being passed to program)");
  // futures_async_syntax(); // use only

  println!("# Applying concurrency");
  applying_concurrency();

  println!("# Working with any number of futures");
  any_num_of_futures();

  println!("# Streams");
  streams();
}

fn futures_async_syntax() {
  println!("## Using a runtime to run async functions");
  let args: Vec<String> = std::env::args().collect();

  trpl::run(async {
    let title_fut_1 = futures_async_syntax::page_title(&args[1]);
    let title_fut_2 = futures_async_syntax::page_title(&args[2]);

    let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
      Either::Left(left) => left,
      Either::Right(right) => right,
    };

    println!("{url} returned first");
    match maybe_title {
      Some(title) => println!("Its page title is: '{title}'"),
      None => println!("Its title could not be parsed."),
    }
  })
}

fn applying_concurrency() {
  println!("## Using join(fut1, fut2)");
  applying_concurrency::tasks::join_two_futures();

  println!("\n## Awaiting fut1 and then awaiting fut2");
  applying_concurrency::tasks::await_two_futures();

  println!("\n## Awaiting fut1 after looping 2");
  applying_concurrency::tasks::await_fut1_after_loop2();

  println!("\n## Using a channel to pass information (simple example)");
  applying_concurrency::message_passing::simple_channel();

  //println!("\n## Using a channel to pass information (example with a future)");
  // issue: does not finish (user has to Ctrl+C) because tx is never dropped
  //applying_concurrency::message_passing::sending_msgs_with_delay();

  println!("\n## Using a channel to pass information (fixed)");
  applying_concurrency::message_passing::sending_msgs_with_delay_fixed();
}

fn any_num_of_futures() {
  println!("\n## Sending messages from two different transmitters, and reading from a single receiver");
  any_num_of_futures::message_passing::sending_msgs_three_futures();

  println!("\n## Racing a slow and a fast future with trpl::race");
  any_num_of_futures::race_and_yield::race_two_futures();

  println!("\n## Yielding control back to runtime with trpl::yield_now()");
  any_num_of_futures::race_and_yield::yielding();

  println!("\n## Building new abstractions with futures");
  any_num_of_futures::race_and_yield::own_abstractions();
}

fn streams() {
  println!("\n## Creating a stream from an iterator");
  streams::from_iterator::execute_stream_from_iterator();

  println!("\n## Creating a stream with timeouts from a channel and consuming it");
  streams::composing_streams::read_msgs_from_stream_with_timeout();

  println!("\n## Creating a stream by merging other streams");
  streams::merging_streams::read_msgs_from_composed_stream();
}