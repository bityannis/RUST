use std::collections::HashMap;

//Hashmap are key value pair stored on the heap. Like Vectors, HashMap are homogenous, all key must have same value as each other.

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Player 1"), 50);
    scores.insert(String::from("Player 2"), 70);

    println!("{:?}", scores);

    let player_name = String::from("Player 1");

    println!("{:?}", scores.get(&player_name).copied().unwrap_or(0));
    println!("{:?}", scores.get(&player_name).unwrap_or(&0));

    for (key, value) in scores {
        println!("{key}: {value}")
    }

    //Hash map and Ownership
    //For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values, as demonstrated:

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    //Move value of field_name and field_value
    map.insert(field_name, field_value);
    println!("Map is {:?}", map);
    // println!("{} {}", field_name, field_value); // ❌ field_name and field_value are not valid anymore because they moved during insertion

    //Avoid this with reference to keep field_name and field_value active after insertion
    // map.insert(&field_name, &field_value);

    //Overwritting value. Same key name get overwriten
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    //Adding a key value pair only if a key is not present
    scores.entry(String::from("Red")).or_insert(50); //Red does not exist, insert key-value red 50
    scores.entry(String::from("Blue")).or_insert(50); //Blue exist already, do nothing

    println!("{:?}", scores);
}
