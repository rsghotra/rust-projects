
#![allow(dead_code)]
use std::{collections::HashMap, fs::{self, read_to_string}};
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    pub fn new(&self) -> User {
        let user = User::new(&self);
        user
    }
}

#[derive(Debug)]
enum  Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum Shape {
    Rectangle(f32, f32),
    Circle(f32),
    Square(f32),
}

fn calculate_area(shape: Shape) -> f32 {
    let area = match shape {
        Shape::Circle(radius) =>std::f32::consts::PI * radius * radius,
        Shape::Square(side_length) => side_length * side_length,
        Shape::Rectangle(width, height) => width * height,
    };
    area
}

fn read_from_file_csv(file_path: String) -> Result<String, String> {
    let result = read_to_string(file_path);

    match result {
        Ok(data) => Ok(data),
        Err(err) => Err(String::from("Contents not found")),
    }
}

fn main() {
    println!("Hello, Rustacean!");

    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter_mut();

    for val in v1_iter {
        *val = *val + 1;
    }

    let res =  fs::read_to_string("example.txt");
    match res {
        Ok(content) => {
            println!("File content: {}", content);
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    let x: i8 = 123;
    
    let y: u8 = 141;

    let z: f32 = 1133.62;

    print!("x: {}, y: {}, z: {}", x, y, z); 

    let is_male = true;
    let is_above_18: bool = true;

    if is_male {
        println!("You are a male");
    } else {
        println!("You ar enot a male");
    }

    if is_male && is_above_18 {
        println!("You are a legal male");
    }

    for i in 0..10 {
        println!("{}", i);
    }

    //string
    let sentence = String::from("Hi I am Rupinder");
    let first_word = get_first_word(sentence);
    println!("First word is: {}", first_word);

    let mut my_string = String::from("hello");
    my_string = takes_ownership(my_string);
    println!("{}", my_string); //this line would cause a compile error because ownership has been moved. 

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 2,
    };
    print!("User 1 username: {:?}", user1.username);
    print!("User 1 email: {:?}", user1.email);
    print!("User 1 active: {:?}", user1.active);


    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    print!("User 1 username: {:?}", user1.username);

    let my_direction = Direction::North;
    move_around(my_direction);

    let circle = Shape::Circle(5.0);
    let square = Shape::Circle(4.0);
    let rectangle = Shape::Rectangle(3.0, 4.0);

    calculate_area(circle);
    calculate_area(square);
    calculate_area(rectangle);

    let mut users = HashMap::new();

    users.insert(String::from("raman"), 32);

    let first_user_age = users.get("raman");

    match first_user_age {
        Some(age) => println!("age is {}", age),
        None => println!("Users not found in the db"),
    }

}

fn move_around(direction: Direction) {
    print!("{:?}", direction);    
}

fn takes_ownership(some_string: String) -> String {
    print!("Ho Ho {}", some_string); //some_string now owns data
    return some_string;
}


fn get_first_word(sentence:String) -> String {
    let mut ans = String::from("");


    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }

    return ans;
}



