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
            // dereference operator
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

    // Initialize counter variable
    let mut order = 1;
    // Declare a car as mutable "Car" struct
    let mut car: Car;

    // Order 6 cars, increment "order" for each request
    // Car order #1: Used, Hard top
    car = car_factory(order, 1000);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #2: Used, Convertible
    order = order + 1;
    car = car_factory(order, 2000);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #3: New, Hard top
    order = order + 1;
    car = car_factory(order, 0);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #4: New, Convertible
    order = order + 1;
    car = car_factory(order, 0);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #5: Used, Hard top
    order = order + 1;
    car = car_factory(order, 3000);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #6: Used, Hard top
    order = order + 1;
    car = car_factory(order, 4000);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);
}
