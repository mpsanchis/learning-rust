use std::collections::HashMap;

fn median(sorted_numbers: &Vec<i32>) -> f32 {
  let len = sorted_numbers.len();

  if len == 1 {
    return sorted_numbers[0] as f32;
  } else if sorted_numbers.len() % 2 == 0 {
    let upper_mode = sorted_numbers[len/2] as f32;
    let lower_mode = sorted_numbers[len/2 - 1] as f32;
    return (upper_mode + lower_mode)/2.0;
  } else {
    return *sorted_numbers.get(len/2 - 1).unwrap() as f32;
  }
}

fn mode(numbers: &Vec<i32>) -> (i32, i32){
  if numbers.len() == 0 {
    return (0,0);
  }

  let mut mode: i32 = 0;
  let mut mode_occurrences: i32 = 0;

  let mut occurences: HashMap<i32, i32> = HashMap::new();

  for n in numbers {
    let occurences_number_n = occurences.entry(*n).or_insert(0);
    *occurences_number_n += 1;

    if *occurences_number_n > mode_occurrences {
      mode_occurrences = *occurences_number_n;
      mode = *n;
    }
  }

  return (mode, mode_occurrences);
}

pub fn calculate_median_and_mode(numbers: &Vec<i32>) {
  let mut sorted_numbers = (*numbers).clone();
  sorted_numbers.sort();
  
  let median = median(&sorted_numbers);
  println!("median({numbers:?}) = {median}");
  let (mode, mode_occurrences) = mode(&sorted_numbers);
  println!("mode({numbers:?}) = {mode} ({mode_occurrences} occurrences)");
}