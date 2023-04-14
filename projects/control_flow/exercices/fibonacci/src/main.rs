fn main() {
    let mut fib_vector: Vec<i64> = vec![1, 1];
    let mut i = 1;

    //Loop until overflow max i64 value std::i64::MAX == 7540113804746346429
    loop {
        println!("{:?}", fib_vector);
        fib_vector.push(fib_vector[i] + fib_vector[i - 1]);
        i += 1;
    }
}
