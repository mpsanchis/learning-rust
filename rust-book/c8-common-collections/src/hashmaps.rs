use std::collections::HashMap;

pub fn create_hashmaps() -> HashMap<String, i32> {
  println!("\n### Creating hashmaps");
  let mut scores = HashMap::new();

  scores.insert(String::from("team1"), 10);
  scores.insert(String::from("team2"), 5);

  println!("scores map is: {scores:?}");

  return scores;
}

fn print_score(team_name: &str, scores: &HashMap<String, i32>) {
  let score_opt = scores.get(team_name);
  match score_opt {
    Some(score) => println!("{}'s score is: {}", team_name, score),
    None => println!("{}'s score does not exist", team_name)
  }
}

pub fn access_hashmaps(scores: &HashMap<String, i32>) {
  println!("\n### Accessing hashmaps");

  println!("The HashMap::get method returns an optional:");
  print_score("team1", scores);
  print_score("team42", scores);

  println!("We can iterate values with 'for (key, value) in &map' loop:");
  for (key, value) in scores {
    println!("key: {key} -> value: {value}");
  }
}

pub fn hashmaps_ownership(scores: &mut HashMap<String, i32>) {
  println!("\n### Ownership in hashmaps");
  
  println!("Defining a new String outside the HashMap: 'team3'");
  let team3 = String::from("team3");
  println!("team3 variable: '{team3}'");
  
  println!("scores.add(team3, 5) will take ownership of 'team3'");
  scores.insert(team3, 5);
  println!("Cannot use variable 'team3' anymore: owned by the HashMap");
  // let team3_score = scores.get(&team3); // Does not compile
  // println!("team3 was: {}", team3); // Does not compile, either
  println!("\n *However*, if the HashMap has references, the variables can be used after adding their references to the HashMap:");
  let mut scores2: HashMap<&String, i32> = HashMap::new();
  let new_string = String::from("new string");
  scores2.insert(&new_string, 500);
  
  println!("Created a new HashMap<&String, i32>: {scores2:?}");
  println!("I can still print the new_string key, because it is still the owner: new_string='{new_string}'");
}

pub fn update_hashmaps(scores: &mut HashMap<String, i32>) {
  println!("\n### Updating hashmaps");
  println!("\n#### A simple *insert* will override values:");
  println!("before: scores = {scores:?}");
  scores.insert(String::from("team3"), 5678);
  println!("After inserting '5678' to 'team3': scores = {scores:?}");
  
  println!("\n#### Using scores.entry(...).or_insert(...) will only insert if not present");
  scores.entry(String::from("team3")).or_insert(333);
  println!("After attempting to add '333'  to team3, scores = {scores:?}");
  
  println!("\n#### Using the result of scores.entry(...).or_insert(...) to modify the map");
  
  println!("Attempt to obtain score of 'team4', or add '0' to it: score_team_4 = scores.entry(String::from(\"team4\")).or_insert(0)");
  let score_team_4 = scores.entry(String::from("team4")).or_insert(0);
  println!("Dereference added '0' and add one to it: *score_team_4 += 1");
  *score_team_4 += 1;
  println!("scores = {scores:?}");
  
}