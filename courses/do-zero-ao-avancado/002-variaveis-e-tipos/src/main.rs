fn main() {
    // <=0>=
    let _number: i32 = 10;
    // >=0
    let _unsigned: u32 = 10;
    let _floating_point: f64 = 10.5;
    let string: &str = "Minha string";
    println!("{}", string);

    // mutability
    let mut _mutable_number: i32 = 10;
    println!("mutable_number: {_mutable_number}");
    _mutable_number = 20;
    println!("mutable_number: {_mutable_number}");

    // escopes
    {
        let _mutable_number: i32 = 210;
        println!("local_number: {_mutable_number}");
    }

    println!("mutable_number: {_mutable_number}");

    // shadowing
    let age = 25;
    let age = age + 1;
    println!("age: {age}");

    // constants
    const _DEFAULT_INDEX: i32 = 10;
    const _DEFAULT_NAME: &str = "John Doe";

    println!(
        "Default Index: {}\nDefault Name: {}",
        _DEFAULT_INDEX, _DEFAULT_NAME
    );

    // alias
    type Age = i32;
    let age: Age = 25;
    let age: Age = age + 1;
    println!("aliased age: {age}");

    // type conversion
    let age: Age = 25;
    let age: f64 = age as f64;
    println!("age as f64: {age}");
}
