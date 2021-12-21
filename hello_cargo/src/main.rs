use std::io;
use rand::Rng;
// Rng is a trait that specifies which particular rand num generator we're using
// traits will be covered in more detail in chapter 10
use std::cmp::Ordering;


// rand is not included in the rust standard library, but there is a crate for it


fn main() {
    println!("Guess the number!");
    // inlcusive of the lower bound but exclusive of the upper bound
    // let keyword is used to bind a value to a variable
    let secret_number = rand::thread_rng().gen_range(1..101);


    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        //mutable
        // :: means an associated function, which will be implemented by a type
        // is an associated function similar to an interface?
        let mut guess = String::new();

        io::stdin()
            // pass in a mutable reference to the new string we created
            // our in will be appended to the new string
            .read_line(&mut guess)
            // returns io::Result enums of either Ok or Err
            // if we dont add the .expect() then we will get an unused Result warning
            // program will still compile however
            .expect("Failed to read line");

        // example of shadowing in Rust
        // trim on a string will remove white-space
        // trim to result on a number will also remove the (\r)\n added due to human input
        // we do a match here to handle exceptions more explicitly (versus an .expect)
        // in this implementation, we can keep looping
        let guess: u32 = match guess.trim().parse() {
            // these are the *arms* of the match statement
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}
