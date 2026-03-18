// First exercise of a long streak (I hope !).
// Just a more or less game. User enter a number in stdin,
// and the program tells him bigger or smaller till the right number
// First time with rust (not so bad)

use std::io::{self, Write};

fn main() -> io::Result<()> {

    let y: u64 = rand::random_range(0..=1000);
    loop {
        print!("Enter a number: ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let the_number: u64 = buffer.trim().parse().expect("not a number");
        if y < the_number {
            println!("Magic number is smaller!");
        } else if y > the_number {
            println!("Magic number is bigger!");
        } else {
            println!("Congratulations!");
            break;
        }
    }
    Ok(())
}
