fn main() {
    println!("Hello, world!");

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64
    }

    let mut user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123")
        active: true,
        sign_in_count: 1,
    }

    user1.email = String::from("anotheremail@example.com");
    // Rust doesn't allow us to mark only certain fields as mutable

    fn build_user(email: String, username: String) -> User {
        User{
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }

        // Using the Field Init Shorthand
        User{
            email, // Because email has the same name field and parameter has the same name,
                    // We only need to write email once
            username,
            active: true,
            sign_in_count: 1,
        }

        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }
        
        fn main() {
            // --snip--
        
            let user1 = User {
                email: String::from("someone@example.com"),
                username: String::from("someusername123"),
                active: true,
                sign_in_count: 1,
            };
        
            let user2 = User {
                email: String::from("another@example.com"),
                ..user1 // This line copies the fields of user1 to be in user2 fields
                        // This though makes user1 invalid, because we copied the contents of a field to user2
            };
        }
        
    }
}
