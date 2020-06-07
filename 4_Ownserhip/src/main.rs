fn main() { 
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here


    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

    println!("{}", x);        

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.git s

/*
{                           // s is not valid here, it’s not yet declared
        let s = "hello";    // s is valid from this point forward
                            // do stuff with s
}

There is a natural point at which we can return the memory our String needs to the operating system: when s goes out of scope. W
hen a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. 
Rust calls drop automatically at the closing curly bracket.

*/

/*
    You’ll get an error like this because Rust prevents you from using the invalidated reference:

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);


    If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity 
    without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, 
    instead of being called a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2. 
    
    That solves our problem! With only s2 valid, when it goes out of scope, it alone will free the memory, and we’re done.

    In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. 
    Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
*/