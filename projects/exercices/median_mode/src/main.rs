use rand::Rng;
use std::collections::HashMap;

fn main() {
    //Median
    let vec_length = rand::thread_rng().gen_range(10..=20); //Random length
    let mut my_vector: Vec<u32> = Vec::new();
    let mut i: u8 = 0;

    while i != vec_length {
        my_vector.push(rand::thread_rng().gen_range(1..=100)); //Push random numbers
        i += 1;
    }

    println!("{:?}", my_vector);

    //Passing reference as argument so the function does not take ownership and my_vector remain valid after function call
    fn get_median(vec: &Vec<u32>) -> u32 {
        let mut sorted_vec = vec.clone(); // Make deep clone of Vector in order to modify it without changing the original
        sorted_vec.sort();
        println!("Sorting: {:?}", sorted_vec);
        let count = sorted_vec.len() - 1;
        if count % 2 == 0 {
            let median = sorted_vec.get(count / 2).unwrap();
            println!("Median is: {:?}", median);
            *median
        } else {
            let floor = count / 2;
            let ceiling = floor + 1;
            let median = (sorted_vec[floor] + sorted_vec[ceiling]) / 2;
            println!("Median is: {:?}", median);
            median.try_into().unwrap()
        }
    }

    get_median(&my_vector);

    //Mode (value that occurs most often)
    fn get_mode(vec: &Vec<u32>) -> u32 {
        let mut map = HashMap::new();

        // Iterate over the vector then increasing value in hashmap by shadowing each iteration
        for value in vec {
            let count = map.entry(value).or_insert(0); //Shadowing
            *count += 1;
        }

        println!("{:?}", map.iter().max_by_key(|&(_, value)| value).unwrap());

        //A hashmap is a key-value pair, declaring in line variables and look for max_value
        let (max_key, max_value) = map.iter().max_by_key(|&(_, value)| value).unwrap();
        if max_value > &1 {
            println!("The mode is: {}", max_key);
            **max_key
        } else {
            println!("There is no mode");
            0
        }
    }

    get_mode(&my_vector);
}
