fn main() {
    //Buffer overflow with cargo run --release : u8 => 255 + 1 = 0
    let mut x: u8 = 255; //max value
    x += 1;
    println!("{}", x);

    //Shadowing -> Redeclaration of variable to change type or content
    let _my_int: u8 = 100;
    let _my_int: char = 'a';
    println!("{}", _my_int);

    //A tuple is a general way of grouping together a number of values with a variety of types into one compound type
    let tup: (u8, bool, char, &str) = (42, true, 'a', "42");
    println!("{}", tup.3);

    //Array must have the same type. They have fixed length. We have to use vector if we want dynamic-sized container.
    let my_array = ['a', 'b', 'c'];
    let _my_typed_array: [u8; 5] = [1, 2, 3, 4, 5];
    let _my_other_array = [3; 5]; //[3, 3, 3, 3, 3];
    println!("{}", my_array[0]);

    //Vectors allow to store more than one value in a single data structure. Vectors can only store values of the same type.
    let mut _my_vector: Vec<u8> = Vec::new(); //Initialization with the type of value u8
    let mut my_second_vector = vec![1, 2, 3]; //Initialization with values (no need for type annotation)

    my_second_vector.push(4);
    println!("{:?}", my_second_vector);
    println!("{}", &my_second_vector[3]);
    let third_element: &u8 = &my_second_vector[2]; //Use of the address
    println!("{}", third_element);
    let third_element: Option<&u8> = my_second_vector.get(2); //Shadowing to use get method
    println!("{:?}", third_element);
    match third_element {
        Some(third_element) => println!("{}", third_element),
        None => println!("Invalid index"),
    }

    // let does_not_exist = &my_second_vector[10]; //This will cause panick.
    let does_not_exist = my_second_vector.get(10); //This will return None.
    println!("{:?}", does_not_exist);

    for i in &my_second_vector {
        println!("{}", i);
    }

    //Alterate element in loop
    for i in &mut my_second_vector {
        *i += 10;
        println!("{}", i);
    }
}
