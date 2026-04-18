# Lecture: Rust Fundamentals via a Word Counter CLI

This is your first Rust CLI project. Let's learn Rust fundamentals clearly and practically through this word-counting program.

---

## Running the Program

```bash
cargo run <filename>
```

### Example:

```bash
cargo run sample.txt
```

---

## Project Files You May See

This project contains both source files and generated build files.

### Source files

* `Cargo.toml` → project configuration and dependencies
* `src/main.rs` → the actual Rust program
* `sample.txt` → example input file

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

* Takes a file name from the command line
* Reads the file
* Counts the number of words
* Prints the result

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
use std::env;
use std::fs;
```

### What each module does:

| Module     | Purpose                                |
| ---------- | -------------------------------------- |
| `std::env` | Access command-line arguments          |
| `std::fs`  | File system operations (reading files) |

### Concept:

Rust uses modules like libraries to organize functionality.

---

# 3. Getting Command-Line Arguments

```rust
let args: Vec<String> = env::args().collect();
```

### Understanding This Line

#### `env::args()`

* Returns command-line arguments
* Example:

```bash
cargo run notes.txt
```

Produces:

```rust
["program_name", "notes.txt"]
```

#### `.collect()`

* Converts iterator → vector

#### `Vec<String>`

* A growable list of strings

---

## Example

```rust
args[0] → program name
args[1] → file name
```

---

# 4. Debug Output

```rust
println!("Command line arguments: {:?}", args);
```

### What is `{:?}`?

* Debug formatting
* Prints full vector structure

---

# 5. Pattern Matching with `match`

```rust
match args.len() {
```

### What is `match`?

A control structure used to compare values.

---

## Cases

### Case 1: No file provided

```rust
1 => {
		eprintln!("usage:{} <filename>", args[0]);
		std::process::exit(1);
}
```

* Prints error message
* Exits program

---

### Case 2: Correct input

```rust
2 => {
```

Program continues normally.

---

### Case 3: Too many arguments

```rust
_ => {
```

* `_` means "anything else"
* Handles invalid usage

---

# 6. Getting the File Path

```rust
let file_path = &args[1];
```

### Important Concept: Borrowing

* `&args[1]` → reference (not copy)
* More efficient
* No ownership transfer

---

# 7. Reading a File

```rust
let content = fs::read_to_string(file_path).unwrap();
```

### Understanding This

#### `fs::read_to_string()`

* Reads entire file into a `String`

#### Return Type:

```rust
Result<String, Error>
```

---

### What is `Result`?

```rust
Ok(value)  → success
Err(error) → failure
```

---

### What does `unwrap()` do?

```rust
.unwrap()
```

* If `Ok(value)` → return value
* If `Err(error)` → crash program

---

### Example

```rust
Ok("Hello").unwrap() → "Hello"
Err("error").unwrap() → panic
```

---

# 8. Mutable Variables

```rust
let mut word_count = 0;
```

### Key Concept

Rust variables are immutable by default.

```rust
let x = 5; // cannot change
```

To modify:

```rust
let mut x = 5;
x = 10;
```

---

# 9. Looping Through Words

```rust
for _word in content.split_whitespace() {
		word_count += 1;
}
```

---

## Understanding `.split_whitespace()`

Splits text by:

* spaces
* tabs
* new lines

---

## Example

```text
Hello Rust world
```

Becomes:

```text
Hello
Rust
world
```

---

## Why `_word`?

* `_` means "ignore this value"
* We only need the count, not the word

---

# 10. Printing Output

```rust
println!("Word count: {}", word_count);
```

### `{}` → placeholder for values

---

# 11. Error Output

```rust
eprintln!("error message");
```

### Difference:

| Function    | Output Type     |
| ----------- | --------------- |
| `println!`  | standard output |
| `eprintln!` | error output    |

---

# 12. Exiting the Program

```rust
std::process::exit(1);
```

### Meaning:

* `0` → success
* `1` → error

---

# Program Flow

```text
Start
↓
Read CLI arguments
↓
Check argument count
↓
If valid → read file
↓
Split text into words
↓
Count words
↓
Print result
↓
End
```

---

# Core Concepts Summary

| Concept               | What It Does              | Example            |
| --------------------- | ------------------------- | ------------------ |
| `use`                 | Import modules            | `use std::fs`      |
| `Vec<String>`         | Dynamic list of strings   | `args`             |
| `env::args()`         | Get CLI input             | `env::args()`      |
| `.collect()`          | Convert iterator → vector | `.collect()`       |
| `match`               | Pattern matching          | `match args.len()` |
| `&`                   | Borrow reference          | `&args[1]`         |
| `Result`              | Success/Error handling    | `Ok / Err`         |
| `unwrap()`            | Extract value or crash    | `.unwrap()`        |
| `mut`                 | Allow variable change     | `let mut x`        |
| `for`                 | Loop                      | `for word in ...`  |
| `.split_whitespace()` | Split text into words     | text processing    |
| `println!`            | Output                    | display result     |
| `eprintln!`           | Error output              | error messages     |
| `exit()`              | Stop program              | exit(1)            |

---

# What You Learned

From this one project, you learned:

✅ CLI input handling
✅ Vectors (`Vec`)
✅ Iterators and `.collect()`
✅ Pattern matching (`match`)
✅ File reading
✅ Result and error handling basics
✅ Mutable variables
✅ Loops
✅ String processing
✅ Program flow control

---

# Key Insight

This project is small, but it teaches real-world CLI structure.

This is similar to tools like:

* wc (word count)
* cat
* grep

---



# One-Line Summary

👉 This program is a Rust CLI tool that reads a file and counts its words while teaching core Rust fundamentals.
