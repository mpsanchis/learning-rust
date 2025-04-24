use trpl::StreamExt;

pub fn execute_stream_from_iterator() {
  println!("\n### Creating even numbers 2..20 and then keeping only multiples of 3 and 5");
  trpl::run(async {
    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = values.iter().map(|n| n * 2);
    let stream = trpl::stream_from_iter(iter);
    let mut filtered_stream = stream.filter(|n| {
      n % 3 == 0 || n % 5 == 0
    });

    while let Some(value) = filtered_stream.next().await {
      println!("Obtained value: {value}");
    }
  });
}