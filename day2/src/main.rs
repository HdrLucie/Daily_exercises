use std::io::Write;
use std::io;

fn main() -> io::Result<()> {
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    // We use trim to remove \n
    for c in buffer.trim().chars() {
        print!("{}", ((c as u8 - b'a' + 13) % 26 + b'a') as char);
    }
    Ok(())
}
