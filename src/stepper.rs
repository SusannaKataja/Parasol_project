use rpi_embedded::gpio::{Gpio, OutputPin};
use std::time::Duration;
use std::thread;

pub struct Stepper {
    pub dir: OutputPin,
    pub step: OutputPin,
    period: Duration,
    position: f32,
    step_size: f32,
}

impl Stepper{
    pub fn new()-> Self{
        let gpio_pins_to_be_set = (25, 24);
        let mut _out = Self{
            dir: Gpio::output(gpio_pins_to_be_set.0).unwrap(),
            step: Gpio::output(gpio_pins_to_be_set.1).unwrap(),
            period: Duration::from_millis(10),
            position: 0.0,
            step_size: 10.0,
        };
        _out
    }

    pub fn steps(&mut self, degrees:f32){
        let step_count = Stepper::deg_to_steps(degrees); // Converts degrees to steps
        for _x in 0..step_count+1 {
            self.step.set_high();
            thread::sleep(self.period);
            self.step.set_low();
        }
        if self.dir.is_set_high() {
            self.position = self.position + degrees;
        }
        else {
            self.position = self.position - degrees;
        }
        //println!("Current position is {} degrees",self.position);
    }

    fn choose_direction(&mut self, light_data1: f32, light_data2: f32, light_data3: f32, light_data4: f32){
        if light_data2 > light_data4 {
            if light_data1 > light_data3 + 100.0{
                self.dir.set_high(); // counter-clockwise
            }
            else if light_data3 > light_data1 + 100.0{
                self.dir.set_low(); // clockwise
            }
        }
        if light_data4 > light_data2 {
            if light_data1 > light_data3 + 100.0{
                self.dir.set_low(); // clockwise
            }
            else if light_data3 > light_data1 + 100.0{
                self.dir.set_high(); // counter-clockwise
            }
        }
    }

    fn deg_to_steps(degrees:f32) -> u32{
        (degrees/1.8) as u32
    }

    fn choose_step_size(&mut self, light_data2: f32, light_data4: f32) {
        let difference = (light_data2-light_data4).abs();
        if difference < 150.0 {
            self.step_size = 5.0;
        }
        if difference >= 150.0 && difference < 300.0 {
             self.step_size = 10.0;
        }
        if difference >= 300.0 && difference < 500.0 {
            self.step_size = 20.0;
        }
        if difference >= 500.0 {
            self.step_size = 30.0;
        }
    }

    pub fn change_state(&mut self, light_data1: f32, light_data2: f32, light_data3: f32, light_data4: f32){
        Stepper::choose_direction(self, light_data1, light_data2, light_data3, light_data4);
        Stepper::choose_step_size(self, light_data2, light_data4);
        if (light_data4-light_data2).abs() > 100.0{
            Stepper::steps(self, self.step_size);
            println!("Stepper is taking a step of {} degrees. Position: {} deg", self.step_size, self.position);
        }
    }
}