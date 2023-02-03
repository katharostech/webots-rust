pub use super::*;

pub enum DistanceSensorType {
    Generic,
    InfraRed,
    Sonar,
    Laser,
}

pub struct DistanceSensor {
    pub device: Device,
}

impl DistanceSensor {
    pub fn new(name: &str) -> Self {
        Self {
            device: Device::new(name),
        }
    }

    pub fn enable(&self, sampling_period: Duration) {
        unsafe {
            sys::wb_distance_sensor_enable(
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
            sys::wb_distance_sensor_disable(self.device.tag);
        }
    }

    pub fn sampling_period(&self) -> Duration {
        unsafe {
            Duration::from_millis(sys::wb_distance_sensor_get_sampling_period(self.device.tag) as _)
        }
    }

    pub fn sensor_type(&self) -> DistanceSensorType {
        unsafe {
            match sys::wb_distance_sensor_get_type(self.device.tag) {
                sys::WbDistanceSensorType_WB_DISTANCE_SENSOR_GENERIC => DistanceSensorType::Generic,
                sys::WbDistanceSensorType_WB_DISTANCE_SENSOR_INFRA_RED => {
                    DistanceSensorType::InfraRed
                }
                sys::WbDistanceSensorType_WB_DISTANCE_SENSOR_SONAR => DistanceSensorType::Sonar,
                sys::WbDistanceSensorType_WB_DISTANCE_SENSOR_LASER => DistanceSensorType::Laser,
                _ => unreachable!(),
            }
        }
    }

    pub fn value(&self) -> f64 {
        unsafe { sys::wb_distance_sensor_get_value(self.device.tag) }
    }

    pub fn min_value(&self) -> f64 {
        unsafe { sys::wb_distance_sensor_get_min_value(self.device.tag) }
    }
    pub fn max_value(&self) -> f64 {
        unsafe { sys::wb_distance_sensor_get_max_value(self.device.tag) }
    }
    pub fn arperture(&self) -> f64 {
        unsafe { sys::wb_distance_sensor_get_aperture(self.device.tag) }
    }

    // TODO: wb_distance_sensor_get_lookup_table_size
}
