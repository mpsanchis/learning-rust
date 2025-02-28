use rand::random;

pub fn add_one(num: usize) -> usize {
  num + 1
}

pub fn add_rand(num: usize) -> usize {
  num + random::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
