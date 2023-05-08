// process new car orders

#[derive(Debug)]
pub struct Car {
    make: String,
    model: String,
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
pub enum Transmission {
    Manual,
    SemiAuto,
    Automatic
}

pub fn car_factory(make: String, model: String, color: String, transmission: Transmission, convertible: bool) -> Car {
    Car {
        make,
        model,
        color,
        transmission,
        convertible,
        mileage: 0
    }
}