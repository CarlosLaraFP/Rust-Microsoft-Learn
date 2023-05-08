use uuid::Uuid;


pub struct Quote {
    pub uuid: Uuid,
    pub amount: f32,
    index: u8,
    accepted: bool
}

// if `Quote` is declared as a private type, then it can't be leaked
pub fn from_quote_factory(amount: f32, accepted: bool) -> Quote {
    Quote {
        uuid: Uuid::new_v4(),
        amount,
        index: 1,
        accepted
    }
}
