fn main() {
    // {} is replace with the 31
    println!("{} days", 31);

    // positional args
    println!("{0} first arg, {1} second arg", '1', '2');

    // Keyword args
    println!(
        "{object} object, {verb} verb, {subject} subject",
        object = "object",
        verb = "verb",
        subject = "subject"
    );

    // different formatting for char format
    println!("Base 10:               {}", 10);
    println!("Base 2 (binary):       {:b}", 10);
    println!("Base 8 (octal):        {:o}", 10);
    println!("Base 16 (hexadecimal): {:x}", 10);
    println!("Base 16 (hexadecimal): {:X}", 10);

    // Specify the right spacing
    println!("{number:>5}", number = 1);

    // add extra extra zero in end
    println!("{number:0<5}", number=1);

    // By appending $ we can use the keyword arg
    println!("{number:0>width$}", number=1, width=5);

    // dead_code disable warning below line will not be complied
    #[allow(dead_code)]
    struct Structure(i32);


    // To check the dead code uncomment the below line
    // fmt::Display
    // println!("This is struct {} this won't print....", Structure(3));

    let width: f64 = 1.0;
    let height:usize = 3;

    println!("{width:>height$}");


 
}
