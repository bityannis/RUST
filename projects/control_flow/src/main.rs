fn main() {
    let number: u8 = 42;
    let motto: &str = "Born to code";

    //If-Else No need for parenthesis
    if number == 42 && motto == "Born to code" {
        println!("Best coding school on 🌍");
    } else {
        println!("Still ok 👍");
    }

    //Variable with expression value
    let condition: bool = true;
    let x: u8 = if condition { 42 } else { 21 };
    println!("{}", x);

    //LOOPS => while / loop / for
    let mut counter: u8 = 10;

    //Decrement counter
    while counter != 0 {
        println!("{}", counter);
        counter -= 1
    }
    println!("💥");

    //Assign return value to result
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{:?}", result);

    //For loop
    let my_array = [1, 2, 3, 4, 5];

    for i in my_array {
        println!("{}", i);
    }

    //.rev() for reverse
    for i in (1..5).rev() {
        println!("{}", i);
    }
    println!("💥");
}
