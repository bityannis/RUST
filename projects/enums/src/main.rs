//An enum is way of saying a value is one of a possible set of values.

//Example: IP addresses can be either version 4 or version 6.
//IpAddress is now a custom data type.
#[derive(Debug)] //To be able to print

//Two types
enum IpAddress {
    V6(String),
    V4(u8, u8, u8, u8),
}

//Method to clear V6 variant
impl IpAddress {
    fn hide_ip(&mut self) {
        //If instance is of type V6, we use ref mut keyword
        if let IpAddress::V6(ref mut ip_v6) = self {
            ip_v6.clear(); //ip_v6 is the String inside V6()
        } else {
            println!("Instance is not type V6")
        }
    }
}

//Other example
#[derive(Debug)]
enum UsState {
    Alaska,
    Michigan,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let home = IpAddress::V4(127, 0, 0, 1);
    let home_two = IpAddress::V6(String::from("ip_v6: 42"));
    let mut home_three = IpAddress::V6(String::from("ip_to_hide"));
    let mut home_four = IpAddress::V4(128, 2, 3, 4);

    println!("{:?}", home);
    println!("{:?}", home_two);
    println!("{:?}", home_three);
    println!("{:?}", home_four);

    //Extract String inside home enum. Match pattern need to cover every possible case of an enum.
    match home {
        IpAddress::V6(addr) => println!("{}", addr),
        //👆🏻 home is V4 so above line is ignored. We can also use _ patterns as seen below
        IpAddress::V4(a, b, c, d) => println!("{} {} {} {}", a, b, c, d),
    };

    //Assigning value to variable with match.
    let home_two_ip = match home_two {
        //home_two is V6 so home_two_ip get for value String inside home_two
        IpAddress::V6(addr) => addr,
        _ => String::from("Something else"),
    };

    println!("home_two_ip is: {}", home_two_ip);

    //home_three is of type V6, so we can call the method to clear field
    home_three.hide_ip();
    println!("home_three after hiding ip: {:?}", home_three);

    //home_four is of type V4, so it wont clear field
    home_four.hide_ip();
    println!("{:?}", home_four);

    //More match example with Coin
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("This quarter is from {:?}", state);
                25
            }
        }
    }

    let _my_penny = Coin::Penny;
    let _my_nickel = Coin::Nickel;
    let _my_dime = Coin::Dime;
    let my_quarter = Coin::Quarter(UsState::Michigan);
    let _my_second_quarter = Coin::Quarter(UsState::Alaska);
    println!("Value of my_quarter is {}", value_in_cents(my_quarter));
}
