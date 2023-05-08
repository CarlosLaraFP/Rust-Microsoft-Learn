use uuid::Uuid;
/*
    v1 - Version 1 UUIDs using a timestamp and monotonic counter.
    v3 - Version 3 UUIDs based on the MD5 hash of some data.
    v4 - Version 4 UUIDs with random data.
    v5 - Version 5 UUIDs based on the SHA1 hash of some data.
      The versions below are unstable. They may be incomplete or depend on other unstable libraries. Unstable features may break between minor releases.
    v6 - Version 6 UUIDs using a timestamp and monotonic counter.
    v7 - Version 7 UUIDs using a Unix timestamp.
    v8 - Version 8 UUIDs using user-defined data.
    zerocopy - adds support for zero-copy deserialization using the zerocopy library.
 */

/*
    The #[derive(Debug)] attribute in Rust is used to automatically generate an implementation
    of the Debug trait for a struct or an enum. The Debug trait is used to format a value for
    debugging purposes, and it is used by the println! macro when using the {:?} format specifier.
    By deriving the Debug trait, the struct or enum can be printed using the println! macro
    with the {:?} format specifier to print the debug representation of the value.
    The debug representation is intended to be easily readable by developers and provides a way
    to quickly check the contents of a struct or enum.
 */
#[derive(Debug)]
pub struct PostRequest {
    pub customer_id: i32
}

#[derive(Debug)]
pub struct PostResponse {
    pub quote_uuid: String,
    pub amount: f32
}

/*
    Each variant in the enum isn't its own type. Any function that uses a variant of the
    Pricing enum must accept all the variants in the enum. We can't have a function that
    accepts only the Request variant, but not the other variants.
 */
#[derive(Debug)]
pub enum Pricing {
    // 2 variants
    Request(PostRequest),
    Response(PostResponse)
}

/*
    In Rust, structs are typically designed to hold their own data, rather than references to external data.
    This is because Rust's ownership model and borrowing rules require that the owner of data is
    responsible for managing its lifetime and preventing multiple mutable references to the same data.
    If a struct held a reference to external data, it would be difficult to manage the lifetime of that data correctly,
    and it would be more prone to memory safety issues such as dangling pointers and use-after-free errors.
    To avoid these issues, Rust does not allow structs to hold references by default.
    However, it is still possible to create structs that hold references using the lifetime parameter syntax.
    This allows the struct to hold a reference to some data, but the data must live at least as long as the struct itself.
    This ensures that the data is always valid while the struct holds a reference to it.
    Quote holds a reference to a string slice using the lifetime parameter 's.
    The lifetime 's ensures that the string slice lives at least as long as the struct itself, so that the reference remains valid.
 */
#[derive(Debug)]
pub struct Quote<'s> {
    pub uuid: Uuid,
    pub amount: f32,
    index: u8,
    accepted: bool,
    pub entangled: Option<&'s str>
}

pub fn from_quote_factory(amount: f32, accepted: bool, entangled: Option<&str>) -> Quote {
    // Arguments can be passed in any other because struct parameter names are unique
    Quote {
        amount,
        accepted,
        entangled: Option::from(entangled.unwrap_or_default()),
        uuid: Uuid::new_v4(),
        index: 1
    }
}

/*
    In Rust, a macro is a way to write code that writes code.
    Macros allow you to define a piece of code that can be reused in multiple places and can be
    parameterized with different values, types, or expressions. Rust macros are similar to macros
    in other programming languages like C or Lisp, but they are more powerful and flexible because
    Rust macros are implemented as part of the Rust compiler, rather than being implemented as part of the preprocessor.

    There are two types of macros in Rust: declarative macros and procedural macros.

    Declarative macros are defined using the macro_rules! macro.
    They are also sometimes called "macro by example" macros because they are defined by providing
    examples of input and output. Declarative macros are expanded at compile-time and allow you to
    define new syntactic constructs that are not available in the language.
    Examples of declarative macros include vec! and println!.

    Procedural macros, on the other hand, are defined using Rust code that is executed at compile-time.
    Procedural macros can be used to define custom attributes, derive traits, or generate code
    based on Rust code. Procedural macros are more flexible than declarative macros because they can
    generate code that depends on the values of variables and expressions at compile-time.
    Examples of procedural macros include #[derive(...)] and #[wasm_bindgen].

    Overall, macros are an important tool in Rust for reducing code duplication, increasing expressiveness, and enabling advanced metaprogramming techniques.
 */
