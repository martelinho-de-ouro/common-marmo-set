// * To define a type with its variants.
// * Strutcs --> to group related fields and data | enums --> a way of saying a
//   value is one of a possible set of values.
// * We can put data directly into each enum variant.
// * The name of each variant also becomes a function that constructs an
//   instance of the enum.
// * Each variant can have different types and amounts of associated data.
// * We can put any kind of data inside an enum variant, including another enum.
// * We can defined methods using `impl` (similar to structs)
// * `Option` is another enum defined by stdlib and the Option type encodes the
//   scenario in which a value could be something or nothing.
// * Rust has no `null` feature.
//
#[derive(Debug)]
enum Fuel {
    Gasoline(f64, String), // variant (with data)
    Diesel(f64),           // variant (with data)
    Electricity(f64),      // variant (with data)
}

impl Fuel {
    fn consume(&self) {
        println!("consumed {:?}", &self);
    }
}

enum FuelOption {
    Gasoline,
    Diesel,
    Electricity,
}

fn main() {
    println!("{:?}", Fuel::Gasoline(5.60, String::from("abc")));
    println!("{:?}", Fuel::Diesel(6.1));
    println!("{:?}", Fuel::Electricity(7.2));

    println!("{:?}", Fuel::Electricity(7.2).consume());
    println!("{:?}", Fuel::Diesel(6.1).consume());

    println!("{}", price_in_brl(FuelOption::Gasoline));
    println!("{}", price_in_brl(FuelOption::Diesel));
    println!("{}", price_in_brl(FuelOption::Electricity));
}

fn price_in_brl(fuel_option: FuelOption) -> f64 {
    match fuel_option {
        FuelOption::Gasoline => 5.60,
        FuelOption::Diesel => 6.1,
        FuelOption::Electricity => 7.2,
    }
}
