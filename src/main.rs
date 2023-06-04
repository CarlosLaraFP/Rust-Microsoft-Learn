mod quote;
mod person;
mod file;
mod car;
mod auth;

use std::collections::HashMap;
use rayon::prelude::*;
use quote::*;
use car::*;
use person::*;
use file::*;
use std::f64::consts::PI;
use auth::*;
use regex::Regex;


mod math {
    type Complex = (f64, f64);
    pub fn sin(f: f64) -> f64 { f }
    pub fn cos(f: f64) -> f64 { f }
    pub fn tan(f: f64) -> f64 { f }
}


fn main() {
    /*
        In Rust, the & symbol is used to create a reference to a value.
        When you pass a value as a function argument using &, you are passing a reference to that value
        rather than the value itself. This can be useful because it allows you to avoid copying the value,
        which can be expensive if the value is large. Instead, you can pass a reference to the value and operate on it directly.
     */
    let sentence = "Hello world? This is a _beautiful_ world. !This is my home!";

    fn count_words(text: &str) -> HashMap<&str, u32> {
        let mut counts = HashMap::new();
        for word in text.split_whitespace() {
            let word = word.trim_matches(|c| {
                !char::is_alphabetic(c)
            });
            let count = counts.entry(word).or_default();
            // dereference operator to increment the actual value the reference points to
            *count += 1;
        }
        counts
    }

    let map = count_words(sentence);

    println!("{:?}", map);

    /*
        In Rust, the && symbol is used to create a reference to a reference, also known as a "double reference".
        A double reference is useful when you need to pass a reference to a reference as a function argument.
        This can occur when working with nested data structures or when you want to modify a reference to a value.
    */

    /*
        In Rust, a &str is an immutable reference to a string slice, while a String is a mutable, owned string.
        A string slice is a reference to a sequence of Unicode scalar values that are encoded in UTF-8 format.
        Since it is a reference, a string slice does not own the memory that contains the string data.
        Instead, it points to a range of memory that is owned by another object, such as a String or a byte array.
        A String, on the other hand, is a growable, mutable, and owned string.
        It owns the memory that contains the string data, and can be modified in place.
     */

    /*
        In a computer's memory, the heap and the stack are two distinct regions used for storing data.

        The stack is a region of memory that is used for storing variables that are declared inside a function.
        It is a contiguous block of memory that grows and shrinks as functions are called and returned.
        Whenever a function is called, the variables that are declared within the function are pushed onto the stack.
        When the function returns, the variables are popped off the stack.
        The stack is managed by the operating system and is typically limited in size.

        The heap is a region of memory that is used for storing dynamically allocated data.
        Dynamically allocated data is allocated using functions like malloc() or new,
        and the memory for this data is allocated on the heap. Unlike the stack,
        the heap is not managed automatically by the operating system.
        The programmer is responsible for allocating and deallocating memory on the heap.

        In summary, the stack is used for storing variables that are declared within a function,
        while the heap is used for storing dynamically allocated data.
        The stack is managed by the operating system and is limited in size,
        while the heap is managed by the programmer and can grow as needed (until the computer runs out of memory).
     */
    let result: String = map
        .iter()
        .map(|(key, value)| format!("{}: {}", key, value))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", result);

    fn sum_of_squares(input: &[i32]) -> i32 {
        input
            .iter()
            .map(|&i| i * i)
            .sum()
    }

    let slice = &[1, 2, 3];
    let expected = &14;
    /*
        Variable shadowing is a safe feature in Rust because the compiler ensures that the memory
        used by the old variable is freed when the new variable goes out of scope.
        This means that there are no memory leaks caused by variable shadowing.
        Variable shadowing in Rust does not cause memory leaks because Rust's
        ownership and borrowing system ensures that memory is managed safely and efficiently.
     */
    let result = &sum_of_squares(slice);
    assert!(result.eq(expected));

    fn sum_of_squares_par(input: &[i32]) -> i32 {
        input
            .par_iter()
            .map(|&i| i * i)
            .sum()
    }

    let par_result_commutative = &sum_of_squares_par(slice);

    assert!(par_result_commutative.eq(expected));

    // Declare first variable binding with name "shadow_num"
    let shadow_num = 5;
    // Declare second variable binding, shadows existing variable "shadow_num"
    let shadow_num = shadow_num + 5;
    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num = shadow_num * 2;
    println!("The number is {}.", shadow_num);

    let quote = from_quote_factory(11.23, true, Some(sentence));

    println!("{}", quote.amount);
    println!("{}", quote.uuid);
    println!("{}", quote.entangled.unwrap_or_default());
    println!("{:#?}", quote);

    let request = PostRequest {
        customer_id: 123456
    };
    let response = PostResponse {
        quote_uuid: String::from("cabc33f5-5edf-4ef0-b698-d3a33dfc15fe"),
        amount: 99.99
    };
    let request = Pricing::Request(request);
    let response = Pricing::Response(response);
    println!("\nPricing enum structure: \n\n {:#?} \n\n {:#?}", request, response);

    // Declare vector, initialize with three values
    let mut vector = vec![15, 3, 46];
    vector.push(1);
    let second = vector[1];
    vector[2] = vector[2] + 10;
    println!("Vector: {:?}, second = {}", vector, second);

    // Declare vector, value = "0", length = 5
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    let mut fruit = Vec::new();
    fruit.push("Apple");
    fruit.push("Strawberry");
    fruit.push("Orange");
    fruit.pop();
    // consider using a `let` binding to create a longer lived value
    let lower = fruit[1].to_lowercase();
    fruit[1] = lower.as_str();
    // temporary value is freed at the end of this statement
    // creates a temporary value which is freed while still in use
    // fruit[1] = fruit[1].to_lowercase().as_str();
    println!("Vector: {:?}", fruit); // borrow later used here

    let new_car = car_factory(
        9,
        9
    );

    println!("{:?}", new_car);

    let used_car = car_factory(
        2,
        150
    );

    println!("{:?}", used_car);

    let rust = "Programming in Rust";
    let scala = "Programming in Scala";
    let python = "Programming in Python";

    let mut reviews = HashMap::new();

    reviews.insert(rust, "Great mission");
    reviews.insert(scala, "Great foundation");
    reviews.insert(python, "Requires compiler");
    reviews.remove(python);

    let reviews_rust_key = reviews.get(rust).unwrap_or(&"Rust review is missing"); // .get immutable borrow occurs here
    println!("{:?}", reviews_rust_key);

    let scala_key = reviews.entry(scala).or_default(); // .entry mutable borrow
    *scala_key = "Great foundation for Rust systems programming"; // replaced with a new immutable string pointer
    // Rust allows it because the HashMap owns its keys, and or_default returns a mutable pointer to &str
    let fresh_binding = reviews.get(rust); // this works because it's a brand-new reference post-mutation(s)
    println!("{:?}", fresh_binding);
    println!("{:?}", reviews.get(python));
    println!("{:?}", reviews.get(scala));
    //println!("{:?}", reviews_rust_key); // immutable borrow later used here
    /*
        "cannot borrow `reviews` as mutable because it is also borrowed as immutable"
        "cannot borrow `reviews` as mutable more than once at a time"
        Somehow, alternating immutable borrows with mutable borrows
        for the same object is not allowed by Rust.
        I think it's because the HashMap would not be thread-safe.
        The problem is using a previous binding to a reference that might have changed since then
        (unsafe => memory safety cannot be guaranteed at compile time)
        In essence, the original key binding is no longer a pointer to the HashMap key's value.
     */

    // Define a hash map with <K, V> pairs.
    // The hash map keys will correspond to the car order numbers.
    // The hash map values will be the order details for each as defined in a Car struct.

    let mut car_orders = HashMap::new();

    let mut miles = 0;

    // A binding in Rust is a named reference to a value, which can be mutable or immutable
    for order in 1 .. 12 {
        car_orders.insert(order, car_factory(order, miles));
        println!("Car order {}: {:?}", order, car_orders.get(&order));

        // Reset miles for order variety
        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }

    let mut counter = 1;
    // stop_loop is set when loop stops
    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };
    // Loop should break when counter = 128
    println!("Break the loop at counter = {}", stop_loop); // 128

    // the addition operator commutes for integers
    //let accumulated =
    //println!("Break the loop at counter = {}", accumulated); // 128

    while counter < 150 {
        counter += 1;
    }
    println!("{}", counter);

    let big_birds = ["apple", "coconut", "orange"];
    for bird in big_birds.iter() {
        println!("Fruit: {}", bird);
    }

    for number in 0..3 {
        println!("{}", number * 2);
    }

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);

    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third);

    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);

    /*
        match arms are evaluated from top to bottom.
        Specific cases must be defined earlier than generic cases or they'll never be matched and evaluated.
        match arms must cover every possible value that the input type could have.
        You'll get a compiler error if you try to match against a non-exhaustive pattern list (no partial functions).
     */
    // Interesting, prefixing with & converts &i32 to usize
    for &index in [4, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }

    let a_number: Option<u8> = Some(7);
    match a_number {
        Some(7) => println!("That's my lucky number!"),
        _ => {}
    }
    /*
        An if let operator compares a pattern with an expression.
        If the expression matches the pattern, the if block is executed.
        The nice thing about if let expressions is that you don't need all the boilerplate code
        of a match expression when you're interested in a single pattern to match against.
     */
    if let Some(7) = a_number {
        println!("That's my lucky number, cleaner");
    }

    /*
        Because this function may panic, its use is generally discouraged.
        Instead, prefer to use pattern matching and handle the None case explicitly,
        or call unwrap_or, unwrap_or_else, or unwrap_or_default.
     */
    let gift = Some("computer");
    assert_eq!(gift.unwrap(), "computer");

    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat");

    let john = Person {
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };
    assert_eq!(build_full_name(&john), "James Oliver Smith");

    let alice = Person {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens"),
    };
    assert_eq!(build_full_name(&alice), "Alice Stevens");

    let bob = Person {
        first: String::from("Robert"),
        middle: Some(String::from("Murdock")),
        last: String::from("Jones"),
    };
    assert_eq!(build_full_name(&bob), "Robert Murdock Jones");

    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));

    if read_file_contents(std::path::PathBuf::from("src/main.rs")).is_ok() {
        println!("The program found the main file.");
    }
    if read_file_contents(std::path::PathBuf::from("non-existent-file.txt")).is_err() {
        println!("The program reported an error for the file that doesn't exist.");
    }

    fn change(text: &mut String) {
        text.push_str(", world");
    }

    fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    fn erase(_: String) { }

    #[derive(Debug)]
    struct Highlight<'document>(&'document str);
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    //erase(text);
    //let new = text;
    println!("{:?}", fox);
    println!("{:?}", dog);

    fn copy_and_return<'t>(vector: &mut Vec<String>, value: &'t str) -> &'t str {
        vector.push(String::from(value));
        value
    }

    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, name1));
    assert_eq!("Chris", copy_and_return(&mut names, name2));
    assert_eq!("Anne", copy_and_return(&mut names, name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    );

    #[derive(Debug, PartialEq)]
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let integer_and_boolean = Point { x: 5, y: false };
    let float_and_string = Point { x: 1.0, y: "hey" };
    let integer_and_float = Point { x: 5, y: 4.0 };
    let both_integer = Point { x: 10, y: 30 };
    let both_boolean = Point { x: true, y: true };

    trait Area<T> {
        fn area(&self) -> T;
    }

    struct Circle {
        radius: f64,
    }

    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Area<f64> for Circle {
        fn area(&self) -> f64 {
            PI * self.radius.powf(2.0)
        }
    }

    impl Area<f64> for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    let circle = Circle {
        radius: 2.0
    };

    assert_eq!(circle.area(), PI * 4.0);

    impl std::fmt::Display for Point<i32, i32> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let p1 = Point { x: 1, y: 2 };
    let mut p2 = Point { x: 4, y: -3 };

    if p1 == p2 { // can't compare two Point values!
        println!("equal!");
    } else {
        println!("not equal!");
    }

    println!("{}", p1); // can't print using the '{}' format specifier!
    println!("{:?}", p1); //  can't print using the '{:?}' format specifier!

    trait AsJson {
        fn as_json(&self) -> String;
        //fn increment(&mut self);
    }

    impl AsJson for Point<i32, i32> {
        fn as_json(&self) -> String {
            format!("{{x:{},y:{}}}", &self.x, &self.y)
        }
    }

    // Make sure to specify reference parameters to avoid unnecessary memory allocations
    fn send_data_as_json_v2(value: &mut impl AsJson) {
        println!("Sending JSON data to microcontroller...");
        println!("-> {}", value.as_json());
        //value.increment();
        println!("Done!");
    }

    // Runtime reflection
    fn send_data_as_json<T: AsJson>(value: &T) {
        println!(
            "Sending JSON data to {:?} microcontroller...",
            std::any::type_name::<T>().split("::").last().unwrap_or("Unknown")
        );
        println!("-> {}", value.as_json());
        println!("Done!");
    }

    println!("{:?}", &p2);
    send_data_as_json(&p2);
    println!("{:?}", &p2);

    struct Owner {
        name: String,
        age: u8,
        favorite_fruit: String,
    }

    struct Dog {
        name: String,
        color: String,
        likes_petting: bool,
    }

    impl AsJson for Owner {
        fn as_json(&self) -> String {
            format!(
                r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
                self.name, self.age, self.favorite_fruit
            )
        }
    }

    impl AsJson for Dog {
        fn as_json(&self) -> String {
            format!(
                r#"{{ "type": "dog", "name": "{}", "color": "{}", "likesPetting": {} }}"#,
                self.name, self.color, self.likes_petting
            )
        }
    }

    let laura = Owner {
        name: String::from("Laura"),
        age: 31,
        favorite_fruit: String::from("apples"),
    };

    let fido = Dog {
        name: String::from("Fido"),
        color: String::from("Black"),
        likes_petting: true,
    };

    send_data_as_json(&laura);
    send_data_as_json(&fido);

    struct Cat {
        name: String,
        sharp_claws: bool,
    }

    impl AsJson for Cat {
        fn as_json(&self) -> String {
            format!(
                r#"{{ "name": "{}", sharpClaws: {} }}"#,
                self.name, self.sharp_claws
            )
        }
    }

    let kitty = Cat {
        name: String::from("Kitty"),
        sharp_claws: false,
    };

    send_data_as_json(&kitty);

    #[derive(Debug)]
    struct Counter {
        length: usize,
        count: usize,
    }

    impl Counter {
        fn new(length: usize) -> Counter {
            Counter {
                count: 0,
                length,
            }
        }
    }

    impl Iterator for Counter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {

            self.count += 1;
            if self.count <= self.length {
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new(6);
    println!("Counter just created: {:#?}", counter);

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), Some(6));
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);  // further calls to `next` will return `None`
    assert_eq!(counter.next(), None);

    println!("Counter exhausted: {:#?}", counter);

    for number in Counter::new(6) {
        println!("{}", number);
    }

    let sum_until_10: usize = Counter::new(10).sum();
    assert_eq!(sum_until_10, 55);

    let powers_of_2: Vec<usize> = Counter::new(8).map(|n| 2usize.pow(n as u32)).collect();
    assert_eq!(powers_of_2, vec![2, 4, 8, 16, 32, 64, 128, 256]);

    /*
        Transform a Container struct that only accepts positive integers of the
        u32 type into a generic container that can hold values of any given type.
     */
    struct Container<T> {
        value: T,
    }
    impl<T> Container<T> {
        pub fn new(value: T) -> Self {
            Container { value }
        }
    }

    assert_eq!(Container::new(42).value, 42);
    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("Foo").value, "Foo");
    assert_eq!(Container::new(String::from("Bar")).value, String::from("Bar"));
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(Some("text")).value, Some("text"));

    // Implement an iterator that returns equal items in a sequence grouped in vectors
    // Input: [ 1, 1, 2, 1, 3, 3 ]
    // Output: [ [1, 1], [2], [1], [3, 3] ]

    struct Groups<T> {
        inner: Vec<T>
    }
    impl<T> Groups<T> {
        fn new(inner: Vec<T>) -> Self {
            Groups { inner }
        }
    }

    impl<T: PartialEq> Iterator for Groups<T> {
        type Item = Vec<T>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.inner.is_empty() {
                return None;
            }
            // let's check the span of equal items
            let mut cursor = 1;
            let first = &self.inner[0];
            for element in &self.inner[1..] {
                if element == first {
                    cursor += 1;
                } else {
                    break;
                }
            }
            // we use the `Vec::drain` to extract items up until the cursor
            // drain transfers ownership of specified item range from inner to items (move)
            let items = self.inner.drain(0..cursor).collect();
            // return the extracted items
            Some(items)
        }
    }

    let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
    // groups:     |->|---->|->|->|--->|----------->|--->|
    assert_eq!(
        Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![4],
            vec![1, 1],
            vec![2],
            vec![1],
            vec![3, 3],
            vec![-2, -2, -2],
            vec![5, 5],
        ]
    );

    let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
    // groups:      |->|---->|---->|----|->|----->|->|
    assert_eq!(
        Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![1],
            vec![2, 2],
            vec![1, 1],
            vec![2, 2],
            vec![3],
            vec![4, 4],
            vec![3],
        ]
    );

    println!("{}", math::cos(45.0));

    // Declare a private struct
    struct Foo;

    // Declare a public struct with a private field
    pub struct Bar {
        field: i32,
    }

    // Declare a public enum with two public variants
    pub enum State {
        PubliclyAccessibleVariant,
        PubliclyAccessibleVariant2,
    }

    let mut user = User::new("Charles", "super-secret");

    println!("{}", user.to_string());

    user.set_password("new-password");

    println!("{}", user.to_string());

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    println!("Did our date match? {}", re.is_match("2014-01-01"));

    assert_eq!(count_letters_and_numbers("221B Baker Street"), (12, 3));
    assert_eq!(count_letters_and_numbers("711 Maple Street"), (11, 3));
    assert_eq!(count_letters_and_numbers("4 Parkway Drive"), (12, 1));
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(32));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(65));
    }
}

