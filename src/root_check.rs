use std::error::Error;
use std::process;

pub fn main() -> Result<(), Box<dyn Error>> {
    match sudo::escalate_if_needed() {
        Ok(data) => {
            println!("Authenticated as {:?}", data);
        }
        Err(_) => {
            println!("This application requires root access, it will now terminate.");
            process::exit(0x0100);
        }
    }
    Ok(())
}
