use std::{ffi::c_int, ptr::NonNull};

use android_sensors_sys::ffi::sensors::{self as bindings, ASensor};

use crate::looper::LooperIdent;

use super::{
    looper::ForeignLooper,
    sensor::{Sensor, SensorType},
    sensor_queue::SensorEventQueue,
};

pub struct SensorManager {
    ptr: *mut bindings::ASensorManager,
}

impl SensorManager {
    pub fn new() -> Self {
        let ptr = unsafe { bindings::ASensorManager_getInstance() };
        Self { ptr }
    }

    pub fn get_sensor_list(&self) -> Vec<Sensor> {
        let mut array_ptr = std::ptr::null();
        let count = unsafe { bindings::ASensorManager_getSensorList(self.ptr, &mut array_ptr) };
        if count < 0 {
            return Vec::new();
        }
        assert_ne!(
            array_ptr,
            std::ptr::null(),
            "ASensorManager_getSensorList returned a null array pointer"
        );
        let sensors_array = unsafe { std::slice::from_raw_parts(array_ptr, count as usize) };
        sensors_array
            .iter()
            .filter_map(|sensor_ptr| unsafe {
                let non_null_ptr = NonNull::new(*sensor_ptr as *mut ASensor)?;
                Some(Sensor::from_ptr(non_null_ptr))
            })
            .collect()
    }

    pub fn get_default_sensor(&self, sensor_type: SensorType) -> Option<Sensor> {
        let internal_type: bindings::ASensorType = sensor_type.into();
        let ptr =
            unsafe { bindings::ASensorManager_getDefaultSensor(self.ptr, internal_type as c_int) };
        Some(unsafe { Sensor::from_ptr(NonNull::new(ptr as *mut ASensor)?) })
    }

    pub fn get_default_sensor_ex(&self, sensor_type: SensorType, wake_up: bool) -> Option<Sensor> {
        let internal_type: bindings::ASensorType = sensor_type.into();
        let ptr = unsafe {
            bindings::ASensorManager_getDefaultSensorEx(self.ptr, internal_type as c_int, wake_up)
        };
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { Sensor::from_ptr(NonNull::new(ptr as *mut ASensor)?) })
    }

    pub fn create_event_queue(&self, looper: ForeignLooper) -> Option<SensorEventQueue> {
        let queue_ptr = unsafe {
            bindings::ASensorManager_createEventQueue(
                self.ptr,
                looper.ptr().as_ptr(),
                LooperIdent::User.into_i32(),
                None,
                std::ptr::null_mut(),
            )
        };
        if queue_ptr.is_null() {
            return None;
        }
        Some(SensorEventQueue::from_ptr(NonNull::new(queue_ptr)?))
    }
}

impl Default for SensorManager {
    fn default() -> Self {
        Self::new()
    }
}