/*
    pub: This means an item is public, and it can be accessed from any code that has access to the
    module that contains the item, including from outside of the crate itself. If your crate is
    used as a dependency by another crate, the other crate can access all pub items of your crate.

    pub(crate): This means an item is visible to your entire crate, but it is not accessible
    outside of your crate. This is useful when you want to allow different modules in your crate
    to use an item, but you do not want to expose it as part of your crate's public API.
 */
mod text_processing {

    pub(crate) mod letters {
        pub(crate) fn count_letters(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_alphabetic()).count()
        }
    }

    pub(crate) mod numbers {
        pub(crate) fn count_numbers(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_numeric()).count()
        }
    }
}
/*
    Why use usize instead of a fixed-size type? In most cases, if you're working with something
    where the size could change depending on the platform (like the length of a vector, which can
    theoretically hold different amounts of elements on 32-bit vs 64-bit platforms), you'd want to
    use usize. This allows your code to be portable between different architectures without having
    to worry about size limitations.

    But if you're dealing with data that has a known, fixed size that won't change regardless of
    platform (like a byte from a file, or a 32-bit pixel color value), you'd typically use one of
    the fixed-size types.
 */
fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    let number_of_letters = text_processing::letters::count_letters(text);
    let number_of_numbers = text_processing::numbers::count_numbers(text);
    (number_of_letters, number_of_numbers)
}
