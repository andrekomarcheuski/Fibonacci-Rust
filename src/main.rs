fn fibonacci(n: i32) -> Vec<i32> {
    let mut sequence: Vec<i32> = vec![0, 1];
    if n > 2 {
        for _ in 0..n-2 {
            let new_elem = sequence[sequence.len() - 2] + sequence[sequence.len() - 1];
            sequence.push(new_elem);
        }
    } else {
        while sequence.len() != n.try_into().unwrap() {
            sequence.pop();
        }
    }
    sequence
}

fn print_fibo(sequence: Vec<i32>) {
    if !sequence.is_empty() {
        let last_elem = sequence[sequence.len() - 1];
        for i in sequence {
            if i != last_elem {
                print!("{} - ", i);
            } else {
                println!("{}", i);
            }
        }
    }
}

fn main() {
    let n = 18;
    let sequence = fibonacci(n);
    print_fibo(sequence);
}
