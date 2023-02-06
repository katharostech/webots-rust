use super::sys;

use std::{
    ffi::{CStr, CString},
    time::Duration,
};

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

pub struct RobotInfo;

impl RobotInfo {
    /// Get the URDF XML string for the robot.
    pub fn urdf_xml() -> String {
        let prefix = CString::default();
        let ptr = unsafe { sys::wb_robot_get_urdf(prefix.as_ptr()) };
        unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_string()
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
    fn step(&mut self, time: Duration, delta_time: Duration);

    /// Run the robot controller.
    fn run(&mut self) {}
}
