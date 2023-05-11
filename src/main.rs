mod structs;
mod car;

use std::collections::HashMap;
use rayon::prelude::*;
use structs::*;
use car::*;


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
    println!("Break the loop at counter = {}", stop_loop);

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
}
