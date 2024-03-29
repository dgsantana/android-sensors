use std::ptr::NonNull;

use android_sensors_sys::ffi::sensors as ffi;
use log::error;
use thiserror::Error;

use crate::sensor::{Sensor, SensorType, VALID_SENSOR_TYPE_INT};

#[derive(Debug)]
pub struct SensorEventQueue {
    ptr: NonNull<ffi::ASensorEventQueue>,
}

impl SensorEventQueue {
    pub fn from_ptr(ptr: NonNull<ffi::ASensorEventQueue>) -> Self {
        Self { ptr }
    }

    pub fn enable_sensor(&self, sensor: &Sensor) -> Result<(), SensorEventQueueError> {
        match unsafe {
            ffi::ASensorEventQueue_enableSensor(self.ptr.as_ptr(), sensor.ptr().as_ptr())
        } {
            0 => Ok(()),
            e => Err(SensorEventQueueError(e)),
        }
    }

    pub fn disable_sensor(&self, sensor: &Sensor) -> Result<(), SensorEventQueueError> {
        match unsafe {
            ffi::ASensorEventQueue_disableSensor(self.ptr.as_ptr(), sensor.ptr().as_ptr())
        } {
            0 => Ok(()),
            e => Err(SensorEventQueueError(e)),
        }
    }

    pub fn events(&self, number: usize) -> Vec<SensorEvent> {
        let mut event = create_empty_event();
        let mut events = Vec::new();
        unsafe {
            while ffi::ASensorEventQueue_getEvents(self.ptr.as_ptr(), &mut event, 1) > 0 {
                let Some(data) = SensorData::from_event(&event) else {
                    error!("Received unknown sensor data");
                    continue;
                };
                events.push(SensorEvent {
                    sensor: event.sensor,
                    timestamp: event.timestamp,
                    version: event.version,
                    type_: event.type_,
                    data
                });
                if events.len() >= number {
                    break;
                }
            }
        }
        events
    }

    pub fn all_events(&self) -> Vec<SensorEvent> {
        let mut event = create_empty_event();
        let mut events = Vec::new();
        unsafe {
            while ffi::ASensorEventQueue_getEvents(self.ptr.as_ptr(), &mut event, 1) > 0 {
                let Some(data) = SensorData::from_event(&event) else {
                    error!("Received unknown sensor data");
                    continue;
                };
                events.push(SensorEvent {
                    sensor: event.sensor,
                    timestamp: event.timestamp,
                    version: event.version,
                    type_: event.type_,
                    data
                });
            }
        }
        events
    }
}

fn create_empty_event() -> ffi::ASensorEvent {
    ffi::ASensorEvent {
        sensor: 0,
        timestamp: 0,
        version: 0,
        type_: 0,
        reserved0: 0,
        __bindgen_anon_1: ffi::ASensorEvent__bindgen_ty_1 {
            __bindgen_anon_1: ffi::ASensorEvent__bindgen_ty_1__bindgen_ty_1 { data: [0.0; 16] },
        },
        flags: 0,
        reserved1: [0; 3],
    }
}

#[derive(Debug)]
pub struct SensorEvent {
    sensor: i32,
    timestamp: i64,
    version: i32,
    type_: i32,
    data: SensorData,
}

