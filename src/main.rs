use std::io;
use colored::Colorize;
use rand::Rng;
fn main() {
    println!("{}","Welcome to the guessing game created by lordofwizard \nhimself.. hope you like it".yellow());
    println!();

    let rand_int : u16  = rand::thread_rng().gen_range(1..100);
    println!("{}",rand_int);
    loop{
        let mut guess : String = String::new();
        println!("Enter your no here sir \n{}","-> ".blue());
        io::stdin().read_line(&mut guess).expect("Something went wrong while reading the string.");
        let guess : u16 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&rand_int){
            std::cmp::Ordering::Equal => {
                println!("{}","You won bro".green());
                break;
            },
            std::cmp::Ordering::Less => println!("{}","Small like your di*k".red()),
            std::cmp::Ordering::Greater => println!("{}","Big no oops".red()),
        }
    }

}
