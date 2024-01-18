fn main() {
    println!("This are variables");

    // default variable are immutable
    let x = 5;
    println!("This is immutable x : {}", x);

    // #[warn(unused_mut)] //! This will create a warning in case of variable in never updated
    let mut y = 6;
    println!("This is mutable y : {}", y);

    y = 7;
    println!("This is mutable y : {}", y);

    const MY_COUNT: u32 = 100_000;
    println!("This is const MY_COUNT : {}", MY_COUNT);

    // Shadowing
    /*
        * Shadowing allows you to create a variable with same name
        * But once the new value is assigned the old value is lost

        ! Why shadowing
        * It preserves mutability
        * see i have not used mut keyword still i am able to change the value of shadow variable
        * And also we can change the datatype of the variable

    */
    let shadow = 10;
    println!("This is int shadow : {}", shadow);

    let shadow = 10.9;
    println!("This is string shadow : {}", shadow);

    let shadow = "This is string";
    println!("This is string shadow : {}", shadow);


    println!("This are data types")
    /*
        ?Scaler Datatype
        signed integer : i8,i16,i32,i64,i128
        Unsigned integer : u8,u16,u32,u64,u128

        Integers can be also assigned in decimal, hex, Octal, binary and byte (byte only Unsigned)

        !Integer overFlow
        When integer overflow occurs
     */
}
