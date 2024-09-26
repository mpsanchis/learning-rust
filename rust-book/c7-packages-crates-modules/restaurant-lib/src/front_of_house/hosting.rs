mod table;

use table::{occupy_table, search_free_table};

pub fn add_to_waitlist() {}

pub fn seat_at_table() -> bool {
  let table = search_free_table();

  match table {
    Some(table_id) => {
      occupy_table(table_id);
      return true;
    },
    None => {
      return false;
    }
  }
}
