use super::sys;

use std::{ffi::CString, time::Duration};

#[derive(Debug, Clone, Copy)]
pub struct Device {
    pub tag: sys::WbDeviceTag,
}

impl Device {
    pub fn new(name: &str) -> Self {
        let name = CString::new(name).unwrap();
        let tag = unsafe { sys::wb_robot_get_device(name.as_ptr()) };

        Self { tag }
    }
}

pub trait Robot {
    /// The time to advance during the next simulation step.
    fn time_step(&self) -> Duration {
        Duration::from_millis(unsafe { sys::wb_robot_get_basic_time_step().round() as u64 })
    }

    /// Initialize the robot.
    fn init() -> Self;

    /// Step the robot simulation
    fn step(&mut self);

    /// Run the robot controller.
    fn run(&mut self) {}
}
