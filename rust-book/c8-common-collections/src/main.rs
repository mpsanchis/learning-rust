use exercises::company_register::Employee;

mod vectors;
mod strings;
mod hashmaps;
mod exercises;

fn main() {
  println!("# Common collections code!");

  println!("\n## Vectors");
  vectors::update_vectors();

  vectors::do_not_respect_borrow_rules();

  vectors::iterate_vectors();

  println!("\n## Strings");
  strings::create_strings();
  
  strings::update_strings();
  
  strings::access_string_indices();
  
  strings::iterate_over_string();

  println!("\n## Hashmaps");
  let mut scores = hashmaps::create_hashmaps();

  hashmaps::access_hashmaps(&scores);

  hashmaps::hashmaps_ownership(&mut scores);

  hashmaps::update_hashmaps(&mut scores);

  println!("\n## Exercises");
  let numbers: Vec<i32> = vec![1,2,3,1,1,4,5,2]; // 1,1,1,2,2,3,4,5 -> mode: 1, median: 2
  exercises::median_and_mode::calculate_median_and_mode(&numbers);
  
  exercises::pig_latin::to_pig_latin("first");
  exercises::pig_latin::to_pig_latin("apple");
  exercises::pig_latin::to_pig_latin("");
  exercises::pig_latin::to_pig_latin("f");
  exercises::pig_latin::to_pig_latin("a");

  let mut company = exercises::company_register::Company {
    employee_list: Vec::new()
  };
  company.add_employee(Employee {name: String::from("Bob"), department: String::from("IT")});
  company.add_employee(Employee {name: String::from("Charles"), department: String::from("HR")});
  company.add_employee(Employee {name: String::from("David"), department: String::from("Finance")});
  company.add_employee(Employee {name: String::from("Alice"), department: String::from("Finance")});
  let all_employees = company.get_all_employees();
  println!("All employees: {all_employees:?}");

  let employees_finance = company.get_employees_by_department("Finance");
  println!("Employees in finance: {employees_finance:?}");
}



