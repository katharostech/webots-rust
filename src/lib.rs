/// Raw, auto-generated bindings to Webots libcontroller C library.
pub mod sys {
    #![allow(clippy::approx_constant)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!("bindings.rs");
}

pub mod prelude {
    pub use super::{motor::*, robot::*, sensors::*, JointType};
}

/// Initialize webot controller. Must be called before any other functions or devices, etc. can be
/// called or created.
pub fn init() {
    unsafe {
        sys::wb_robot_init();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum JointType {
    Linear,
    Rotational,
}

impl JointType {
    pub(crate) fn from_ffi(value: sys::WbJointType) -> Self {
        match value {
            sys::WbJointType_WB_LINEAR => JointType::Linear,
            sys::WbJointType_WB_ROTATIONAL => JointType::Rotational,
            _ => unreachable!(),
        }
    }
}

pub mod sensors {
    use std::{convert::TryInto, time::Duration};

    use super::{prelude::*, sys};

    pub use distance_sensor::*;
    mod distance_sensor;
}
pub mod motor;

pub mod robot {
    use super::sys;

    use std::{convert::TryInto, ffi::CString, time::Duration};

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
        fn time_step(&self) -> Duration;

        /// Step the robot simulation
        fn step(&mut self);

        /// Run the robot controller.
        fn run(&mut self) {
            loop {
                let step_duration = self
                    .time_step()
                    .as_millis()
                    .try_into()
                    .expect("Duration too long");

                self.step();

                if unsafe { sys::wb_robot_step(step_duration) } == -1 {
                    break;
                }
            }

            unsafe {
                sys::wb_robot_cleanup();
            }
        }
    }
}
