use std::any::type_name;

/// this file is used to understand the difference between &str and String
/// in rust, there's no such thing as str in rust
/// when it comes to strings in rust, there's two types : &str and String
/// as the name suggest, &str is borrowed, and String is owned
/// define the function to check the type
fn type_checker<T>(_:T) -> &'static str {
// don't add semicolon here
    type_name::<T>()
}
fn main() {
    // by default, strings are borrowed, the type is &str
    let some_string : &str = "100";
    // this would not be able to be parsed
    
    // we use expect to handle errors in the event the appropraite resulting value isn't returned
    // the string/error message within expect is stored within the variable instead
    let parsed : i64 = some_string.parse().expect("There could be an error");
    
    
    // this is syntax for initializing an owned string
    // this will allocate memory within the heap
    let owned_string : String = "another string".to_string();
    
    
    // .trim() is used to remove whitespace characters around a string
    // when we change a string to .to_owned(), the variable type changes from &str -> String instead
    // _ since we are not using it at the moment
    let _another_parse : String = some_string.to_owned().trim().parse().expect("There could be potential error when parsing");
    
    println!(" some_string type {:?}", type_checker(parsed));
    println!("owned_string type {:?}", type_checker(owned_string));
}
t