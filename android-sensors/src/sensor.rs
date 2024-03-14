use std::ptr::NonNull;

use android_sensors_sys::ffi::sensors as ffi;

#[derive(Debug)]
pub struct Sensor {
    ptr: NonNull<ffi::ASensor>,
}

impl Sensor {
    /// Create a new sensor from a pointer.
    /// # Safety
    /// The pointer must be valid and point to a valid sensor.
    pub unsafe fn from_ptr(ptr: NonNull<ffi::ASensor>) -> Self {
        Self { ptr }
    }

    pub fn ptr(&self) -> NonNull<ffi::ASensor> {
        self.ptr
    }

    pub fn get_name(&self) -> String {
        let ptr = unsafe { ffi::ASensor_getName(self.ptr.as_ptr()) };
        let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };
        cstr.to_string_lossy().into_owned()
    }

    pub fn get_vendor(&self) -> String {
        let ptr = unsafe { ffi::ASensor_getVendor(self.ptr.as_ptr()) };
        let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };
        cstr.to_string_lossy().into_owned()
    }

    pub fn get_type(&self) -> i32 {
        unsafe { ffi::ASensor_getType(self.ptr.as_ptr()) }
    }

    pub fn get_min_delay(&self) -> i32 {
        unsafe { ffi::ASensor_getMinDelay(self.ptr.as_ptr()) }
    }

    pub fn get_resolution(&self) -> f32 {
        unsafe { ffi::ASensor_getResolution(self.ptr.as_ptr()) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensorType {
    Invalid,
    Unknown(i32),
    Accelerometer,
    MagneticField,
    Gyroscope,
    Light,
    Proximity,
    Gravity,
    LinearAcceleration,
    RotationVector,
    RelativeHumidity,
    AmbientTemperature,
    MagneticFieldUncalibrated,
    GameRotationVector,
    GyroscopeUncalibrated,
    SignificantMotion,
    StepDetector,
    StepCounter,
    GeomagneticRotationVector,
    HeartRate,
    Pose6DOF,
    StationaryDetect,
    MotionDetect,
    HeartBeat,
    LowLatencyOffBodyDetect,
    AccelerometerUncalibrated,
}

impl From<ffi::ASensorType> for SensorType {
    fn from(value: ffi::ASensorType) -> Self {
        match value {
            ffi::ASENSOR_TYPE_ACCELEROMETER => Self::Accelerometer,
            ffi::ASENSOR_TYPE_MAGNETIC_FIELD => Self::MagneticField,
            ffi::ASENSOR_TYPE_GYROSCOPE => Self::Gyroscope,
            ffi::ASENSOR_TYPE_LIGHT => Self::Light,
            ffi::ASENSOR_TYPE_PROXIMITY => Self::Proximity,
            ffi::ASENSOR_TYPE_GRAVITY => Self::Gravity,
            ffi::ASENSOR_TYPE_LINEAR_ACCELERATION => Self::LinearAcceleration,
            ffi::ASENSOR_TYPE_ROTATION_VECTOR => Self::RotationVector,
            ffi::ASENSOR_TYPE_RELATIVE_HUMIDITY => Self::RelativeHumidity,
            ffi::ASENSOR_TYPE_AMBIENT_TEMPERATURE => Self::AmbientTemperature,
            ffi::ASENSOR_TYPE_MAGNETIC_FIELD_UNCALIBRATED => Self::MagneticFieldUncalibrated,
            ffi::ASENSOR_TYPE_GAME_ROTATION_VECTOR => Self::GameRotationVector,
            ffi::ASENSOR_TYPE_GYROSCOPE_UNCALIBRATED => Self::GyroscopeUncalibrated,
            ffi::ASENSOR_TYPE_SIGNIFICANT_MOTION => Self::SignificantMotion,
            ffi::ASENSOR_TYPE_STEP_DETECTOR => Self::StepDetector,
            ffi::ASENSOR_TYPE_STEP_COUNTER => Self::StepCounter,
            ffi::ASENSOR_TYPE_GEOMAGNETIC_ROTATION_VECTOR => Self::GeomagneticRotationVector,
            ffi::ASENSOR_TYPE_HEART_RATE => Self::HeartRate,
            ffi::ASENSOR_TYPE_POSE_6DOF => Self::Pose6DOF,
            ffi::ASENSOR_TYPE_STATIONARY_DETECT => Self::StationaryDetect,
            ffi::ASENSOR_TYPE_MOTION_DETECT => Self::MotionDetect,
            ffi::ASENSOR_TYPE_HEART_BEAT => Self::HeartBeat,
            ffi::ASENSOR_TYPE_LOW_LATENCY_OFFBODY_DETECT => Self::LowLatencyOffBodyDetect,
            ffi::ASENSOR_TYPE_ACCELEROMETER_UNCALIBRATED => Self::AccelerometerUncalibrated,
            ffi::ASENSOR_TYPE_INVALID => Self::Invalid,
            _ => Self::Unknown(value as i32),
        }
    }
}

impl From<SensorType> for ffi::ASensorType {
    fn from(value: SensorType) -> Self {
        match value {
            SensorType::Accelerometer => ffi::ASENSOR_TYPE_ACCELEROMETER,
            SensorType::MagneticField => ffi::ASENSOR_TYPE_MAGNETIC_FIELD,
            SensorType::Gyroscope => ffi::ASENSOR_TYPE_GYROSCOPE,
            SensorType::Light => ffi::ASENSOR_TYPE_LIGHT,
            SensorType::Proximity => ffi::ASENSOR_TYPE_PROXIMITY,
            SensorType::Gravity => ffi::ASENSOR_TYPE_GRAVITY,
            SensorType::LinearAcceleration => ffi::ASENSOR_TYPE_LINEAR_ACCELERATION,
            SensorType::RotationVector => ffi::ASENSOR_TYPE_ROTATION_VECTOR,
            SensorType::RelativeHumidity => ffi::ASENSOR_TYPE_RELATIVE_HUMIDITY,
            SensorType::AmbientTemperature => ffi::ASENSOR_TYPE_AMBIENT_TEMPERATURE,
            SensorType::MagneticFieldUncalibrated => ffi::ASENSOR_TYPE_MAGNETIC_FIELD_UNCALIBRATED,
            SensorType::GameRotationVector => ffi::ASENSOR_TYPE_GAME_ROTATION_VECTOR,
            SensorType::GyroscopeUncalibrated => ffi::ASENSOR_TYPE_GYROSCOPE_UNCALIBRATED,
            SensorType::SignificantMotion => ffi::ASENSOR_TYPE_SIGNIFICANT_MOTION,
            SensorType::StepDetector => ffi::ASENSOR_TYPE_STEP_DETECTOR,
            SensorType::StepCounter => ffi::ASENSOR_TYPE_STEP_COUNTER,
            SensorType::GeomagneticRotationVector => ffi::ASENSOR_TYPE_GEOMAGNETIC_ROTATION_VECTOR,
            SensorType::HeartRate => ffi::ASENSOR_TYPE_HEART_RATE,
            SensorType::Pose6DOF => ffi::ASENSOR_TYPE_POSE_6DOF,
            SensorType::StationaryDetect => ffi::ASENSOR_TYPE_STATIONARY_DETECT,
            SensorType::MotionDetect => ffi::ASENSOR_TYPE_MOTION_DETECT,
            SensorType::HeartBeat => ffi::ASENSOR_TYPE_HEART_BEAT,
            SensorType::LowLatencyOffBodyDetect => ffi::ASENSOR_TYPE_LOW_LATENCY_OFFBODY_DETECT,
            SensorType::AccelerometerUncalibrated => ffi::ASENSOR_TYPE_ACCELEROMETER_UNCALIBRATED,
            SensorType::Invalid => ffi::ASENSOR_TYPE_INVALID,
            SensorType::Unknown(_) => ffi::_bindgen_ty_4::ASENSOR_TYPE_INVALID,
        }
    }
}
