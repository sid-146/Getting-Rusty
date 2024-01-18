<h1 style='text-align:center; color:#b7410e '>
Getting Rusty
</h1>

Rust is a modern, systems programming language that combines high performance with a strong emphasis on memory safety and zero-cost abstractions. What sets Rust apart is its ownership system, which enables fine-grained control over memory allocation without the need for a garbage collector. Rust is particularly well-suited for building robust and efficient systems, including operating systems, game engines, and networked applications.

<h2 style='text-align:center;'>Variables</h2>
<!-- ## Variables -->

`let` is used to create a variable

Syntax to declare Syntax

`let variable_name = value_to_assign`

In rust the variables are immutable by default to make them mutable `mut` keyword is used before variable name

`let mut variable_name = value_to_assign`

<h3 style='text-align:center;'>Sample Code for variable</h3>
<!-- ### Sample Code for variable -->

```
// default variable are immutable
let x = 5;
println!("This is immutable x : {}", x);

#This is immutable x : 5

// #[warn(unused_mut)] //! This will create a warning in case of variable in never updated
let mut y = 6;
println!("This is mutable y : {}", y);

#This is mutable y : 6

y = 7;
println!("This is mutable y : {}", y);

#This is mutable x : 7

const MY_COUNT: u32 = 100_000;
println!("This is const MY_COUNT : {}", MY_COUNT);

#This is const MY_COUNT x : 100000
```

### Shadowing

- Shadowing allows you to create a variable with same name
- But once the new value is assigned the old value is lost

### Why shadowing

- It preserves mutability
- see i have not used mut keyword still i am able to change the value of shadow variable
- also we can change the datatype of the variable

### Sample Code

```
let shadow = 10;
println!("This is int shadow : {}", shadow);

#This is int shadow : 10

let shadow = shadow * 2;
println!("This is shadow : {}", shadow);

#This is shadow : 20

let shadow = 10.9;
println!("This is string shadow : {}", shadow);

#This is int shadow : 10.9

let shadow = "This is string";
println!("This is string shadow : {}", shadow);

#This is int shadow : This is string
```

<p style= "text-align :center;">
Made by ❤ sid-146
</p>
