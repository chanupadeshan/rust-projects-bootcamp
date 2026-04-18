// use to get command line arguments
use std::env;
// use to read files
use std::fs;


fn main() {
    // let args = variable that holds the command line arguments as a vector of strings
    // Vec<String> is a growable array of strings - data type of the variable args
    // env::args() = this get the command line arguments 
    // when you run a Rust program in the terminal you can pass the values after the program name example: cargo run notes.txt
    // env::args() will produce somethings like this : "my_program", "notes.txt" - the first argument is always the program name and the second argument is the file name we want to read
    // .collect() == env::args() does not directly give a vector of strings, it gives an iterator that produces the command line arguments one by one. .collect() is used to take all the items produced by the iterator and put them into a vector of strings.
    let args: Vec<String> = env::args().collect();
    println!("------------------------------");
    println!("Command line arguments: {:?}", args);

    // match statement  = a control flow construct that allows you to compare a value against a series of patterns and execute code based on which pattern matches. In this case, we are matching the length of the args vector to determine if the user has provided the correct number of command line arguments.
    match args.len() {
        1 => {
            eprintln!("usage:{} <filename>", args[0]);
            std::process::exit(1); // exit the program with a error code
        }
        2 => {
            let file_path = &args[1]; // get the file path from the command line arguments
            println!("Reading file {}",file_path);

            // fs::read_to_string(file_path) is not directly giving the content of the file - ok(file_content) or Err(some_error)
            // .unwrap() = if it is ok(file_content) it will give the file content and if it is Err(some_error) it will panic and stop the program with and error message
            let content = fs::read_to_string(file_path).unwrap();

            let mut word_count = 0;

            // if I want to count the number of words in the file, I can use "_"(_word) to indicate that 
            // if I want to use each word in the content don't use "_" and just use "word" instead of "_word"
            for _word in content.split_whitespace(){
                word_count += 1;
            }
            println!("Word count: {}", word_count);
        }
        _ => {
            eprintln!("Too many arguments provided. Pass only the filename as an argument.");
            std::process::exit(1); // exit the program with a error code
        }
    }
}
