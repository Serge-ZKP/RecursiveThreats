use std::thread;

fn main() {
    let result = recursive_factorial(5);
    println!("Factorial of 5 is {}", result);
}

fn recursive_factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }

    let (tx, rx) = std::sync::mpsc::channel();

    let child = thread::spawn(move || {
        let child_result = recursive_factorial(n - 1);
        tx.send(child_result * n).unwrap();
    });

    let child_result = rx.recv().unwrap();
    child.join().unwrap();

    child_result
}
