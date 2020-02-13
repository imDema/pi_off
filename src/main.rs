use std::error::Error;
use std::process::Command;
use rppal::gpio::{Gpio, Trigger, Level};

const GPIO_SWITCH : u8 = 26; //Pin 37 BCM 26

fn main() -> Result<(), Box<dyn Error>> {
    // Setup pin
    let gpio = Gpio::new()?;
    let mut pin = gpio.get(GPIO_SWITCH)?.into_input();
    
    // Setup interrupt and wait
    pin.set_interrupt(Trigger::FallingEdge)?;
    let int = pin.poll_interrupt(true, None)?;

    // Handle FallingEdge event
    println!("Registered event: {:?}", &int);
    if let Some(Level::Low) = int {
        shutdown()?;
    }
    Ok(())
}

fn shutdown() -> Result<(), std::io::Error> {
    println!("Running `shutdown -h now`");
    Command::new("shutdown")
        .args(&["-h", "now"])
        .spawn()
        .map(|_|())
}