use std::{ffi::c_int, ptr::NonNull};

#[cfg(not(target_os = "windows"))]
use std::{
    ffi::c_void,
    mem::ManuallyDrop,
    os::fd::{AsRawFd, BorrowedFd, RawFd},
};

use android_sensors_sys::ffi::sensors as ffi;

#[cfg(not(target_os = "windows"))]
use crate::{looper::FdEvent, utils::abort_on_panic};

use super::{
    looper::ForeignLooper,
    sensor::{Sensor, SensorType},
    sensor_queue::SensorEventQueue,
};

pub struct SensorManager {
    ptr: *mut ffi::ASensorManager,
}

impl SensorManager {
    pub fn new() -> Self {
        let ptr = unsafe { ffi::ASensorManager_getInstance() };
        Self { ptr }
    }

    pub fn get_sensor_list(&self) -> Vec<Sensor> {
        let mut array_ptr = std::ptr::null();
        let count = unsafe { ffi::ASensorManager_getSensorList(self.ptr, &mut array_ptr) };
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
                let non_null_ptr = NonNull::new(*sensor_ptr as *mut ffi::ASensor)?;
                Some(Sensor::from_ptr(non_null_ptr))
            })
            .collect()
    }

    pub fn get_default_sensor(&self, sensor_type: SensorType) -> Option<Sensor> {
        let internal_type: ffi::ASensorType = sensor_type.into();
        let ptr = unsafe { ffi::ASensorManager_getDefaultSensor(self.ptr, internal_type as c_int) };
        Some(unsafe { Sensor::from_ptr(NonNull::new(ptr as *mut ffi::ASensor)?) })
    }

    pub fn get_default_sensor_ex(&self, sensor_type: SensorType, wake_up: bool) -> Option<Sensor> {
        let internal_type: ffi::ASensorType = sensor_type.into();
        let ptr = unsafe {
            ffi::ASensorManager_getDefaultSensorEx(self.ptr, internal_type as c_int, wake_up)
        };
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { Sensor::from_ptr(NonNull::new(ptr as *mut ffi::ASensor)?) })
    }

    /// Create a new event queue for the given looper and ident.
    /// The ident is used to identify the events in the queue and should be a positive number.
    /// The looper is used to signal when events are available.
    /// The callback is called when events are available.
    pub fn create_event_queue(
        &self,
        looper: &ForeignLooper,
        ident: i32,
    ) -> Option<SensorEventQueue> {
        let queue_ptr = unsafe {
            ffi::ASensorManager_createEventQueue(
                self.ptr,
                looper.ptr().as_ptr(),
                ident,
                None,
                std::ptr::null_mut(),
            )
        };
        if queue_ptr.is_null() {
            return None;
        }
        Some(SensorEventQueue::from_ptr(NonNull::new(queue_ptr)?))
    }

    #[cfg(not(target_os = "windows"))]
    pub fn create_event_queue_with_callback<F: FnMut(BorrowedFd<'_>, FdEvent) -> bool>(
        &self,
        looper: &ForeignLooper,
        ident: i32,
        callback: F,
    ) -> Option<SensorEventQueue> {
        extern "C" fn cb_handler<F: FnMut(BorrowedFd<'_>, FdEvent) -> bool>(
            fd: RawFd,
            events: i32,
            data: *mut c_void,
        ) -> i32 {
            abort_on_panic(|| unsafe {
                let mut cb = ManuallyDrop::new(Box::<F>::from_raw(data as *mut _));
                let events = FdEvent::from_bits_retain(
                    events.try_into().expect("Unexpected sign bit in `events`"),
                );
                let keep_registered = cb(BorrowedFd::borrow_raw(fd), events);
                if !keep_registered {
                    ManuallyDrop::into_inner(cb);
                }
                keep_registered as i32
            })
        }
        let data = Box::into_raw(Box::new(callback)) as *mut _;
        let queue_ptr = unsafe {
            ffi::ASensorManager_createEventQueue(
                self.ptr,
                looper.ptr().as_ptr(),
                ident,
                Some(cb_handler::<F>),
                data,
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
