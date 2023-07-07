use std::io;
fn main() {

    let number: i32 = loop {
        println!("Enter a number: ");

        let mut inp: String = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("cant read line");

        match inp.trim().parse() {
            Ok(num) => break num,
            Err(_)  => continue,
        };
    };

    let answer = f_to_c(number);
    println!("{number} in celcius is {answer}");
}

pub fn f_to_c(f: i32) -> i32 {
    (f - 32) * 5/9
}
