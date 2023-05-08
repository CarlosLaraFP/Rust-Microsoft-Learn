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

// Quote holds a reference to a string slice using the lifetime parameter 's.
// The lifetime 's ensures that the string slice lives at least as long as the struct itself, so that the reference remains valid.
pub struct Quote<'s> {
    pub uuid: Uuid,
    pub amount: f32,
    index: u8,
    accepted: bool,
    pub entangled: Option<&'s str>
}

// if `Quote` is declared as a private type, then Rust does not allow leaking it
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
