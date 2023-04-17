mod model; //Look for model directory in same directory with mod.rs within
use model::user::{Rectangle, User}; //Import User and Rectangle struct into the current scope - path

fn main() {
    //Mutable instance. Every field are mutable. Not possible to make only certain field mutable.
    let mut user1 = User {
        name: String::from("John"),
        email: String::from("john@john.com"),
        is_active: true,
        sign_in_count: 42,
    };

    println!("{}", user1.sign_in_count);
    user1.sign_in_count += 1;
    println!("{}", user1.sign_in_count);

    //Function that return User
    fn build_user(name: String, email: String) -> User {
        User {
            name, //name and email are initiate with parameters function. Because they have the same name, we can use field init shorthand.
            email,
            is_active: true,
            sign_in_count: 1,
        }
    }

    let user2 = build_user(String::from("Michel"), String::from("michel@michel.com"));
    println!("{} {}", user2.name, user2.sign_in_count);

    //Create instance from other instances
    //user3 have the same characteristic of user2 except his email.
    let user3 = User {
        email: String::from("michel2@michel2.com"),
        ..user2 //struct update syntax. Get all other fields same as user2.
    };

    println!("{}", user3.email);
    // println!("{}", user2.name); ❌ user2.name is a String and were moved into user3. So user2.name is not valid anymore. This applies only to String. Others data type have Copy trait.
    println!("{}", user2.is_active); // ✅ bool -> remain active.

    //Example
    let mut my_rectangle = Rectangle {
        width: 30,
        height: 50,
        area_result: None, //Optional, initialization with None
    };

    my_rectangle.area(); //Calling method to alterate area_result

    println!("Returned area value: {}", my_rectangle.return_area());

    println!("Area_res {:?}", my_rectangle.area_result);
    println!(
        "Area_res unwrapped {}",
        my_rectangle.area_result.unwrap_or(0)
    ); //Remove the Some()

    //Two way of printing whole struct instance with #[derive(Debug)] in struct definition
    println!("whole my_rectangle is {:?}", my_rectangle);
    dbg!(&my_rectangle);

    //Comparing two rectangles
    let my_second_rectangle = Rectangle {
        width: 15,
        height: 25,
        area_result: None,
    };

    let my_third_rectangle = Rectangle {
        width: 40,
        height: 60,
        area_result: None,
    };

    println!(
        "My 1st rectangle is bigger than my 2nd: {}",
        my_rectangle.can_hold(&my_second_rectangle)
    ); //true
    println!(
        "My 1st rectangle is bigger than my 3rd: {}",
        my_rectangle.can_hold(&my_third_rectangle)
    ); //false
}
