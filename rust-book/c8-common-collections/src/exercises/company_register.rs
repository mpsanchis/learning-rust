#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Employee {
  pub name: String,
  pub department: String
}

pub struct Company {
  pub employee_list: Vec<Employee>
}

impl Company {
  pub fn add_employee(&mut self, employee: Employee) {
    let pos = self.employee_list.binary_search(&employee).unwrap_or_else(|e| e);
    self.employee_list.insert(pos, employee);
  }

  pub fn get_all_employees(&self) -> &Vec<Employee> {
    return &self.employee_list;
  }

  pub fn get_employees_by_department(&self, dept: &str) -> Vec<&Employee>{
    return self.employee_list
      .iter()
      .filter(|employee| employee.department == dept)
      .collect();
  }
}