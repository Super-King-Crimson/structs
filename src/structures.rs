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
    //Oh btw the name convention for structs is CamelCase
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

    println!("You are at the origin.\n How many steps right would you like to take?");
    io::stdin().read_line(&mut input).expect("Expected to read input");
    println!("{input}");
    *hunt_x += input.trim().parse::<i64>().expect("Expected a number input");

    input.clear();

    println!("How many steps forward would you like to take?");
    io::stdin().read_line(&mut input).expect("Expected to read input");
    println!("{input}");
    *hunt_y += input.trim().parse::<i64>().expect("Expected a number input");

    println!("You are now at ({}, {})", treasure_hunt.x, treasure_hunt.y);

    //That's it!
}


/*
 *
 * END
 * OF
 * SECTION
 * 
*/


//Now, let's make an example program with structs
pub fn explain() {
    //But to know something's value we have to do things without it, so...
    calculate_area_of_rectangle();
}


struct Rectangle {
    length: u32,
    width: u32
}

fn calculate_area_of_rectangle() {
    let l = 10;
    let w = 20;
    println!("The area is {} square pixels", area_1(l, w));

    let lw = (10, 20);
    println!("The area is {} square pixels", area_2(lw));

    let rect = Rectangle { length: 10, width: 20 };
    println!("The area is {} square pixels", area_3(&rect));

    derived_traits();
}


fn area_1(length: u32, width: u32) -> u32 {
    //ok, this is alright, but at face-value this function just multiplies two unrelated numbers together
    //While it accepts two arguments, there's no indication that they're related
    length * width
}


fn area_2 (dimensions: (u32, u32)) -> u32 {
    //Ok, good! Now we know that the two numbers are related.
    //But since tuples don't have names, we don't know how they're related: which is width, which is height?
    dimensions.0 * dimensions.1
}


//reference so we the function doesn't consume ownership!
fn area_3(rectangle: &Rectangle) -> u32 {
    //perfect! We're returning the area, calculated from the triangle's length and width
    rectangle.length * rectangle.width
}


struct Triangle {
    base: u32,
    height: u32
}

fn derived_traits() {  
    //Obviously, we can't print a triangle. But what if we could? Let's listen to the compiler:
    let tri = Triangle { base: 15, height: 50 };

    // println!("{tri}");
    // = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
    // = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
 
    //Alright, let's use Debug output format by using {:?}
    // println!("{:?}", tri);
    // note: add `#[derive(Debug)]` to `Triangle` or manually `impl Debug for Triangle`


    //Okay then, let's add the #[derive(Debug)] thing to a new struct
    #[derive(Debug)]
    struct DebugTriangle {
        base: u32, 
        height: u32 
    }

    let debug_tri = DebugTriangle { base: 15, height: 50 };
    //WOAH!
    println!("{:?}", debug_tri);
    //Use :#? to make it *pretty*
    println!("Here are the details, but prettier: {:#?}", debug_tri);
    //We can also use the dbg! macro, which takes ownership of an expression,
    //prints it with the line number and the evaluation of that expression, and returns ownership of that evaluation
    let debug_tri_ = dbg!(DebugTriangle { base: 30, ..debug_tri });

    // By default, the curly brackets tell println! to use formatting known as Display: output 
    // intended for direct end user consumption. 
    // The primitive types we’ve seen so far implement Display by default, 
    // because there’s only one way you’d want to show a 1 or any other primitive type to a user. 
    // But with structs, the way println! should format the output is less clear.

    //Also dbg! prints to smthn called stderr instead of stdout (we'll talk about that later)


    //We can also use the derive keyword to add custom behavior to structs (we'll talk about that later)
}