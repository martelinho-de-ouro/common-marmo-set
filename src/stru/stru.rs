// * Strutcs are similar to tuples but we can name the attributes.
// * It is a general template for the type.
// * In case mutable, we need to add `mut` to the instance, not possible to `mut` the attributes directly.
// * It contains a thing called field-init-shorthand
// * It contains a thing called struct-update-syntax
// * It can be used like a tuple, and that is called tuple-struct
//   * To be used when we want to give the whole tuple a name and make the tuple a diff type from other tuples, and when naming each field would be verbose or boring.
// * It contains a thing called unit-like-struct ( strutcs with no fields )
//   * useful when we need to implement a trait on some type but don't have any data to be stored in the type itself.
struct Car {
    name: String,
    speed: i16,
    fabrication_date: String,
}

fn main() {
    let car1 = Car {
        name: String::from("Palio"),
        speed: 180,
        fabrication_date: String::from("01/01/1300"),
    };

    println!(
        "{:?} {:?} {:?}",
        car1.name, car1.speed, car1.fabrication_date
    );

    // using field-init-shorthand
    let car2 = make_a_car(String::from("foo"), 1, String::from("01/01/9002"));
    println!(
        "{:?} {:?} {:?}",
        car2.name, car2.speed, car2.fabrication_date
    );

    // using struct-update-syntax
    let car3 = Car {
        name: car2.name,
        speed: 10,
        fabrication_date: car2.fabrication_date,
    };
    println!(
        "{:?} {:?} {:?}",
        car3.name, car3.speed, car3.fabrication_date
    );

    // using struct-update-syntax with less code
    let car4 = Car {
        name: String::from("aaa"),
        ..car3 // the remaining fields will get the same values from other car.
    };
    println!(
        "{:?} {:?} {:?}",
        car4.name, car4.speed, car4.fabrication_date
    );

    // tuple-struct
    #[derive(Debug)]
    struct Foo(String, String, String);
    #[derive(Debug)]
    struct Bar(String, String, String);
    // same values, different types
    let foo_tuple_struct_instance = Foo(String::from("a"), String::from("b"), String::from("c"));
    let bar_tuple_struct_instance = Bar(String::from("a"), String::from("b"), String::from("c"));

    println!("{:?}", foo_tuple_struct_instance);
    println!("{:?}", bar_tuple_struct_instance);
    // unit-like-struct
    #[derive(Debug)]
    struct Never;
    let finished = Never;
    // sends to stdout
    println!("{:?}", finished);
    // sends to stderr
    dbg!(Never);
    dbg!(bar_tuple_struct_instance);
}

fn make_a_car(name: String, speed: i16, fabrication_date: String) -> Car {
    Car {
        name,
        speed,
        fabrication_date,
    }
}
