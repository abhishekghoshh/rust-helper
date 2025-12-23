# Rust Programming Guide

## Table of Contents

- [Installation](#installation)
- [Rustc Compiler](#rustc-compiler)
- [Cargo Package Manager](#cargo-package-manager)
- [Rust Core Features](#rust-core-features)
- [Ownership and Borrowing](#ownership-and-borrowing)
- [Data Structures](#data-structures)
- [Error Handling](#error-handling)
- [Writing Idiomatic Rust](#writing-idiomatic-rust)

---

## Installation

### Installing Rust

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

You will also need a linker, which is a program that Rust uses to join its compiled outputs into one file. It is likely you already have one. If you get linker errors, you should install a C compiler, which will typically include a linker. A C compiler is also useful because some common Rust packages depend on C code and will need a C compiler.

### Installing C Compiler

**On macOS:**

```bash
xcode-select --install
```

**On Linux (Ubuntu/Debian):**

```bash
sudo apt update
sudo apt install build-essential
```

**On Linux (Fedora):**

```bash
sudo dnf install gcc
```

### Updating Rust

```bash
rustup update
```

### Checking Rust Version

```bash
rustc --version
cargo --version
rustup --version
```

### Uninstalling Rust

```bash
rustup self uninstall
```

---

## Rustc Compiler

The Rust compiler (`rustc`) compiles Rust source code into executable binaries.

### Basic Compilation

```bash
# Create and compile a Rust file
rustc hello-world.rs

# Run the executable
./hello-world
```

### Example: Hello World

```rust
// hello-world.rs
fn main() {
    println!("Hello, World!");
}
```

### Compilation with Optimizations

```bash
# Compile with optimizations
rustc -O hello-world.rs

# Compile with specific optimization level
rustc -C opt-level=3 hello-world.rs
```

### Additional Rustc Commands

```bash
# Show compiler version
rustc --version

# Explain compiler error codes
rustc --explain E0308

# Emit different output types
rustc --emit=asm,llvm-ir main.rs

# Set target architecture
rustc --target x86_64-unknown-linux-gnu main.rs
```

---

## Cargo Package Manager

Cargo is Rust's build system and package manager. It handles building code, downloading dependencies, and building those dependencies.

### Creating New Projects

```bash
# Create a new binary project
cargo new project-name

# Create a new library project
cargo new --lib library-name

# Create project in current directory
cargo init
```

### Building Projects

```bash
# Build debug version (fast compilation, slower execution)
cargo build

# Build release version (slower compilation, optimized execution)
cargo build --release

# Check code without producing executable (fastest)
cargo check
```

### Running Projects

```bash
# Build and run debug version
cargo run

# Build and run release version
cargo run --release

# Run with command-line arguments
cargo run -- arg1 arg2
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in parallel with thread count
cargo test -- --test-threads=1

# Run integration tests only
cargo test --test integration_test
```

### Documentation

```bash
# Build documentation for your project
cargo doc

# Build and open documentation in browser
cargo doc --open

# Build documentation for dependencies too
cargo doc --document-private-items
```

### Dependency Management

```bash
# Update dependencies to latest compatible versions
cargo update

# Add a dependency (requires cargo-edit)
cargo add serde

# Remove a dependency
cargo remove serde

# Show dependency tree
cargo tree
```

### Cleaning and Maintenance

```bash
# Remove target directory
cargo clean

# Format code according to Rust style guidelines
cargo fmt

# Run Clippy linter for better code quality
cargo clippy

# Fix automatically fixable warnings
cargo fix
```

### Publishing

```bash
# Package the project
cargo package

# Publish to crates.io
cargo publish

# Install a binary crate
cargo install ripgrep

# Uninstall a binary
cargo uninstall ripgrep
```

### Useful Cargo Commands

```bash
# Search for crates on crates.io
cargo search serde

# Show current Cargo version
cargo --version

# Verify project compiles
cargo verify-project

# Generate shell completions
cargo --list
```

---

## Rust Core Features

### 1. Memory Safety Without Garbage Collection

Rust ensures memory safety at compile time through its ownership system, eliminating the need for a garbage collector.

### 2. Zero-Cost Abstractions

Abstractions in Rust have no runtime overhead. High-level code compiles to efficient machine code.

### 3. Fearless Concurrency

Rust's ownership system prevents data races at compile time, making concurrent programming safer.

### 4. Pattern Matching

Powerful pattern matching allows for expressive and safe control flow.

```rust
fn describe_number(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1..=10 => "small",
        11..=100 => "medium",
        _ => "large",
    }
}
```

### 5. Type Inference

Rust has strong static typing with excellent type inference.

```rust
fn main() {
    let x = 5;              // i32 inferred
    let y = 3.14;           // f64 inferred
    let z = vec![1, 2, 3];  // Vec<i32> inferred
}
```

### 6. Traits (Similar to Interfaces)

Traits define shared behavior across types.

```rust
trait Greet {
    fn greet(&self) -> String;
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, I'm {}", self.name)
    }
}
```

### 7. Enums and Option Types

Rust's enums are powerful algebraic data types.

```rust
enum Status {
    Active,
    Inactive,
    Pending(String),
}

// Option type for handling null values
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}
```

---

## Ownership and Borrowing

Rust's ownership system is its most distinctive feature, ensuring memory safety without a garbage collector.

### Ownership Rules

1. Each value in Rust has a variable that's called its owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value is dropped

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2
    
    // println!("{}", s1);  // ERROR: s1 no longer valid
    println!("{}", s2);     // OK
}
```

### Borrowing (References)

Borrowing allows you to refer to a value without taking ownership.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // Borrow s1
    
    println!("The length of '{}' is {}", s1, len);  // s1 still valid
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope but doesn't drop the String
```

### Mutable References

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);  // Prints: hello, world
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

### Borrowing Rules

1. At any given time, you can have either one mutable reference OR any number of immutable references
2. References must always be valid

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;     // OK
    let r2 = &s;     // OK
    // let r3 = &mut s; // ERROR: cannot borrow as mutable while immutable refs exist
    
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used
    
    let r3 = &mut s; // OK now
    r3.push_str(" world");
}
```

### The Borrow Checker

The borrow checker is a compile-time analysis that ensures all borrowing rules are followed.

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    
    for i in &v {
        println!("{}", i);
        // v.push(4);  // ERROR: cannot modify while borrowed
    }
    
    v.push(4);  // OK: borrow from loop ended
}
```

### Lifetimes

Lifetimes ensure references are always valid.

```rust
// Explicit lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("Longest: {}", result);
    }
}
```

---

## Data Structures

### Scalar Types

```rust
fn main() {
    // Integers
    let x: i32 = 42;
    let y: u64 = 100;
    
    // Floating point
    let pi: f64 = 3.14159;
    
    // Boolean
    let is_rust_awesome: bool = true;
    
    // Character (4 bytes, Unicode)
    let emoji: char = '🦀';
}
```

### Compound Types

#### Tuples

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // Destructuring
    let (x, y, z) = tup;
    
    // Access by index
    let five_hundred = tup.0;
}
```

#### Arrays

```rust
fn main() {
    // Fixed size, same type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Initialize with same value
    let zeros = [0; 10];  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    
    // Access elements
    let first = arr[0];
}
```

### String Types

```rust
fn main() {
    // String slice (immutable, fixed size)
    let s: &str = "hello";
    
    // String (growable, heap-allocated)
    let mut string = String::from("hello");
    string.push_str(" world");
    
    // Converting between types
    let s2: String = s.to_string();
    let s3: &str = &string;
}
```

### Vectors

Dynamic arrays that can grow or shrink.

```rust
fn main() {
    // Creating vectors
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    
    // Adding elements
    v1.push(1);
    v1.push(2);
    
    // Accessing elements
    let third = &v2[2];
    
    match v2.get(2) {
        Some(third) => println!("Third element: {}", third),
        None => println!("No third element"),
    }
    
    // Iterating
    for i in &v2 {
        println!("{}", i);
    }
    
    // Modifying while iterating
    for i in &mut v1 {
        *i += 50;
    }
}
```

### HashMaps

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    // Inserting
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    
    // Accessing
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    
    // Iterating
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // Update only if key doesn't exist
    scores.entry(String::from("Yellow")).or_insert(50);
    
    // Update based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
```

### Structs

```rust
// Classic struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit struct
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1,
    };
    
    user1.email = String::from("newemail@example.com");
    
    // Struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    
    let black = Color(0, 0, 0);
}

// Methods
impl User {
    fn new(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    
    fn is_active(&self) -> bool {
        self.active
    }
}
```

### Enums

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    let msg = Message::Write(String::from("hello"));
    msg.call();
}
```

---

## Error Handling

### Result Type

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("hello.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}
```

### The `?` Operator

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

### Panic for Unrecoverable Errors

```rust
fn main() {
    // This will panic
    // panic!("crash and burn");
    
    // This will also panic on out of bounds access
    let v = vec![1, 2, 3];
    // v[99];  // Panic!
}
```

### Custom Error Types

```rust
use std::fmt;

#[derive(Debug)]
struct CustomError {
    details: String,
}

impl CustomError {
    fn new(msg: &str) -> CustomError {
        CustomError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for CustomError {}

fn risky_operation() -> Result<(), CustomError> {
    Err(CustomError::new("Something went wrong"))
}
```

---

## Writing Idiomatic Rust

### Use `match` for Control Flow

```rust
fn process_option(opt: Option<i32>) {
    match opt {
        Some(value) => println!("Got value: {}", value),
        None => println!("Got nothing"),
    }
}
```

### Prefer `if let` for Single Pattern Matching

```rust
fn main() {
    let some_value = Some(3);
    
    if let Some(3) = some_value {
        println!("three");
    }
}
```

### Use Iterators

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Functional style
    let sum: i32 = numbers.iter().sum();
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
}
```

### Use `impl Trait` for Return Types

```rust
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
```

### Destructuring

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    
    match p {
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

### Use Clippy for Better Code

```bash
cargo clippy
```

### Follow Rust Naming Conventions

- Types, traits: `PascalCase`
- Functions, variables: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Lifetimes: `'lowercase`

```rust
const MAX_POINTS: u32 = 100_000;

struct MyStruct;

fn my_function() {}

fn with_lifetime<'a>(s: &'a str) -> &'a str {
    s
}
```

### Documentation Comments

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

---

## Additional Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
- [Rustlings - Small exercises](https://github.com/rust-lang/rustlings)
- [Rust Playground](https://play.rust-lang.org/)
