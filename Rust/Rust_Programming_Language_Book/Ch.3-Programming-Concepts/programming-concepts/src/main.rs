// The content here contains code snippets I wasn't 100% aware of

fn main() {
    println!("Hello, world!");

    let mut x = 5; // mut means we can change the value inside the variable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    // A constant is a variable that is valid for the entire time a program runs, 
    // With global scope
    // Can't be changed


    // SHADOWING:
    // Can let us change the value of a variable withing a certain scope
    let x = 5;
    let x = x + 1 // Variable x shadowed within the same scope

    {
        let x = x * 2; // Variable x shadowed within different scope
        println!("The value of x in the inner scope is: {x}"); // returns 12
    }

    println!("The value of x is: {x}"); // returns 6
    // By using shadowing we can still keep the variable immutable after transformations have been completed
    // Without 'let' keyword there'll be compile error
    // It also allows us to parse the input the user has given without creating new variable:
    let spaces = "   ";
    let spaces = spaces.len();

    // COMPOUND TYPES: (With multiple values of variety types)
    // Tuples:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    // or:
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    // Unit - A tuple without any type

    // Arrays:
    // Must have the same type
    let a = [1, 2, 3, 4, 5]
    let a: [i32; 5] = [1,2,3,4,5] // The type of i32, with 5 fixed length
    let a = [3; 5]; // An Array with 5 elements with ALL SET to the value 3 initially
    // Useful for:
        // Having data allocated on the Stack rather than on the Heap
        // When you need fixed number of elements
    let first = a[0];
    let second = a[2]
    let second = a[10] // Will produce runtime error
    // Rust will check that the index you've specified is less than the array length
    // ^ That's and example of Rust's memory safety principles in action
    // ^ In many low-level languages this is not done
    
    // Vectors: 
    // Allowed to grow or shrink size
    let v = vec![1,2,3];


    // FUNCTIONS:
    fn function1() {
        let y = {
            let x = 3; // Is a statement that doesn't have return value
            x + 1  // Is an expression (because it doesn't have semicolon and has a return value)
        };
    
        println!("The value of y is: {y}");
    }

    fn five() -> i32 {
        5 // Because We've declared the type of a return value,
        // this Function can return values to the code that calls them
    }
    
    fn function_that_uses_five() {
        let x = five();
    
        println!("The value of x is: {x}"); // Returns 5
    }

    fn main() {
        let x = plus_one(5);
    
        println!("The value of x is: {x}");
    }
    
    fn plus_one(x: i32) -> i32 {
        x + 1
    }




}
