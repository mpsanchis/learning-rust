# Guessing Game

## Result Objects

Method `read_line` (from `std::io::stdin()`) returns a `Result` enum, which can be:
* `Ok`, or
* `Err`

The `Result` object has an `expect` method that causes the program to crash if the value of the Result is an `Err`. If the value of the Result is an `Ok`, then `expect()` will return the value that the Ok is holding.

## References

The `&` symbol represents a reference, so that the data is not copied in memory again. However, references are immutable by default the same ways values are. Therefore, `&mut guess` passes a mutable reference.