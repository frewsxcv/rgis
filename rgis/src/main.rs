use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    rgis::run();
    Ok(())
}
