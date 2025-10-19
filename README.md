# Rust Helper

## Tutorial list:
- [rust lang book](https://doc.rust-lang.org/book/)
- [rust by example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Rust for Rustaceans](https://rust-for-rustaceans.com/)
- [Rust in Action](./pdf/rustinaction.pdf)
- [Zero To Production In Rust](./pdf/zero-to-production-in-rust-an-introduction-to-backend-development.pdf)
- [Idiomatic Rust](https://www.manning.com/books/idiomatic-rust)
- [Programming Rust: Fast, Safe Systems Development](./pdf/Programming%20Rust%202nd%20Edition.pdf)
- [Rust Atomics and Locks](https://marabos.nl/atomics/)
- [Black Hat Rust](https://kerkour.com/black-hat-rust)
- [Programming WebAssembly with Rust: Unified Development for Web, Mobile, and Embedded Applications](./pdf/hoffmanWasmRust.pdf)
- [Comprehensive Rust](./pdf/comprehensive-rust.pdf)


## Youtube 

### Introduction

- [Why Everyone's Switching to Rust (And Why You Shouldn't)](https://www.youtube.com/watch?v=meEXag1XCFw)


- **playlist**
  - [The Rust Lang Book](https://www.youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8)

- **Why rust is safe**
  - [PRESS RELEASE: Future Software Should Be Memory Safe](https://www.whitehouse.gov/oncd/briefing-room/2024/02/26/press-release-technical-report/)
    - [A PATH TOWARD SECURE AND MEASURABLE SOFTWARE](https://www.whitehouse.gov/wp-content/uploads/2024/02/Final-ONCD-Technical-Report.pdf)
    - [white house issues report that Rust is superior.](https://www.youtube.com/watch?v=T4ZUMvALdKI)
    - [Rust Recommended By White House??](https://www.youtube.com/watch?v=0BdePS7dx1I)

- **others**
  - [Rust YouTubers Tier List](https://www.youtube.com/watch?v=t6eaP2AjdyA)
  - [Why your Rust projects won't land you a job (the 5 levels of Rust projects)](https://www.youtube.com/watch?v=p-Uc1C2pHuA)

## Udemy
- [Ultimate Rust Crash Course](https://www.udemy.com/course/ultimate-rust-crash-course)
- [Ultimate Rust 2: Intermediate Concepts](https://www.udemy.com/course/ultimate-rust-2)
- [Learn Rust by Building Real Applications](https://www.udemy.com/course/rust-fundamentals)

#### To install rust use following command
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
You will also need a linker, which is a program that Rust uses to join its compiled outputs into one file. </br>
It is likely you already have one. If you get linker errors, you should install a C compiler, </br>
which will typically include a linker. A C compiler is also useful because some common Rust packages </br>
depend on C code and will need a C compiler. </br>

**On macOS, you can get a C compiler by running:** </br>
```
xcode-select --install
```
Linux users should generally install GCC or Clang, according to their distribution’s documentation. </br>
For example, if you use Ubuntu, you can install the build-essential package. </br>

#### using rustc
```
# create any project directory then start writing the rust file and compile using rustc command
rustc hello-world.rs

# you will get an executable in the same directory
./hello-world
```

#### using cargo to manage all your packages
```
# to create a new package use the following command
cargo new <project-name>

# to compile and build the project use this
cargo build
# it willl create a dev build and will put it inside the ./target/debug folder

# we can directly run the binary compiled file
./target//debug/hello-world

# we can also run rust project use the following command
cargo run

# Cargo also provides a command called cargo check. 
# This command quickly checks your code to make sure it compiles but doesn’t produce an executable
cargo check

# When your project is finally ready for release, you can use --release o compile it with optimizations
cargo build --release
# This command will create an executable in target/release instead of target/debug. 
# The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile.

# Add a new dependency in the cargo.toml under dependency section, and build the project
cargo build
```

## Rust Lang Book
- [sample](/rust-lang/sample)
- [hello-world](/rust-lang/hello-world)
- [guessing game](/rust-lang/guessing-game)
