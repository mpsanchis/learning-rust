pub fn search_free_table() -> Option<u8> {
  let there_are_free_tables = true;

  if there_are_free_tables {
    return None
  } else {
    return Some(5)
  }
}

pub fn occupy_table(table_id: u8) {
  // occupy table
}