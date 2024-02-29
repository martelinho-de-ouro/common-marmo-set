use std::any::Any;

/// `as` examples
fn main() {
    // primitive type conversion
    let x:i32 = 8;
    let y:f64 = x as f64;
    println!("{}", y); // 8

    // changing integer width
    let a: i32 = 255;
    let b: i16 = a as i16;
    println!("{}", b); // 255

    // converting enum variant
    enum TheEnum {
        A,
        B
    }
    let enum_a: TheEnum = TheEnum::A;
    let not_enum_b: i32 = enum_a as i32;
    println!("{}", not_enum_b); // 0

    // conventing between raw pointers and integers
    let z: *const u8 = &11;
    let w: usize = z as usize;
    println!("{}", w); // something like 94897023180892

    // converting to trait objects
    trait OtherTrait {}
    struct OtherStruct;
    impl OtherTrait for OtherStruct {}
    let other_struct = OtherStruct;
    let trait_object: &dyn OtherTrait = &other_struct as &dyn OtherTrait;
    println!("{:?}", &other_struct.type_id()); // TypeId { t: 145253946453286275321116384999504789661 }
    println!("{:?}", trait_object.type_id()); // TypeId { t: 284663888163317462848502332857878526020 }

}