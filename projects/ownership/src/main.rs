fn main() {
    //Pointer arithmetic, as saw in C, which is explicity unsafe in Rust
    let school: u8 = 42;
    let ptr_school = &school as *const u8; //Raw pointer

    println!("Memory address of school: {:?}", ptr_school);

    unsafe {
        println!("Content of ptr_school: {:?}", *ptr_school);
    }

    // Performing pointer arithmetic (unsafe and not recommended)
    let offset = 1; // Change this value to see different memory addresses
    let fake_ptr = (ptr_school as usize + offset * std::mem::size_of::<u8>()) as *const u8;

    println!("Memory address after pointer arithmetic: {:?}", fake_ptr);

    unsafe {
        println!(
            "Content of the memory address after pointer arithmetic: {}",
            *fake_ptr
        );
    }

    //String: Two types -> &str and String
    let my_str: &str = "Hello"; //&str are immutable string slice

    let i: usize = 0; //Index of string
    let character = my_str.chars().nth(i);

    match character {
        //Print string index
        Some(c) => println!("{}", c), //H
        None => println!("Wrong index"),
    }

    //Looping over string to print each character incremented by 1 (a -> b). Just printing because &str is not mutable directly.
    for c in my_str.chars() {
        let incremented_value = (c as u32) + 1;
        //If let to check if the character exist and extract the value inside Some
        if let Some(incremented_char) = char::from_u32(incremented_value) {
            println!("{}", incremented_char);
        } else {
            eprintln!(
                "The incremented value is not a valid Unicode scalar value: {}",
                incremented_value
            );
        }
    }

    //Reverse printing. &str is not mutable / just printing.
    fn reverse_printing(s: &str) {
        let mut i: usize = s.len();
        while i != 0 {
            //If let to extract the value inside Some
            if let Some(character) = s.chars().nth(i - 1) {
                println!("{}", character);
            }
            i -= 1;
        }
    }

    reverse_printing(my_str);

    //ASCII
    for code_point in 0x20..=0x7E {
        if let Some(c) = char::from_u32(code_point) {
            print!("{}", c);
        }
    }
    print!("{}", '\n');

    //Stack and Heap
    let mut s = String::from("Hello"); //String are mutable, stored on the Heap, growable and not null terminated.
    reverse_printing(&s);
    s.insert_str(5, ", ");
    s.push_str("le monde");
    s.push('!');
    println!("{}", s);

    //When String goes out of scope, Rust call drop function to deallocate variable from heap automatically. In C, this is done manually with free malloc

    //Move s into s2. s become invalid.
    let s2 = s;
    println!("{}", s2);
    // println!("{}", s); // ❌ s is not valid anymore

    /* 📝
     A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.
     When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to.

             s
    name     | value          index | value
    ptr      | ------------>     0  | h
    len      | 5         |       1  | e
    capacity | 5         |       2  | l
                         |       3  | l
                         |       4  | o
                         |
             s2          |
    name     | value     |
    ptr      | -----------
    len      | 5
    capacity | 5
     */

    //Variables an Data Interacting with Clone
    //If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.
    let s3 = String::from("hi");
    let s4 = s3.clone();

    println!("s3={} s4={}", s3, s4);

    let s5 = String::from("hey");

    fn takes_ownership(s: String) {
        println!("{}", s);
    }
    takes_ownership(s5);
    // println!("{}", s5); ❌ s5 were moved into takes_ownership and is no longer valid because droped after fn call

    let s6 = String::from("hey");

    //Reference and borrowing
    fn read_only(s: &String) {
        println!("{}", s);
    }

    read_only(&s6);
    println!("{}", s6); //s6 remain valid

    //Dereference -> Assigning pointer and checking *content
    let x = String::from("toto");
    let ptr_x = &x as *const String; //Raw pointer

    println!("Address of x is {:?}", ptr_x); //Address
    unsafe {
        println!("Content of ptr_x is {:?}", *ptr_x); // *content
        println!("{}", x == *ptr_x); //true
    }

    //Taking Ownership and droping afterward
    fn change_toto(mut s: String) {
        s.push('!');
        println!("{}", s);
    }
    change_toto(x); //toto!
                    // println!("{}", x); ❌ x were moved into change_toto and is no longer valid because droped after fn call

    //Mutable Reference
    let mut x2 = String::from("tata");

    fn change_tata(s: &mut String) {
        s.push('!');
    }

    change_tata(&mut x2);

    println!("{}", x2); //x2 remain valid

    //Immutable &str
    let x3: &str = "titi";

    //Again, alterate &str is not possible.

    // fn not_working_change_titi(s: &mut str) {
    //     s.push('!');
    // }

    // change_titi(x3);
    println!("{}", x3);

    //Data race -> Assigning multiple reference must happen in different scope

    let mut x4 = String::from("hello");

    {
        let x5 = &mut x4;
        println!("x5: {}", x5);
    }

    let x6 = &mut x4;
    println!("x6: {}", x6);

    //Slice types
    let mut sentence = String::from("Hello World");

    //Alterate sentence to the first word
    fn truncate_first_word(s: &mut String) {
        if let Some(space_index) = s.find(' ') {
            s.truncate(space_index);
        }
    }

    truncate_first_word(&mut sentence);
    println!("{}", sentence);

    let sentence_two = String::from("Bonjour le monde");

    //Returning first word of string argument into a new string
    fn return_first_word(s: &str) -> String {
        let mut first_word = String::new();
        for c in s.chars() {
            if c == ' ' {
                break;
            }
            first_word.push(c);
        }
        first_word // return statement without return keyword or ;
    }

    let bonjour = return_first_word(&sentence_two);
    println!("{}", bonjour);

    //Slicing and returning
    let sentence_three = String::from("Aurevoir les gens");

    fn slice_first_word(s: &String) -> &str {
        let space_index = s.find(' ').unwrap_or(s.len()); //unwrap_or is used in case sentence is only one word to prevent crash.
        &s[..space_index]
    }

    let aurevoir = slice_first_word(&sentence_three);
    println!("{}", aurevoir);

    println!("first aurevoir character is {}", &aurevoir[..1]);
}
