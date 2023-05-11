// process new car orders

// enum is like a Scala sealed trait's companion object and struct is like a mutable Scala case class

#[derive(PartialEq, Debug)]
pub struct Car {
    make: String,
    model: String,
    pub color: String,
    pub motor: Transmission,
    pub roof: bool,
    pub age: (Age, u32)
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

pub fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];
    // Prevent panic: Check color index for colors array, reset as needed
    // Valid color = 1, 2, 3, or 4
    // usize is typically used as the type for array indices and pointers
    let mut color = order as usize;
    // If color > 4, reduce color to valid index
    if color > colors.len() {
        // color = 5 --> index 1, 6 --> 2, 7 --> 3, 8 --> 4
        color = color % colors.len();
    }

    // Add variety to orders for motor type and roof type
    let mut motor = Transmission::Manual;
    let mut roof = true;

    if order % 3 == 0 {          // 3, 6, 9
        motor = Transmission::Automatic;
    }
    else if order % 2 == 0 {   // 2, 4, 8, 10
        motor = Transmission::SemiAuto;
        roof = false;
    }                            // 1, 5, 7, 11

    Car {
        make: String::from("Maserati"),
        model: String::from("Folgore"),
        color: String::from(colors[(color-1) as usize]),
        motor,
        roof,
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

