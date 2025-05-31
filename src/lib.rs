use rppal::gpio::{Gpio, OutputPin, InputPin, Level};
use std::{thread, time::Duration, time::Instant};

pub struct Sr04 {
    echo_pin: u8,
    trigger_pin: u8,
    distance_cm: Option<f32>,
}

impl Sr04 {
    pub fn new(echo_pin: u8, trigger_pin: u8) -> Self {
        Sr04 {
            echo_pin,
            trigger_pin,
            distance_cm: None,
        }
    }

    fn trigger_pulse(&self, trigger: &mut OutputPin) {
        // Ensure trigger is low
        trigger.set_low();
        thread::sleep(Duration::from_micros(2));
        // Send 10us high pulse
        trigger.set_high();
        thread::sleep(Duration::from_micros(10));
        trigger.set_low();
    }

    fn calculate_distance_cm(&self, echo: &InputPin) -> Option<f32> {
        // Wait for echo to go high
        let mut timeout = Instant::now();
        while echo.read() == Level::Low {
            if timeout.elapsed().as_millis() > 100 {
                return None; // Timeout waiting for echo high
            }
        }
        let start = Instant::now();

        // Wait for echo to go low
        timeout = Instant::now();
        while echo.read() == Level::High {
            if timeout.elapsed().as_millis() > 100 {
                return None; // Timeout waiting for echo low
            }
        }
        let duration = start.elapsed().as_micros() as f32;

        // Calculate distance in cm with sub-centimeter precision
        let distance = duration / 58.0;
        Some(distance)
    }

    pub fn read_distance(&mut self) -> Option<f32> {
        let gpio = Gpio::new().expect("Failed to access GPIO");
        let mut trigger: OutputPin = gpio.get(self.trigger_pin).expect("Failed to get trigger pin").into_output();
        let echo: InputPin = gpio.get(self.echo_pin).expect("Failed to get echo pin").into_input();

        self.trigger_pulse(&mut trigger);
        self.distance_cm = self.calculate_distance_cm(&echo);
        self.distance_cm
    }

    pub fn as_in(&self) -> Option<f32> {
        self.distance_cm.map(|cm| cm / 2.54)
    }
}

