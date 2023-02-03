use std::{convert::TryInto, time::Duration};

use super::{prelude::*, sys};

pub struct Motor {
    pub device: Device,
}

impl Motor {
    pub fn new(name: &str) -> Self {
        Self {
            device: Device::new(name),
        }
    }

    /// Set the position, specified in radians or meters depending on the [`JointType`].
    pub fn set_position(&self, position: f64) {
        unsafe {
            sys::wb_motor_set_position(self.device.tag, position);
        }
    }
    /// Set the velocity, specified in radians/second or meters/second depending on the [`JointType`].
    pub fn set_velocity(&self, velocity: f64) {
        unsafe {
            sys::wb_motor_set_velocity(self.device.tag, velocity);
        }
    }
    /// Set the acceleration, specified in radians/second/second or meters/second/second
    /// depending on the [`JointType`].
    pub fn set_acceleration(&self, acceleration: f64) {
        unsafe {
            sys::wb_motor_set_acceleration(self.device.tag, acceleration);
        }
    }
    /// Set the force, specified in newtons.
    pub fn set_force(&self, force: f64) {
        unsafe {
            sys::wb_motor_set_force(self.device.tag, force);
        }
    }
    pub fn set_availble_force(&self, force: f64) {
        unsafe {
            sys::wb_motor_set_available_force(self.device.tag, force);
        }
    }
    /// Set the torque, specified in newton meters.
    pub fn set_torque(&self, force: f64) {
        unsafe {
            sys::wb_motor_set_torque(self.device.tag, force);
        }
    }
    pub fn set_available_torque(&self, force: f64) {
        unsafe {
            sys::wb_motor_set_available_torque(self.device.tag, force);
        }
    }
    pub fn set_control_pid(&self, p: f64, i: f64, d: f64) {
        unsafe {
            sys::wb_motor_set_control_pid(self.device.tag, p, i, d);
        }
    }

    pub fn enable_force_feedback(&self, sampling_period: Duration) {
        unsafe {
            sys::wb_motor_enable_force_feedback(
                self.device.tag,
                sampling_period
                    .as_millis()
                    .try_into()
                    .expect("Duration too big"),
            );
        }
    }
    pub fn disable_force_feedback(&self) {
        unsafe {
            sys::wb_motor_disable_force_feedback(self.device.tag);
        }
    }
    pub fn force_feedback_sampling_period(&self) -> Duration {
        Duration::from_millis(unsafe {
            sys::wb_motor_get_force_feedback_sampling_period(self.device.tag) as _
        })
    }
    pub fn force_feedback(&self) -> f64 {
        unsafe { sys::wb_motor_get_force_feedback(self.device.tag) }
    }

    pub fn enable_torque_feedback(&self, sampling_period: Duration) {
        unsafe {
            sys::wb_motor_enable_torque_feedback(
                self.device.tag,
                sampling_period
                    .as_millis()
                    .try_into()
                    .expect("Duration too big"),
            );
        }
    }
    pub fn disable_torque_feedback(&self) {
        unsafe {
            sys::wb_motor_disable_torque_feedback(self.device.tag);
        }
    }
    pub fn torque_feedback_sampling_period(&self) -> Duration {
        Duration::from_millis(unsafe {
            sys::wb_motor_get_torque_feedback_sampling_period(self.device.tag) as _
        })
    }
    pub fn torque_feedback(&self) -> f64 {
        unsafe { sys::wb_motor_get_torque_feedback(self.device.tag) }
    }

    pub fn target_position(&self) -> f64 {
        unsafe { sys::wb_motor_get_target_position(self.device.tag) }
    }
    pub fn min_position(&self) -> f64 {
        unsafe { sys::wb_motor_get_min_position(self.device.tag) }
    }
    pub fn max_position(&self) -> f64 {
        unsafe { sys::wb_motor_get_max_position(self.device.tag) }
    }
    pub fn velocity(&self) -> f64 {
        unsafe { sys::wb_motor_get_velocity(self.device.tag) }
    }
    pub fn max_velocity(&self) -> f64 {
        unsafe { sys::wb_motor_get_max_velocity(self.device.tag) }
    }
    pub fn acceleration(&self) -> f64 {
        unsafe { sys::wb_motor_get_acceleration(self.device.tag) }
    }
    pub fn available_force(&self) -> f64 {
        unsafe { sys::wb_motor_get_available_force(self.device.tag) }
    }
    pub fn available_torque(&self) -> f64 {
        unsafe { sys::wb_motor_get_available_torque(self.device.tag) }
    }
    pub fn max_torque(&self) -> f64 {
        unsafe { sys::wb_motor_get_max_torque(self.device.tag) }
    }
    pub fn multiplier(&self) -> f64 {
        unsafe { sys::wb_motor_get_multiplier(self.device.tag) }
    }

    pub fn joint_type(&self) -> JointType {
        JointType::from_ffi(unsafe { sys::wb_motor_get_type(self.device.tag) })
    }

    pub fn brake(&self) -> Brake {
        Brake {
            device: Device {
                tag: unsafe { sys::wb_motor_get_brake(self.device.tag) },
            },
        }
    }

    pub fn position_sensor(&self) -> PositionSensor {
        PositionSensor {
            device: Device {
                tag: unsafe { sys::wb_motor_get_brake(self.device.tag) },
            },
        }
    }
}

pub struct PositionSensor {
    pub device: Device,
}

impl PositionSensor {
    pub fn new(name: &str) -> Self {
        Self {
            device: Device::new(name),
        }
    }

    pub fn enable(&self, sampling_period: Duration) {
        unsafe {
            sys::wb_position_sensor_enable(
                self.device.tag,
                sampling_period
                    .as_millis()
                    .try_into()
                    .expect("Duration too big"),
            )
        }
    }

    pub fn disable(&self) {
        unsafe {
            sys::wb_position_sensor_disable(self.device.tag);
        }
    }

    pub fn sampling_period(&self) -> Duration {
        Duration::from_millis(unsafe {
            sys::wb_position_sensor_get_sampling_period(self.device.tag) as _
        })
    }

    pub fn value(&self) -> f64 {
        unsafe { sys::wb_position_sensor_get_value(self.device.tag) }
    }

    pub fn joint_type(&self) -> JointType {
        JointType::from_ffi(unsafe { sys::wb_position_sensor_get_type(self.device.tag) })
    }

    pub fn motor(&self) -> Motor {
        Motor {
            device: Device {
                tag: unsafe { sys::wb_position_sensor_get_motor(self.device.tag) },
            },
        }
    }

    pub fn brake(&self) -> Brake {
        Brake {
            device: Device {
                tag: unsafe { sys::wb_position_sensor_get_brake(self.device.tag) },
            },
        }
    }
}

pub struct Brake {
    pub device: Device,
}