impl SensorEvent {
    pub fn sensor(&self) -> i32 {
        self.sensor
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn version(&self) -> i32 {
        self.version
    }

    pub fn type_i32(&self) -> i32 {
        self.type_
    }

    pub fn sensor_type(&self) -> SensorType {
        if !VALID_SENSOR_TYPE_INT.contains(&self.type_) {
            return SensorType::Unknown(self.type_);
        }
        let type_: ffi::ASensorType = unsafe { std::mem::transmute(self.type_) };
        match type_ {
            ffi::ASENSOR_TYPE_ACCELEROMETER => SensorType::Accelerometer,
            ffi::ASENSOR_TYPE_MAGNETIC_FIELD => SensorType::MagneticField,
            ffi::ASENSOR_TYPE_GYROSCOPE => SensorType::Gyroscope,
            ffi::ASENSOR_TYPE_LIGHT => SensorType::Light,
            ffi::ASENSOR_TYPE_PROXIMITY => SensorType::Proximity,
            ffi::ASENSOR_TYPE_GRAVITY => SensorType::Gravity,
            ffi::ASENSOR_TYPE_LINEAR_ACCELERATION => SensorType::LinearAcceleration,
            ffi::ASENSOR_TYPE_ROTATION_VECTOR => SensorType::RotationVector,
            ffi::ASENSOR_TYPE_RELATIVE_HUMIDITY => SensorType::RelativeHumidity,
            ffi::ASENSOR_TYPE_AMBIENT_TEMPERATURE => SensorType::AmbientTemperature,
            ffi::ASENSOR_TYPE_MAGNETIC_FIELD_UNCALIBRATED => SensorType::MagneticFieldUncalibrated,
            ffi::ASENSOR_TYPE_GAME_ROTATION_VECTOR => SensorType::GameRotationVector,
            ffi::ASENSOR_TYPE_GYROSCOPE_UNCALIBRATED => SensorType::GyroscopeUncalibrated,
            ffi::ASENSOR_TYPE_SIGNIFICANT_MOTION => SensorType::SignificantMotion,
            _ => SensorType::Unknown(self.type_),
        }
    }

    pub fn data(&self) -> &SensorData {
        &self.data
    }

}

#[derive(Debug)]
pub enum SensorData {
    Accelerometer {
        x: f32,
        y: f32,
        z: f32,
    },
    MagneticField {
        x: f32,
        y: f32,
        z: f32,
    },
    Gyroscope {
        x: f32,
        y: f32,
        z: f32,
    },
    Light {
        value: f32,
    },
    Proximity {
        value: f32,
    },
    Gravity {
        x: f32,
        y: f32,
        z: f32,
    },
    LinearAcceleration {
        x: f32,
        y: f32,
        z: f32,
    },
    RotationVector {
        x: f32,
        y: f32,
        z: f32,
        scalar: f32,
    },
    RelativeHumidity {
        value: f32,
    },
    AmbientTemperature {
        value: f32,
    },
    MagneticFieldUncalibrated {
        x: f32,
        y: f32,
        z: f32,
        x_bias: f32,
        y_bias: f32,
        z_bias: f32,
    },
    GameRotationVector {
        x: f32,
        y: f32,
        z: f32,
        scalar: f32,
    },
    GyroscopeUncalibrated {
        x: f32,
        y: f32,
        z: f32,
        x_bias: f32,
        y_bias: f32,
        z_bias: f32,
    },
    SignificantMotion,
}

impl SensorData {
    /// Convert an `ASensorEvent` into a `SensorData` enum.
    ///
    /// # Panics
    ///
    /// Panics if the event type is unknown.
    /// This should never happen, as the event type is checked by the Android NDK before being passed
    /// to this function.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferences the `ASensorEvent` pointer.
    pub unsafe fn from_event(event: &ffi::ASensorEvent) -> Option<Self> {
        // Validate that the event type is known
        if !VALID_SENSOR_TYPE_INT.contains(&event.type_) {
            error!("Received event for unknown sensor type");
            return None;
        }

        let event_type: ffi::_bindgen_ty_4 = std::mem::transmute(event.type_);

        let event = match event_type {
            ffi::ASENSOR_TYPE_ACCELEROMETER => SensorData::Accelerometer {
                x: event.__bindgen_anon_1.__bindgen_anon_1.data[0],
                y: event.__bindgen_anon_1.__bindgen_anon_1.data[1],
                z: event.__bindgen_anon_1.__bindgen_anon_1.data[2],
            },
            ffi::ASENSOR_TYPE_MAGNETIC_FIELD => SensorData::MagneticField {
                x: event.__bindgen_anon_1.__bindgen_anon_1.data[0],
                y: event.__bindgen_anon_1.__bindgen_anon_1.data[1],
                z: event.__bindgen_anon_1.__bindgen_anon_1.data[2],
            },
            ffi::ASENSOR_TYPE_GYROSCOPE => SensorData::Gyroscope {
                x: event.__bindgen_anon_1.__bindgen_anon_1.data[0],
                y: event.__bindgen_anon_1.__bindgen_anon_1.data[1],
                z: event.__bindgen_anon_1.__bindgen_anon_1.data[2],
            },
            ffi::ASENSOR_TYPE_LIGHT => SensorData::Light {
                value: event.__bindgen_anon_1.__bindgen_anon_1.data[0],
            },
            ffi::ASENSOR_TYPE_PROXIMITY => SensorData::Proximity {
                value: event.__bindgen_anon_1.__bindgen_anon_1.data[0],
            },
            ffi::ASENSOR_TYPE_GRAVITY => SensorData::Gravity {
                x: event.__bindgen_anon_1.__bindgen_anon_1.data[0],
                y: event.__bindgen_anon_1.__bindgen_anon_1.data[1],
                z: event.__bindgen_anon_1.__bindgen_anon_1.data[2],
            },
            ffi::ASENSOR_TYPE_LINEAR_ACCELERATION => SensorData::LinearAcceleration {
                x: event.__bindgen_anon_1.__bindgen_anon_1.data[0],
                y: event.__bindgen_anon_1.__bindgen_anon_1.data[1],
                z: event.__bindgen_anon_1.__bindgen_anon_1.data[2],
            },
            ffi::ASENSOR_TYPE_ROTATION_VECTOR => SensorData::RotationVector {
                x: event.__bindgen_anon_1.__bindgen_anon_1.data[0],
                y: event.__bindgen_anon_1.__bindgen_anon_1.data[1],
                z: event.__bindgen_anon_1.__bindgen_anon_1.data[2],
                scalar: event.__bindgen_anon_1.__bindgen_anon_1.data[3],
            },
            ffi::ASENSOR_TYPE_RELATIVE_HUMIDITY => SensorData::RelativeHumidity {
                value: event.__bindgen_anon_1.__bindgen_anon_1.data[0],
            },
            ffi::ASENSOR_TYPE_AMBIENT_TEMPERATURE => SensorData::AmbientTemperature {
                value: event.__bindgen_anon_1.__bindgen_anon_1.data[0],
            },
            ffi::ASENSOR_TYPE_MAGNETIC_FIELD_UNCALIBRATED => {
                SensorData::MagneticFieldUncalibrated {
                    x: event.__bindgen_anon_1.__bindgen_anon_1.data[0],
                    y: event.__bindgen_anon_1.__bindgen_anon_1.data[1],
                    z: event.__bindgen_anon_1.__bindgen_anon_1.data[2],
                    x_bias: event.__bindgen_anon_1.__bindgen_anon_1.data[3],
                    y_bias: event.__bindgen_anon_1.__bindgen_anon_1.data[4],
                    z_bias: event.__bindgen_anon_1.__bindgen_anon_1.data[5],
                }
            }
            ffi::ASENSOR_TYPE_GAME_ROTATION_VECTOR => SensorData::GameRotationVector {
                x: event.__bindgen_anon_1.__bindgen_anon_1.data[0],
                y: event.__bindgen_anon_1.__bindgen_anon_1.data[1],
                z: event.__bindgen_anon_1.__bindgen_anon_1.data[2],
                scalar: event.__bindgen_anon_1.__bindgen_anon_1.data[3],
            },
            ffi::ASENSOR_TYPE_GYROSCOPE_UNCALIBRATED => SensorData::GyroscopeUncalibrated {
                x: event.__bindgen_anon_1.__bindgen_anon_1.data[0],
                y: event.__bindgen_anon_1.__bindgen_anon_1.data[1],
                z: event.__bindgen_anon_1.__bindgen_anon_1.data[2],
                x_bias: event.__bindgen_anon_1.__bindgen_anon_1.data[3],
                y_bias: event.__bindgen_anon_1.__bindgen_anon_1.data[4],
                z_bias: event.__bindgen_anon_1.__bindgen_anon_1.data[5],
            },
            ffi::ASENSOR_TYPE_SIGNIFICANT_MOTION => SensorData::SignificantMotion,
            _ => {
                error!("Received event for unknown sensor type");
                return None;
            }
        };
        Some(event)
    }
}

#[derive(Debug, Copy, Clone, Error)]
#[error("Android sensor event queue error")]
pub struct SensorEventQueueError(pub i32);
