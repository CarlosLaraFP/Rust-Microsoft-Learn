#[derive(PartialEq, Debug)]
pub struct Person {
    pub first: String,
    pub middle: Option<String>,
    pub last: String,
}

pub fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first);
    full_name.push_str(" ");

    if let Some(name) = &person.middle {
        full_name.push_str(name);
        full_name.push_str(" ");
    }

    full_name.push_str(&person.last);
    full_name
}

#[derive(Debug)]
pub struct DivisionByZeroError;

pub fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}
