
use std::thread;
use std::time::Duration;
use color_eyre::Result;

use rppal::gpio::Gpio;
use rppal::pwm::{Pwm, Channel};
use rppal::system::DeviceInfo;

mod engine;
mod input;

fn main() -> Result<()> {
    println!("Working on a {}.", DeviceInfo::new()?.model());

    let mut engine_left = engine::Engine{
        forward_pin: Gpio::new()?.get(14)?.into_output(),
        backward_pin: Gpio::new()?.get(15)?.into_output(),
        pwm: Pwm::new(Channel::Pwm0)?,
    };

    // engine_left.set(1.0)?;
    // sleep(Duration::from_secs(5)).await;
    // Ok(())
    
    let mut engine_right = engine::Engine{
        forward_pin: Gpio::new()?.get(26)?.into_output(),
        backward_pin: Gpio::new()?.get(13)?.into_output(),
        pwm: Pwm::new(Channel::Pwm1)?,
    };
    let mut ddc = engine::DigitalDirectionController{
        left: engine_left,
        right: engine_right,
    };
    let mut input = input::ButtonInput{
        forward: Gpio::new()?.get(8)?.into_input(),
        backward: Gpio::new()?.get(1)?.into_input(),
        left: Gpio::new()?.get(25)?.into_input(),
        right: Gpio::new()?.get(7)?.into_input(),
    };

    loop {
        let direction = input.get_direction();
        println!("Direction: {:?}", direction);
        ddc.direction(direction)?;
        thread::sleep(Duration::from_millis(50));
    }
}