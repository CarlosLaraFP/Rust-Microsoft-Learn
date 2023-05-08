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
            *count += 1;
        }
        counts
    }

    let map = count_words(sentence);

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

    let car = car_factory(
        String::from("Maserati"),
        String::from("Grecale"),
        String::from("Grigio Lava Metallic"),
        Transmission::Automatic,
        false
    );

    println!("{:?}", car);
}
