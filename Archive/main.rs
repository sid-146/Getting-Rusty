fn main() {
    println!("Hello World");
    let result_add: i32 = add(10, 20);
    println!("result {}", result_add);
    let result_sub: f32 = sub(16.0, 11.56);
    println!("result sub {}", result_sub);
    let result_multi: f32 = multiply(16.0, 11.56);
    println!("result multi {}", result_multi);

    let error_result = make_error(10.0, 10.0); 
    match error_result {
        Ok(value) => println!("Error result : {}", value),
        Err(error) => println!("Error result : {}", error),
    }
}

fn add(a: i32, b: i32) -> i32 {
    let sum: i32 = a + b;
    // println!("variable {}", a);
    return sum;
}

fn multiply(a: f32, b: f32) -> f32 {
    let multi: f32 = a * b;
    return multi;
}

fn sub(a: f32, b: f32) -> f32 {
    let minus: f32 = a - b;
    return minus;
}

fn make_error(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Divide by zero error"))
    } else {
        Ok(x / y)
    }
}
