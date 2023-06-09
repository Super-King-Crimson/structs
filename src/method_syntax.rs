#![allow(unused, dead_code)]

use std::slice::RChunksExactMut;

//You know what a method is. Here's how you put one in a struct:
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    height: u8,
    hunger_level: u8
}

//impl ==> implement
//we implement the eat_food method for the Person struct like so
impl Person {
    //This is short for mut self: &mut Self, methods must have their first be param self with type Self
    //Don't have to define type because Self represents whatever type the impl is for
    fn eat_food(&mut self) {
        if self.hunger_level <= 90 {
            self.hunger_level += 10;
            println!("Fed him! New hunger level is {}", self.hunger_level);
        } else {
            println!("Already full!");
        }
    }
}

pub fn explain() {
    let mut goodie = Person {
        name: String::from("Mr. Goodman"),
        age: 255,
        height: 255,
        hunger_level: 80
    };

    loop {
        //We can call the  method with method syntax (.) (automatically passes the struct itself as the first arg)
        goodie.eat_food();
        
        if goodie.hunger_level >= 100 {
            //Or with :: notation, manually passing the struct
            Person::eat_food(&mut goodie);
            break;
        }

        //note that we don't have to manually reference the variable with . syntax, but we do with :: notation
    }

    //Method syntax is mostly used so we don't have to continuously write out our types

    //Methods can also have the same name as fields (rust differentiates w/the parentheses)
    impl Person {
        fn age(&self) -> u8 {
            self.age
        }
    }

    println!("{} and {} are the same thing.", goodie.age, goodie.age());
    methods_with_more_args();
}


impl Person {
    fn can_date(&self, other: &Person) -> bool {
        if other.age - self.age > 5 || other.height - self.height > 30 { false } else { true }
    } 
}
fn methods_with_more_args() {
    let me = Person {
        name: String::from("cool guy"),
        age: 17,
        height: 156,
        hunger_level: 99
    };

    let ur_mom = Person {
        name: String::from("hot momma"),
        age: 49,
        height: 160,
        hunger_level: 43
    };

    let can = me.can_date(&ur_mom);

    println!("I can{}do your mom, because we are{}compatible.",
        if can { " " } else { "'t " }, 
        if can { " " } else { "n't " } 
    );

    associated_functions()
}


//Associated functions are like Java static functions - they can't be called on a specific struct (ex: String::new)
//They're usually used to return a new instance of a struct (ex. String::new)
//That being said, new isn't a special word, and multiple constructors can be made with diff names:
impl Person {
    fn from_birth(name: String) -> Self {
        Self {
            age: 0,
            height: 50,
            hunger_level: 100,
            name,
        }
    }
}

impl Person {
    fn from(age: u8, height: u8, hunger_level: u8, name: String) -> Self {
        Self {
            age,
            height,
            hunger_level,
            name  
        }
    }
}
fn associated_functions() {
    //To call an associated function, you can only use :: syntax with the struct name
    let baby = Person::from_birth(String::from("Junior"));
    let adult = Person::from(32, 
        160, 
        96, 
        String::from("Tris \"32\"Deuce")
    );
    //This function is namespaced by the Person struct (we'll talk about that later)

    //Also it's possible to have multiple impl blocks, that'll be useful for generics (we'll talk about that later)

    methods_and_ownership();
}


impl Person {
    //accepts reference to self
    fn name(&self) -> &String {
        &self.name
    }

    //accepts mutable reference to self
    fn have_birthday(&mut self) {
        self.age += 1;
    }

    //takes ownership of self
    fn fight(self, other: Person) -> Person {
        if self.height > other.height { self } else { other }
    }
}
fn methods_and_ownership() {
    //Perms: RO
    let person1 = Person {age: 16, name: String::from("Donald"), height: 178, hunger_level: 100};
    //Perms: RWO
    let mut person2 = Person::from(19, 167, 53, String::from("Jorge"));

    //ok, has R perms
    println!("My name is {}.", person1.name()); 
    //ok, has R perms
    println!("And my name is {}.", person2.name()); 

    //ok, both have RO perms, and both have their ownership moved
    println!("In a fight between us, {} would win.",  person1.fight(person2).name);

    //not ok, requires W perms and you're not mutable plus you lost ownership
    // println!("It's my birthday! My new age is {}", { person1.have_birthday(); person1.age });

    //not ok, ownership consumed by the fight (the string owns heap data so its a double free)
    //println!("It's my birthday! My new age is {}.", { person2.have_birthday(); person2.age });
}