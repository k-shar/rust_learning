use std::io;

fn fib(x: i32) -> i32 {
    match x {
        0 => 0,
        1 => 1,
        _ => fib(x-1) + fib(x-2),
    }
} 

fn ask_for_fib() {
    let number: i32 = loop {
        println!("Enter a number: ");
        let mut inp: String = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Cant read line");
        match inp.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    let ans: i32 = fib(number);
    println!("{ans}");
}

pub fn main() {
    loop { ask_for_fib(); }
}