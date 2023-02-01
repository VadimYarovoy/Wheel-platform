use color_eyre::Result;
use rppal::gpio::OutputPin;
use rppal::pwm::Pwm;

pub struct Engine {
    pub forward_pin: OutputPin,
    pub backward_pin: OutputPin,
    pub pwm: Pwm,
}

impl Engine {
    pub fn set(&mut self, direction: f64) -> Result<()> {
        if !self.pwm.is_enabled()? {
            
            self.pwm.set_frequency(20.0, 0.0)?;
self.pwm.enable()?;
        }
        let direction = direction.clamp(-1.0, 1.0);
        if direction >= 0.0 {
            self.forward_pin.set_high();
            self.backward_pin.set_low();
        } else {
            self.forward_pin.set_low();
            self.backward_pin.set_high();
        }
        self.pwm.set_duty_cycle(direction.abs())?;
        Ok(())
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.forward_pin.set_low();
        self.backward_pin.set_low();
    }
}

pub struct AnalogDirectionController {
    pub left: Engine,
    pub right: Engine,
}

impl AnalogDirectionController {
    pub fn direction(&mut self, gradient: f64, radial: f64) -> Result<()> {
        let gradient = gradient.clamp(-1.0, 1.0);
        let radial = radial.clamp(-1.0, 1.0);
        Ok(())
    }
}

#[derive(Debug)]
pub enum Direction {
    Stop,
    Forward,
    Backward,
    Left,
    Right
}

pub struct DigitalDirectionController {
    pub left: Engine,
    pub right: Engine,
}

impl DigitalDirectionController {
    pub fn direction(&mut self, direction: Direction) -> Result<()> {
        match direction {
            Direction::Stop => {
                self.left.set(0.0)?;
                self.right.set(0.0)
            },
            Direction::Forward => {
                self.left.set(1.0)?;
                self.right.set(1.0)
            },
            Direction::Backward => {
                self.left.set(-1.0)?;
                self.right.set(-1.0)
            },
            Direction::Left => {
                self.left.set(-1.0)?;
                self.right.set(1.0)
            },
            Direction::Right => {
                self.left.set(1.0)?;
                self.right.set(-1.0)
            },
        }
    }
}
