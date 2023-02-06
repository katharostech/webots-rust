#![allow(clippy::approx_constant)]

use webots::prelude::*;

const TIME_STEP: Duration = Duration::from_millis(64);
const MAX_SPEED: f64 = 6.28;

struct MyRobot {
    distance_sensors: Vec<DistanceSensor>,
    left_motor: Motor,
    right_motor: Motor,
}

impl Robot for MyRobot {
    fn time_step(&self) -> Duration {
        TIME_STEP
    }

    fn init() -> Self {
        let distance_sensor_names = vec!["ps0", "ps1", "ps2", "ps3", "ps4", "ps5", "ps6", "ps7"];
        let distance_sensors: Vec<DistanceSensor> = distance_sensor_names
            .iter()
            .map(|name| {
                let sensor = DistanceSensor::new(name);
                sensor.enable(TIME_STEP);
                sensor
            })
            .collect();

        let left_motor = Motor::new("left wheel motor");
        let right_motor = Motor::new("right wheel motor");

        left_motor.set_position(f64::INFINITY);
        right_motor.set_position(f64::INFINITY);
        left_motor.set_velocity(0.1 * MAX_SPEED);
        right_motor.set_velocity(0.1 * MAX_SPEED);

        Self {
            distance_sensors,
            left_motor,
            right_motor,
        }
    }

    fn step(&mut self, _time: Duration, _delta_time: Duration) {
        let distance_values = self
            .distance_sensors
            .iter()
            .map(|sensor| sensor.value())
            .collect::<Vec<_>>();

        // detect obsctacles
        let left_obstacle =
            distance_values[5] > 80.0 || distance_values[6] > 80.0 || distance_values[7] > 80.0;
        let right_obstacle =
            distance_values[0] > 80.0 || distance_values[1] > 80.0 || distance_values[2] > 80.0;

        // initialize motor speeds at 50% of MAX_SPEED.
        let mut left_speed = 0.5 * MAX_SPEED;
        let mut right_speed = 0.5 * MAX_SPEED;

        // modify speeds according to obstacles
        if left_obstacle {
            // turn right
            left_speed += 0.5 * MAX_SPEED;
            right_speed -= 0.5 * MAX_SPEED;
        } else if right_obstacle {
            // turn left
            left_speed -= 0.5 * MAX_SPEED;
            right_speed += 0.5 * MAX_SPEED;
        }

        // write actuators inputs
        self.left_motor.set_velocity(left_speed);
        self.right_motor.set_velocity(right_speed);
    }
}

fn main() {
    println!("Rust controller has started");
    webots::run_robot::<MyRobot>();
}
