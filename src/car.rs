// process new car orders

// enum is like a Scala sealed trait's companion object and struct is like a mutable Scala case class

#[derive(PartialEq, Debug)]
pub struct Car {
    make: String,
    model: String,
    color: Color,
    transmission: Transmission,
    convertible: bool,
    age: (Age, u32)
}

#[derive(PartialEq, Debug)]
pub enum Transmission {
    Manual,
    SemiAuto,
    Automatic
}

#[derive(PartialEq, Debug)]
pub enum Age {
    New,
    Used
}

#[derive(PartialEq, Debug)]
pub enum Color {
    Blue,
    Green,
    Red,
    Silver
}

// Best practice: a struct should own its own data
pub fn car_factory(make: String, model: String, color: Color, transmission: Transmission, convertible: bool, miles: u32) -> Car {
    if miles >= 10 {
        if convertible {
            println!("Prepare a used car: {:?}, {:?}, Convertible, {} miles", transmission, color, miles);
        }
    }
    Car {
        make,
        model,
        color,
        transmission,
        convertible,
        age: car_quality(miles)
    }
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles < 10 {
        (Age::New, miles)
    }
    else {
        (Age::Used, miles)
    }
}

