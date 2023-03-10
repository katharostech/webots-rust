use std::time::Duration;

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

pub mod motor;
pub mod robot;
pub mod sensors;

/// Run a Webots robot controller.
pub fn run_robot<R: robot::Robot>() {
    use std::convert::TryInto;

    unsafe {
        sys::wb_robot_init();
    }

    let mut robot = R::init();
    let mut elapsed = Duration::default();
    loop {
        let delta = robot.time_step();
        let delta_millis = robot
            .time_step()
            .as_millis()
            .try_into()
            .expect("Duration too long");

        if unsafe { sys::wb_robot_step(delta_millis) } == -1 {
            break;
        }

        elapsed += delta;

        robot.step(robot::StepTime { elapsed, delta });
    }

    unsafe {
        sys::wb_robot_cleanup();
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
