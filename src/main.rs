extern crate rppal;

use std::error::Error;
use std::process::Command;
use rppal::gpio::{Gpio, Trigger, Level};

const GPIO_SWITCH : u8 = 26; //Pin 37 BCM 26

fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;
    let mut pin = gpio.get(GPIO_SWITCH)?.into_input();
    pin.set_interrupt(Trigger::FallingEdge)?;
    
    let int = pin.poll_interrupt(true, None)?;
    println!("Registered event: {:?}", &int);
    if let Some(Level::Low) = int {
        println!("Starting shutdown!");
        Command::new("shutdown")
            .args(&["-h", "now"])
            .spawn()?;
    }
    Ok(())
}
