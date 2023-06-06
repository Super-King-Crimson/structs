#![allow(unused)]
#![allow(dead_code)]

use std::io;

//A struct is like an object - allows named data grouping
struct User {
    //The name and type of a specific piece of data is called a field
    //Also we use strings instead of &str because we want the struct to own its data (a slice is a reference)
    username: String,
    //You can technically use &str with lifetimes, but we dk what those are yet so aaaaa
    password: String,
    online: bool,      
    friend_number: u8,
}
//Also don't use a semicolon here

pub fn introduce() {
    //Similar to tuples, but instead of an index, you use a name
    let user_tuple: (String, String, bool, u8) = (
        String::from("SuperKC"), 
        String::from("some_passwordL0L"), 
        false, 
        42,
    );

    //To instantiate a struct, do this and assign all fields a value w/key: value pairs
    let mut user_struct = User {
        username: String::from("SuperKC"),
        password: String::from("some_passwordL0L"),
        online: true,
        friend_number: 0,
    };

    //To get a value from a struct, use dot notation
    println!("{} made a friend!", user_struct.username);
    user_struct.friend_number += 1;    
    //Also note that you can't mark specific fields as immutable/mutable, only the entire struct

    let some_guy = create_user(
        String::from("some_user"), 
        String::from("some_password")
    );

    explain_updates(some_guy);
}


fn create_user(username: String, password: String) -> User {
    // User {
    //     username: username,
    //     password: password,      TOO MUCH WRITING
    //     friend_number: 0,
    //     online: true
    // }

    //Init shorthand! If a field name has the same name as a local variable (in this case the args)
    //then you only have to write the name once!

    //Much better!
    User {
        username,
        password,
        friend_number: 0,
        online: true
    }

}


//A special syntax that allows you to easily create a new struct that changes some fields
fn explain_updates(some_user: User) {
    // let another_user = User {
    //     username: some_user.username,
    //     password: some_user.password,    TOOOOOO MUCH WRITING!
    //     friend_number: 0,
    //     online: some_user.online
    // };

    let another_user = User {
        friend_number: 0,
        ..some_user  //holy crap spread syntax
    };

    //Do note that now another_user has consumed ownership of some_user 
    //(this wouldn't happen if all types carried over implemented Copy)

    explain_tuple_structs();
}


struct Color(u8, u8, u8);
struct Point(i64, i64, i64);

fn explain_tuple_structs() {
    //Tuple structs are structs that look like tuples
    //Useful for when you want to differentiate two types of tuples, or when naming fields would be too long

    let red: Color = Color(255, 0, 0);
    let left_top_back: Point = Point(-20, 34, 123);

    println!("Left top back means x is negative ({}), y is positive ({}), and z is positive ({})", 
    left_top_back.0, 
    left_top_back.1,
    left_top_back.2);

    explain_unit_like_structs();
}


fn explain_unit_like_structs() {
    //IDK why but you can also describe field-less structs 
    //(they're called unit-like structs) because they are like a unit (remember the () tuple)

    //No curly braces, parentheses, nothing!
    struct AlwaysEqual;
    let idk = AlwaysEqual;

    //Useful for traits (we'll talk abt that later)
    explain_field_borrowing();
}


struct Point2D {
    x: i64,
    y: i64,
}

fn explain_field_borrowing() {
    let mut treasure_hunt = Point2D {x: 0, y: 0};

    //Borrow checker tracks ownership separately for each field (not like an array)
    //That means if you borrow from a specific field, you can still borrow from other fields
    let hunt_x = &mut treasure_hunt.x;
    let hunt_y = &mut treasure_hunt.y;

    let mut input = String::new();

    println!("How many steps right would you like to take?");
    io::stdin().read_line(&mut input).expect("Expected to read input");
    println!("{input}");
    *hunt_x += input.trim().parse::<i64>().expect("Expected a number input");

    input.clear();

    println!("How many steps forward would you like to take?");
    io::stdin().read_line(&mut input).expect("Expected to read input");
    println!("{input}");
    *hunt_y += input.trim().parse::<i64>().expect("Expected a number input");

    println!("You are now at ({}, {})", treasure_hunt.x, treasure_hunt.y);
}