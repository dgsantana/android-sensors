use std::ptr::NonNull;

use android_sensors_sys::ffi::sensors as ffi;
use log::error;

use crate::sensor::Sensor;

pub struct SensorEventQueue {
    ptr: NonNull<ffi::ASensorEventQueue>,
}

impl SensorEventQueue {
    pub fn from_ptr(ptr: NonNull<ffi::ASensorEventQueue>) -> Self {
        Self { ptr }
    }

    pub fn enable_sensor(&self, sensor: &Sensor) -> i32 {
        unsafe { ffi::ASensorEventQueue_enableSensor(self.ptr.as_ptr(), sensor.ptr().as_ptr()) }
    }

    pub fn disable_sensor(&self, sensor: &Sensor) -> i32 {
        unsafe { ffi::ASensorEventQueue_disableSensor(self.ptr.as_ptr(), sensor.ptr().as_ptr()) }
    }

    pub fn events(&self) -> Vec<SensorEvent> {
        let mut event = create_empty_event();
        let mut events = Vec::new();
        unsafe {
            while ffi::ASensorEventQueue_getEvents(self.ptr.as_ptr(), &mut event, 1) > 0 {
                let ptr = NonNull::new(event.sensor as *mut ffi::ASensor);
                if ptr.is_none() {
                    error!("Received event for unknown sensor");
                    continue;
                }
                let sensor = Sensor::from_ptr(ptr.unwrap());
                let Some(data) = SensorData::from_event(&event) else {
                    error!("Received unknown sensor data");
                    continue;
                };
                events.push(SensorEvent { sensor, data });
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

pub struct SensorEvent {
    pub sensor: Sensor,
    pub data: SensorData,
}

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
        if event.type_ < 1 || event.type_ > 15 {
            error!("Received event for unknown sensor type");
            panic!("Received event for unknown sensor type");
        }

        let event_type: ffi::ASensorType = std::mem::transmute(event.type_);
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
