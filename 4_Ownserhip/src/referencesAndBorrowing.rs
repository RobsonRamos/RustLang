fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// The &  allow you to refer to some value without taking ownership of it. 
fn calculate_length(s: &String) -> usize {
    s.len()
}

//The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. 
// Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.

/* 

So what happens if we try to modify something we’re borrowing? it doesn’t work!
We can do this changing the previous code.

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

*/