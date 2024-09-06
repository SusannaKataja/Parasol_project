use rpi_embedded::uart::{Uart, Parity};
use std::vec::Vec;
use std::time::Duration;

pub struct Parasol{
    pub light_string: (String, String, String, String),
    pub light_data: (f32, f32, f32, f32),
}

impl Parasol{
    pub fn new()-> Self{
        Self{
            light_string: (String::new(), String::new(), String::new(), String::new()),
            light_data: (0.0, 0.0, 0.0, 0.0),
        }
    }
    pub fn read_data(&mut self){
        let mut uart = Uart::new(9600, Parity::None, 8, 1).unwrap();
        uart.set_read_mode(1, Duration::default()).unwrap();

        let data_string = uart.read_until('\n').unwrap(); //Reads the data in one line
        let data_vector: Vec<&str> = data_string.split(" ").collect();

        self.light_string = (data_vector[0].to_string(), data_vector[1].to_string(), data_vector[2].to_string(), data_vector[3].to_string());
        self.light_data = (self.light_string.0.parse::<f32>().unwrap(), self.light_string.1.parse::<f32>().unwrap(), self.light_string.2.parse::<f32>().unwrap(), self.light_string.3.parse::<f32>().unwrap());
    }
    pub fn print_data(&mut self){
        println!("Sensor 1: {}, Sensor 2: {}, Sensor 3: {}, Sensor 4: {}", self.light_data.0, self.light_data.1, self.light_data.2, self.light_data.3);
    }
}