use rpi_embedded::servo;

pub struct MyServo {
    pub servo: servo::Servo,
    pub pwm_pin: u8,
    pub new_dest: u8,
    pub old_dest: u8,
    step_size: u8,
}

impl MyServo {
    pub fn new(pwm_pin: u8) -> Self {
        let mut _out = Self {
            servo: servo::Servo::new(pwm_pin),
            pwm_pin: pwm_pin,
            new_dest: 0,
            old_dest: 0,
            step_size: 10,
        };
        _out.servo.write(90).expect("Setting servo to 90 deg failed");
        println!("Servo set to 90 deg -position");
        _out
    }
    pub fn change_state(&mut self, light_data1: f32, light_data3: f32) -> u8{
        MyServo::choose_step_size(self, light_data1, light_data3);
        if light_data1 > light_data3 + 50.0 && self.old_dest < 180{
            self.new_dest = self.old_dest + self.step_size;
            if self.new_dest > 180 {
                self.new_dest = 180;
            }
            self.servo.write(self.new_dest).expect("Setting servo failed");
            println!("Servo is taking a step of {} degrees. Position: {} deg", self.step_size, self.new_dest);
        }
        if light_data3 > light_data1 + 50.0  && self.old_dest > 0 {
            if self.old_dest > self.step_size {
                self.new_dest = self.old_dest - self.step_size;
            }
            else {
                self.new_dest = 0;
            }
            self.servo.write(self.new_dest).expect("Setting servo failed");
            println!("Servo is taking a step of {} degrees. Position: {} deg", self.step_size, self.new_dest);
        }

        self.old_dest = self.new_dest;
        self.old_dest
    }

    fn choose_step_size(&mut self, light_data1: f32, light_data3: f32) {
        let difference = (light_data1-light_data3).abs();
        if difference < 150.0 {
            self.step_size = 5;
        }
        if difference >= 150.0 && difference < 300.0 {
             self.step_size = 10;
        }
        if difference >= 300.0 && difference < 500.0 {
            self.step_size = 20;
        }
        if difference >= 500.0 {
            self.step_size = 30;
        }
    }
}