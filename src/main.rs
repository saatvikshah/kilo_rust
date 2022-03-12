use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    for line in io::stdin().lock().lines() {
        let line = line?;
        if line.trim() == "q" {
            return Ok(());
        }
    }
    Ok(())
}
