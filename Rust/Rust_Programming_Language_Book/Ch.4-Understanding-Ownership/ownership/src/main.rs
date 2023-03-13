fn main() {
    println!("Hello, world!");




// OWNERSHIP
    // Data on the Stack is immutable, while on the Heap is mutable
    // But data on the Stack can be received faster

// OWNERSHIP RULES:

    // Each value in Rust has an owner.
    // There can only be one owner at a time
    fn main() {
        let s1 = String::from("hello");
        let s2 = s1; // At this point Rust invalidated s1, and only s2 is valid
    
        println!("{}, world!", s1); // THIS IS WRONG, because s1 is not valid
        println!("{}, world!", s2) // THIS IS CORRECT
                                    // When s2 goes out of scope, it alone will free the memory of two variables
                                    // So there'll be no memory corruption of double-freeing

        
    }

    // When the owner goes out of scope, the value will be dropped
    
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid

}
// Example of Ownership in Function:
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// Example of Return Values and Scope

fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}


// REFERENCES (Pointers):
// To prevent a function taking ownership of a variable (making it unreachable)
// We can use references

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // Here we use a reference

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Because of it the variable s1 will not be dropped here
// Refenrences are IMMUTABLE by default

// Using Mutable References:
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// You can't have two references to the same value though
// The lack of this measure have a risk of Data Race occuring,
// Data Race - Similar to Race Condition when:
    // Two or more pointers access the same data at the same time
    // At least one of the pointers is being used to write to the data,
    // There's no mechanism being used to synchronize access to the data.
    fn main() {
        let mut s = String::from("hello");
    
        let r1 = &mut s; // THIS WON'T COMPILE
        let r2 = &mut s; // THIS WON'T COMPILE
    
        println!("{}, {}", r1, r2);
    }
// We can however create new bodies allowing to have multiple mutable references

let mut s = String::from("hello");
{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.
let r2 = &mut s;

// We also cannot have mutable reference while we have an immutable one at the same value:
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}

fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// DANGLING REFERENCES:
// With languages like C, C++, a dangling pointer is a pointer that points to
// a location in memory that might have been freed

// SLICE TYPE
// Slices let you reference a contiguous sequence of elements,
// rather than the whole collection

fn main(){
    let s = String::from("hello there");
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
    print
}

