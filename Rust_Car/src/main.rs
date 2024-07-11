// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual,
    SemiAuto,
    Automatic,
}

//use crate::Transmission::SemiAuto;
fn main() {
   // Order three cars
   let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
   println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
   let mut car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
   println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
   let mut car = car_factory(String::from("Black"), Transmission::Automatic, true);
   println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0
    }
}