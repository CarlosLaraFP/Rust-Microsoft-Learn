// lib.rs contains the code exposed as a public API

/*
    From the output, we can see that Rust places the results of the tests in separate sections.
    Unit test results come first, then integration results, and finally, documentation results.

    In the integration tests section, we can see that our two tests inside the tests/pizzas.rs
    file were collected and executed by the test suite.

    Only library crates can be tested via integration tests because binary crates don't expose
    any functionality that other crates can use. As a result, many Rust binary crates include a
    src/lib.rs file that contains most of the code in src/main.rs. Integration tests can then
    test the binary's functionality by importing the crate as a library with use.
 */

pub struct Pizza {
    pub topping: String,
    pub inches: u8,
}

impl Pizza {
    pub fn pepperoni(inches: u8) -> Self {
        Pizza::bake("pepperoni", inches)
    }

    pub fn mozzarella(inches: u8) -> Self {
        Pizza::bake("mozzarella", inches)
    }

    fn bake(topping: &str, inches: u8) -> Self {
        Pizza {
            topping: String::from(topping),
            inches,
        }
    }
}

/// Generally, the first line is a brief summary describing the function.
///
/// The next lines present detailed documentation.
/// Code blocks start with triple backticks. The code has an implicit fn main() inside and extern crate <cratename>,
/// which means you can just start writing code.
///
/// ```
/// let result = Rust_Microsoft_Learn::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/*
    The cfg attribute controls conditional compilation and will only compile the thing it's attached
    to if the predicate is true. The test compilation flag is issued automatically by Cargo whenever
    we execute the command $ cargo test, so it will always be true when we run our tests.

    The use super::*; declaration is necessary for the code inside the add_function_tests module
    to access the add in the outer module.
 */
#[cfg(test)]
mod add_function_tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(2, 2), 7);
    }

    #[test]
    #[ignore = "not yet reviewed by the Q.A. team"]
    fn add_negatives() {
        assert_eq!(add(-2, -2), -4)
    }
}