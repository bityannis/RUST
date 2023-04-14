fn main() {
    //Variable can take expression value
    let school_name: u8 = {
        let x: u8 = 41;
        x + 1 //No semicolon at the end
    };
    another_function(school_name, "Born to Code");

    let mut i: u8 = five();
    i = increment(i);
    println!("i is {}", i);
}

fn another_function(num: u8, sentence: &str) {
    println!("My school name is {} and its motto is {}", num, sentence);
}

//Return values
fn five() -> u8 {
    5
}

fn increment(x: u8) -> u8 {
    x + 1
}
