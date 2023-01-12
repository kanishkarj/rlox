# rust-lox

This is an implementation of Lox programming language, a Dynamically Typed Programming Language created by Bob Nystrom in his book [Crafting Interpreters](https://craftinginterpreters.com/) in rust. I built this while reading the book. It implements all the language features explained in the book like:

- Functions
- Classes and inheritance
- Dynamic dispatch
- run-time reflection
- Function Closures
- Mark and sweep garbage collection.

## Tests

The folder `test-scripts` contains lox script files categorized by the language feature, refer to that to get a better idea of the syntax. The folder `lox_tests` has the rust files execute the test lox scripts and validate their results.

- Run `cargo test` to run all tests.
- Specific tests can be run by specifying their module path, e.g. `cargo test tests::while_stmt::fun_in_body`. 
