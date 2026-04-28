# Lecture: Rust Fundamentals via a String Reverse CLI

This project is a simple Rust CLI program that reverses a string and demonstrates ownership, borrowing, mutable references, and basic terminal input/output.

---

## Running the Program

```bash
cargo run
```

The program will prompt you to enter a string, then it will print the reversed result in a few different ways.

---

## Project Files You May See

### Source files

* `Cargo.toml` → project configuration and dependencies
* `src/main.rs` → the actual Rust program

### Generated files and folders

These are created by Cargo or the Rust compiler when you build or run the project:

* `target/` → build output directory
* `target/debug/` → debug build artifacts
* `Cargo.lock` → locked dependency versions for reproducible builds
* `.rustc_info.json` → Rust compiler metadata used during builds
* `CACHEDIR.TAG` → cache marker file used by tooling

These generated files are normal in a Rust project and usually do not need to be edited manually.

---

## What This Program Does

* Prompts the user to enter a string
* Reads input from the terminal
* Reverses the string using a borrowed reference
* Reverses the string again by moving ownership
* Reverses the string in place using a mutable reference
* Prints all results to the terminal

---

# 1. Program Structure (Entry Point)

```rust
fn main() {
```

Every Rust program starts from `main()`.

### Key Idea:

* `main()` is the entry point
* Execution begins here

---

# 2. Importing Modules

```rust
use std::io::{self, write};
```

### What this module does:

| Module / Item | Purpose                         |
| ------------- | ------------------------------- |
| `std::io`     | Input and output utilities      |
| `write`       | Enables flushing terminal output |

### Concept:

Rust uses modules to organize functionality.

---

# 3. Printing a Prompt

```rust
print!("Enter a String to reverse:");
```

### Why `print!` instead of `println!`?

* `print!` keeps the cursor on the same line
* Useful for input prompts

---

# 4. Flushing Standard Output

```rust
io::stdout().flush().unwrap();
```

### Why is this needed?

Without flushing, the prompt may not appear immediately.

### Understanding This Line

#### `io::stdout()`

* Gets standard output

#### `.flush()`

* Forces buffered output to appear right away

#### `.unwrap()`

* If the write succeeds, continue
* If it fails, the program crashes

---

# 5. Reading User Input

```rust
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
```

### Understanding This

#### `String::new()`

* Creates an empty string

#### `let mut input`

* `mut` means the variable can be changed

#### `io::stdin().read_line(&mut input)`

* Reads one line from the terminal
* Stores it in `input`

### Return Type:

```rust
Result<usize, Error>
```

---

# 6. Trimming Whitespace

```rust
let input = input.trim();
```

### What does `.trim()` do?

* Removes leading whitespace
* Removes trailing whitespace
* Removes the newline from pressing Enter

### Important Concept:

After this line, `input` becomes a string slice (`&str`).

---

# 7. Borrowing a String Slice

```rust
let reversed = reverse_string(input);
```

### What is happening here?

* `input` is borrowed, not moved
* The function receives `&str`
* The original string data is not consumed

---

# 8. String Reversal with Borrowing

```rust
fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}
```

### Understanding This Function

#### `&str`

* A borrowed string slice
* Efficient and non-owning

#### `input.chars()`

* Converts the string into an iterator of characters

#### `.rev()`

* Reverses the iterator order

#### `.collect()`

* Collects the reversed characters into a new `String`

### Key Idea:

This is the cleanest and safest reversal approach in the program.

---

# 9. Converting Back to Owned `String`

```rust
let owned_input = String::from(input);
```

### Why do this?

The next function takes ownership of the string, so the borrowed slice must be converted back into an owned `String`.

---

# 10. Moving Ownership into a Function

```rust
let moved_result = reverse_move(owned_input);
```

### Key Concept: Ownership Transfer

* `owned_input` is moved into the function
* After the call, the original variable cannot be used again

---

# 11. Reversal by Taking Ownership

```rust
fn reverse_move(input: String) -> String {
    let bytes = input.as_bytes().to_vec();
    let mut reversed_bytes = bytes;
    reversed_bytes.reverse();
    String::from_utf8(reversed_bytes).unwrap()
}
```

### Understanding This Function

#### `input: String`

* The function takes ownership of the string

#### `input.as_bytes()`

* Converts the string into a byte slice

#### `.to_vec()`

* Copies the bytes into a vector

#### `.reverse()`

* Reverses the byte order

#### `String::from_utf8(...)`

* Converts bytes back into a `String`

### Note:

This method works, but reversing bytes directly can be unsafe for some Unicode text. The character-based approach is usually better.

---

# 12. Mutable Borrowing

```rust
let mut mutable_input = String::from(input);
reverse_in_place_safe(&mut mutable_input);
```

### Key Concept

* `&mut` gives a mutable reference
* The function can modify the original value without taking ownership

---

# 13. Reversal Using a Mutable Reference

```rust
fn reverse_in_place_safe(input: &mut String) {
    let reversed: String = input.chars().rev().collect();
    *input = reversed;
}
```

### Understanding This Function

#### `&mut String`

* A mutable borrow of the original string

#### `input.chars().rev().collect()`

* Builds a reversed string

#### `*input = reversed;`

* Replaces the original string value

### Key Idea:

This shows how mutable references let you change data in place.

---

# 14. Program Output

The program prints:

* The original prompt
* The reversed string using borrowing
* The reversed string using ownership transfer
* The reversed string using a mutable reference

---

# Program Flow

```text
Start
↓
Print prompt
↓
Flush terminal output
↓
Read user input
↓
Trim whitespace
↓
Reverse using borrowed string slice
↓
Reverse using ownership move
↓
Reverse using mutable reference
↓
Print results
↓
End
```

---

# Core Concepts Summary

| Concept                   | What It Does                              | Example                         |
| ------------------------ | ----------------------------------------- | ------------------------------- |
| `use`                    | Import modules                             | `use std::io`                   |
| `String`                 | Owned, growable text                       | `String::new()`                 |
| `&str`                   | Borrowed string slice                      | `input.trim()`                  |
| `mut`                    | Allows modification                        | `let mut input`                 |
| `stdin()`                | Read terminal input                        | `io::stdin()`                   |
| `stdout()`               | Write terminal output                      | `io::stdout()`                  |
| `.flush()`               | Force prompt to display immediately        | `flush()`                       |
| `.read_line()`           | Read one line from the terminal            | `read_line(&mut input)`         |
| `.trim()`                | Remove surrounding whitespace              | `input.trim()`                  |
| `&`                      | Borrow a reference                         | `reverse_string(input)`         |
| `&mut`                   | Borrow mutably                             | `reverse_in_place_safe(...)`    |
| Ownership move           | Transfer ownership into a function         | `reverse_move(owned_input)`     |
| `.chars()`               | Iterate through characters                 | `input.chars()`                 |
| `.rev()`                 | Reverse an iterator                        | `.chars().rev()`                |
| `.collect()`             | Collect iterator into a `String`           | `.collect()`                    |
| `.as_bytes()`            | Convert text to bytes                      | `input.as_bytes()`              |
| `.to_vec()`              | Copy a slice into a vector                 | `.to_vec()`                    |
| `from_utf8()`             | Convert bytes back to text                 | `String::from_utf8(...)`        |
| `unwrap()`               | Extract a success value or crash           | `.unwrap()`                     |
| `*input = ...`           | Replace the value behind a mutable borrow  | `*input = reversed`             |

---

# Learning Goal

This project is designed to teach:

* Ownership
* Borrowing
* Mutable references
* Terminal input and output
* Basic string handling in Rust
