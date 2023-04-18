use std::collections::HashMap;
use std::io;

fn main() {
    //Initiate Vector of HashMap String
    let mut company: Vec<HashMap<String, String>> = Vec::new();

    loop {
        let mut choice = String::new();
        let mut name = String::new();
        let mut department = String::new();
        let mut entry = HashMap::new();

        println!("Type 'get' to retreive all employee or '+' to add new one");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        //Check user input to select appropriate action. trim() to remove '\n' on string
        match choice.trim() {
            //Add employee
            "+" => {
                println!("Add employee name");

                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");

                println!("Add employee department");

                io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read line");

                entry.insert(name.trim().to_string(), department.trim().to_string());
                company.push(entry);
            }
            //Get all employee by alphabetical order
            "get" => {
                company.sort_by(|a, b| {
                    let key_a = a.keys().next().unwrap();
                    let key_b = b.keys().next().unwrap();
                    key_a.cmp(key_b)
                });

                println!("{:?}", company);
                break;
            }
            _ => {
                println!("Type + or get only");
            }
        }
    }
}
