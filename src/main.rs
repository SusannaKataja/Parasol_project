mod parasol;
mod myservo;
mod stepper;
// use std::thread;
// use std::time::Duration;

fn main() {
    let mut my_parasol = parasol::Parasol::new();
    let mut my_servo = myservo::MyServo::new(0);

    let mut stepper1 = stepper::Stepper::new();
    stepper1.dir.set_high(); // counter-clockwise

    loop{
        my_parasol.read_data();
        println!("-----------------------");
        my_parasol.print_data();
        my_servo.change_state(my_parasol.light_data.0, my_parasol.light_data.2);
        stepper1.change_state(my_parasol.light_data.0, my_parasol.light_data.1, my_parasol.light_data.2, my_parasol.light_data.3);
    }
}

