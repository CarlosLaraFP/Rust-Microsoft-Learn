// process new car orders

// enum is like a Scala sealed trait and struct is like a Scala case class

#[derive(PartialEq, Debug)]
pub struct Car {
    make: String,
    model: String,
    color: String,
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

pub fn car_factory(make: String, model: String, color: String, transmission: Transmission, convertible: bool) -> Car {
    Car {
        make,
        model,
        color,
        transmission,
        convertible,
        age: (Age::New, 0)
    }
}

pub fn car_quality(miles: u32) -> (Age, u32) {
    let quality: (Age, u32) = (Age::New, miles);
    quality
}

