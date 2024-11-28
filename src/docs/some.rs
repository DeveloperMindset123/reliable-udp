/// this is solely for understanding how Some() works
/// @see https://users.rust-lang.org/t/hi-i-am-new-to-rust-and-have-a-simple-question-about-some/2684/8
fn main() {
    // sum "wraps up" the integer 1, changing the possible values from
    // [1] -> [1, None]
    let mut val = Some(1);
    let integer_val = val.unwrap();

    // when it comes to printing out non-integer type
    // to handle any/all datatypes
    // we use {:?} within the formatter brackets
    println!(
        "val with wrapper : {:?},\n val unwrapped {:?}",
        val, integer_val
    );
}
