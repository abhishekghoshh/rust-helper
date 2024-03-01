# Rust style guide 

#### by Rust lang book
- If you want to stick to a standard style across Rust projects, you can use an automatic formatter tool called [rustfmt](https://doc.rust-lang.org/stable/book/appendix-04-useful-development-tools.html) to format your code in a particular style
- If you’re using more than one word in your filename, the convention is to use an underscore to separate them. For example, use hello_world.rs rather than helloworld.rs.
- Rust style is to indent with four spaces, not a tab.

#### Ownership rules
- Each value in rust has a variable that's called its owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

#### Reference rules
- We can create multiple mutable references.
- We can not create more than one mutable reference.
- mutable and immutable references can not work in same scope. Though we can create once one of them is out of scope.
- References must always be valid.