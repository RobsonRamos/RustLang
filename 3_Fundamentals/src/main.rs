const MAX_POINTS: u32 = 100_000;

fn main() {
    
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;

    let y = y + 1;

    let y = y * 2;

     println!("The value of x is: {}", y);


    let spaces = "   ";
    let spaces = spaces.len();

    
    
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // booleans
    let t = true;
    let f : bool = false;

    // characters
    let c = 'z';
    let heart_eyed_cat = '😻'; // emoji

    //
    // compound types
    //

    // Tuples => Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup : (i32, f64, u8) = (500, 6.4, 1); 

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    // Arrays => Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5]; 

    let first = a[0];
    let second = a[1];

    another_function(5, 6);

    function_with_statement();


    println!("The value of this function is: {}", five());
    println!("The value of this function is: {}", plus_one(five()));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn function_with_statement(){
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };
}

fn five() -> i32{
    5
}


fn plus_one(x: i32) -> i32 {
    x + 1
}