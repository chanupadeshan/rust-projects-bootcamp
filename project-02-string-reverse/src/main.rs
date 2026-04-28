// io : use for reading user input
// Write : use for writing output to the terminal
use std::io::{self, Write};

fn main() {
    println!("String Reverse Tool (ownership and borrowing)");
    print!("Enter a String to reverse:");
    // flush the output to ensure the prompt is displayed before waiting for input
    // without this user does not see the Enter a String to reverse: prompt until they enter something
    // with this the prompt is displayed immediately and then the program waits for user input
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim(); // remove any leading or trailing whitespace from the input string

    let reversed = reverse_string(input);
    println!("Reversed String: {}", reversed);

    // After line 15 (`let input = input.trim();`), `input` is no longer an owned `String`.
    // It becomes an `&str` slice that borrows from the original string.
    // need to convert the '&str' back to 'String' to pass it to the reverse_move function which takes ownership of the string
    let owned_input = String::from(input);
    let moved_result = reverse_move(owned_input);   
    println!("Reversed String (moved): {}", moved_result);


    let mut mutable_input = String::from(input);
    // does not pass the ownership of the string to the function, instead it borrows a mutable reference to the string, allowing the function to modify the original string without taking ownership of it.
    reverse_in_place_safe(&mut mutable_input);
    println!("In-place reversal: {}", mutable_input);
}

// when I pass a variable into a function, Rust will move the ownership of that variable to the function.
// after passing a variable into the function, the original variable is no longer valid and cannot be used in the rest of the program.

fn reverse_string(input: &str) -> String{
    // input.chars() = this converts the input string into an iterator of characters
    // .rev() = this reverses the order of the characters in the iterator
    input.chars().rev().collect()
}


fn reverse_move(input: String) -> String{
    // this line turns the string into a vector of bytes
    // input.as_bytes() = gives the bytes of the string as a slice (&[u8])
    // .to_vec() = make new vector of bytes from the slice of bytes, this is necessary because we want to reverse the bytes and we cannot reverse a slice directly, we need to have a vector that we can modify
    let bytes = input.as_bytes().to_vec();
    let mut reversed_bytes = bytes;
    // reverse the order of the bytes in the vector
    reversed_bytes.reverse();
    // convert the reversed vector of bytes back into a string
    String::from_utf8(reversed_bytes).unwrap()
}


fn reverse_in_place_safe(input: &mut String){
    // input.chars() = this converts the input string into an iterator of characters
    // .rev() = this reverses the order of the characters in the iterator
    // .collect() = this collects the reversed characters into a new string
    let reversed: String = input.chars().rev().collect();
    // in this function input is not real string. It is a mutable reference to a string, so we need to point to the actual string using '*'
    // this line relpace the original string with reversed string
    *input = reversed;
}