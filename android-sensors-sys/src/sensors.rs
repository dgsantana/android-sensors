use crate::looper::{ALooper, ALooper_callbackFunc};

#[doc = " {@link ASensorRef} is a type for constant pointers to {@link ASensor}.\n\n This is used to define entry in {@link ASensorList} arrays."]
pub type ASensorRef = *const ASensor;
#[doc = " {@link ASensorList} is an array of reference to {@link ASensor}.\n\n A {@link ASensorList} can be initialized using ASensorManager_getSensorList()."]
pub type ASensorList = *const ASensorRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AHardwareBuffer {
    _unused: [u8; 0],
}
#[doc = " A sensor event."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ASensorVector {
    pub __bindgen_anon_1: ASensorVector__bindgen_ty_1,
    pub status: i8,
    pub reserved: [u8; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ASensorVector__bindgen_ty_1__bindgen_ty_1 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ASensorVector__bindgen_ty_1__bindgen_ty_2 {
    pub azimuth: f32,
    pub pitch: f32,
    pub roll: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AMetaDataEvent {
    pub what: i32,
    pub sensor: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AUncalibratedEvent {
    pub __bindgen_anon_1: AUncalibratedEvent__bindgen_ty_1,
    pub __bindgen_anon_2: AUncalibratedEvent__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUncalibratedEvent__bindgen_ty_1__bindgen_ty_1 {
    pub x_uncalib: f32,
    pub y_uncalib: f32,
    pub z_uncalib: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUncalibratedEvent__bindgen_ty_2__bindgen_ty_1 {
    pub x_bias: f32,
    pub y_bias: f32,
    pub z_bias: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AHeartRateEvent {
    pub bpm: f32,
    pub status: i8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ADynamicSensorEvent {
    pub connected: i32,
    pub handle: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AAdditionalInfoEvent {
    #[doc = " Event type, such as ASENSOR_ADDITIONAL_INFO_BEGIN, ASENSOR_ADDITIONAL_INFO_END and others.\n Refer to {@link ASENSOR_TYPE_ADDITIONAL_INFO} for the expected reporting behavior."]
    pub type_: i32,
    pub serial: i32,
    pub __bindgen_anon_1: AAdditionalInfoEvent__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AHeadTrackerEvent {
    #[doc = " The fields rx, ry, rz are an Euler vector (rotation vector, i.e. a vector\n whose direction indicates the axis of rotation and magnitude indicates\n the angle to rotate around that axis) representing the transform from\n the (arbitrary, possibly slowly drifting) reference frame to the\n head frame. Expressed in radians. Magnitude of the vector must be\n in the range [0, pi], while the value of individual axes are\n in the range [-pi, pi]."]
    pub rx: f32,
    pub ry: f32,
    pub rz: f32,
    #[doc = " The fields vx, vy, vz are an Euler vector (rotation vector) representing\n the angular velocity of the head (relative to itself), in radians per\n second. The direction of this vector indicates the axis of rotation, and\n the magnitude indicates the rate of rotation."]
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    #[doc = " This value changes each time the reference frame is suddenly and\n significantly changed, for example if an orientation filter algorithm\n used for determining the orientation has had its state reset."]
    pub discontinuity_count: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALimitedAxesImuEvent {
    pub __bindgen_anon_1: ALimitedAxesImuEvent__bindgen_ty_1,
    pub __bindgen_anon_2: ALimitedAxesImuEvent__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ALimitedAxesImuEvent__bindgen_ty_1__bindgen_ty_1 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ALimitedAxesImuEvent__bindgen_ty_2__bindgen_ty_1 {
    pub x_supported: f32,
    pub y_supported: f32,
    pub z_supported: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALimitedAxesImuUncalibratedEvent {
    pub __bindgen_anon_1: ALimitedAxesImuUncalibratedEvent__bindgen_ty_1,
    pub __bindgen_anon_2: ALimitedAxesImuUncalibratedEvent__bindgen_ty_2,
    pub __bindgen_anon_3: ALimitedAxesImuUncalibratedEvent__bindgen_ty_3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ALimitedAxesImuUncalibratedEvent__bindgen_ty_1__bindgen_ty_1 {
    pub x_uncalib: f32,
    pub y_uncalib: f32,
    pub z_uncalib: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ALimitedAxesImuUncalibratedEvent__bindgen_ty_2__bindgen_ty_1 {
    pub x_bias: f32,
    pub y_bias: f32,
    pub z_bias: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ALimitedAxesImuUncalibratedEvent__bindgen_ty_3__bindgen_ty_1 {
    pub x_supported: f32,
    pub y_supported: f32,
    pub z_supported: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AHeadingEvent {
    #[doc = " The direction in which the device is pointing relative to true north in\n degrees. The value must be between 0.0 (inclusive) and 360.0 (exclusive),\n with 0 indicating north, 90 east, 180 south, and 270 west."]
    pub heading: f32,
    #[doc = " Accuracy is defined at 68% confidence. In the case where the underlying\n distribution is assumed Gaussian normal, this would be considered one\n standard deviation. For example, if the heading returns 60 degrees, and\n accuracy returns 10 degrees, then there is a 68 percent probability of\n the true heading being between 50 degrees and 70 degrees."]
    pub accuracy: f32,
}
#[doc = " Information that describes a sensor event, refer to\n <a href=\"/reference/android/hardware/SensorEvent\">SensorEvent</a> for additional\n documentation.\n\n NOTE: changes to this struct has to be backward compatible and reflected in\n sensors_event_t"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ASensorEvent {
    pub version: i32,
    pub sensor: i32,
    #[doc = " The sensor that generates this event"]
    pub type_: i32,
    #[doc = " Sensor type for the event, such as {@link ASENSOR_TYPE_ACCELEROMETER}"]
    pub reserved0: i32,
    #[doc = " do not use */\n/**\n The time in nanoseconds at which the event happened, and its behavior\n is identical to <a href=\"/reference/android/hardware/SensorEvent#timestamp\">\n SensorEvent::timestamp</a> in Java API."]
    pub timestamp: i64,
    pub __bindgen_anon_1: ASensorEvent__bindgen_ty_1,
    pub flags: u32,
    pub reserved1: [i32; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ASensorManager {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ASensorEventQueue {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ASensor {
    _unused: [u8; 0],
}
pub const ASENSOR_FIFO_COUNT_INVALID: i32 = -1;
pub const ASENSOR_DELAY_INVALID: i32 = -2147483648;
pub const ASENSOR_INVALID: i32 = -1;
pub const ASENSOR_STANDARD_GRAVITY: f64 = 9.80665;
pub const ASENSOR_MAGNETIC_FIELD_EARTH_MAX: f64 = 60.0;
pub const ASENSOR_MAGNETIC_FIELD_EARTH_MIN: f64 = 30.0;
pub const ASENSOR_TYPE_INVALID: ASensorType = ASensorType::ASENSOR_TYPE_INVALID;
pub const ASENSOR_TYPE_ACCELEROMETER: ASensorType = ASensorType::ASENSOR_TYPE_ACCELEROMETER;
pub const ASENSOR_TYPE_MAGNETIC_FIELD: ASensorType = ASensorType::ASENSOR_TYPE_MAGNETIC_FIELD;
pub const ASENSOR_TYPE_GYROSCOPE: ASensorType = ASensorType::ASENSOR_TYPE_GYROSCOPE;
pub const ASENSOR_TYPE_LIGHT: ASensorType = ASensorType::ASENSOR_TYPE_LIGHT;
pub const ASENSOR_TYPE_PRESSURE: ASensorType = ASensorType::ASENSOR_TYPE_PRESSURE;
pub const ASENSOR_TYPE_PROXIMITY: ASensorType = ASensorType::ASENSOR_TYPE_PROXIMITY;
pub const ASENSOR_TYPE_GRAVITY: ASensorType = ASensorType::ASENSOR_TYPE_GRAVITY;
pub const ASENSOR_TYPE_LINEAR_ACCELERATION: ASensorType =
    ASensorType::ASENSOR_TYPE_LINEAR_ACCELERATION;
pub const ASENSOR_TYPE_ROTATION_VECTOR: ASensorType = ASensorType::ASENSOR_TYPE_ROTATION_VECTOR;
pub const ASENSOR_TYPE_RELATIVE_HUMIDITY: ASensorType =
    ASensorType::ASENSOR_TYPE_RELATIVE_HUMIDITY;
pub const ASENSOR_TYPE_AMBIENT_TEMPERATURE: ASensorType =
    ASensorType::ASENSOR_TYPE_AMBIENT_TEMPERATURE;
pub const ASENSOR_TYPE_MAGNETIC_FIELD_UNCALIBRATED: ASensorType =
    ASensorType::ASENSOR_TYPE_MAGNETIC_FIELD_UNCALIBRATED;
pub const ASENSOR_TYPE_GAME_ROTATION_VECTOR: ASensorType =
    ASensorType::ASENSOR_TYPE_GAME_ROTATION_VECTOR;
pub const ASENSOR_TYPE_GYROSCOPE_UNCALIBRATED: ASensorType =
    ASensorType::ASENSOR_TYPE_GYROSCOPE_UNCALIBRATED;
pub const ASENSOR_TYPE_SIGNIFICANT_MOTION: ASensorType =
    ASensorType::ASENSOR_TYPE_SIGNIFICANT_MOTION;
pub const ASENSOR_TYPE_STEP_DETECTOR: ASensorType = ASensorType::ASENSOR_TYPE_STEP_DETECTOR;
pub const ASENSOR_TYPE_STEP_COUNTER: ASensorType = ASensorType::ASENSOR_TYPE_STEP_COUNTER;
pub const ASENSOR_TYPE_GEOMAGNETIC_ROTATION_VECTOR: ASensorType =
    ASensorType::ASENSOR_TYPE_GEOMAGNETIC_ROTATION_VECTOR;
pub const ASENSOR_TYPE_HEART_RATE: ASensorType = ASensorType::ASENSOR_TYPE_HEART_RATE;
pub const ASENSOR_TYPE_POSE_6DOF: ASensorType = ASensorType::ASENSOR_TYPE_POSE_6DOF;
pub const ASENSOR_TYPE_STATIONARY_DETECT: ASensorType =
    ASensorType::ASENSOR_TYPE_STATIONARY_DETECT;
pub const ASENSOR_TYPE_MOTION_DETECT: ASensorType = ASensorType::ASENSOR_TYPE_MOTION_DETECT;
pub const ASENSOR_TYPE_HEART_BEAT: ASensorType = ASensorType::ASENSOR_TYPE_HEART_BEAT;
pub const ASENSOR_TYPE_DYNAMIC_SENSOR_META: ASensorType =
    ASensorType::ASENSOR_TYPE_DYNAMIC_SENSOR_META;
pub const ASENSOR_TYPE_ADDITIONAL_INFO: ASensorType = ASensorType::ASENSOR_TYPE_ADDITIONAL_INFO;
pub const ASENSOR_TYPE_LOW_LATENCY_OFFBODY_DETECT: ASensorType =
    ASensorType::ASENSOR_TYPE_LOW_LATENCY_OFFBODY_DETECT;
pub const ASENSOR_TYPE_ACCELEROMETER_UNCALIBRATED: ASensorType =
    ASensorType::ASENSOR_TYPE_ACCELEROMETER_UNCALIBRATED;
pub const ASENSOR_TYPE_HINGE_ANGLE: ASensorType = ASensorType::ASENSOR_TYPE_HINGE_ANGLE;
pub const ASENSOR_TYPE_HEAD_TRACKER: ASensorType = ASensorType::ASENSOR_TYPE_HEAD_TRACKER;
pub const ASENSOR_TYPE_ACCELEROMETER_LIMITED_AXES: ASensorType =
    ASensorType::ASENSOR_TYPE_ACCELEROMETER_LIMITED_AXES;
pub const ASENSOR_TYPE_GYROSCOPE_LIMITED_AXES: ASensorType =
    ASensorType::ASENSOR_TYPE_GYROSCOPE_LIMITED_AXES;
pub const ASENSOR_TYPE_ACCELEROMETER_LIMITED_AXES_UNCALIBRATED: ASensorType =
    ASensorType::ASENSOR_TYPE_ACCELEROMETER_LIMITED_AXES_UNCALIBRATED;
pub const ASENSOR_TYPE_GYROSCOPE_LIMITED_AXES_UNCALIBRATED: ASensorType =
    ASensorType::ASENSOR_TYPE_GYROSCOPE_LIMITED_AXES_UNCALIBRATED;
pub const ASENSOR_TYPE_HEADING: ASensorType = ASensorType::ASENSOR_TYPE_HEADING;
pub const ASENSOR_STATUS_NO_CONTACT: ASensorStatus = ASensorStatus::ASENSOR_STATUS_NO_CONTACT;
pub const ASENSOR_STATUS_UNRELIABLE: ASensorStatus = ASensorStatus::ASENSOR_STATUS_UNRELIABLE;
pub const ASENSOR_STATUS_ACCURACY_LOW: ASensorStatus = ASensorStatus::ASENSOR_STATUS_ACCURACY_LOW;
pub const ASENSOR_STATUS_ACCURACY_MEDIUM: ASensorStatus =
    ASensorStatus::ASENSOR_STATUS_ACCURACY_MEDIUM;
pub const ASENSOR_STATUS_ACCURACY_HIGH: ASensorStatus = ASensorStatus::ASENSOR_STATUS_ACCURACY_HIGH;
pub const AREPORTING_MODE_INVALID: AReportingMode = AReportingMode::AREPORTING_MODE_INVALID;
pub const AREPORTING_MODE_CONTINUOUS: AReportingMode = AReportingMode::AREPORTING_MODE_CONTINUOUS;
pub const AREPORTING_MODE_ON_CHANGE: AReportingMode = AReportingMode::AREPORTING_MODE_ON_CHANGE;
pub const AREPORTING_MODE_ONE_SHOT: AReportingMode = AReportingMode::AREPORTING_MODE_ONE_SHOT;
pub const AREPORTING_MODE_SPECIAL_TRIGGER: AReportingMode =
    AReportingMode::AREPORTING_MODE_SPECIAL_TRIGGER;
pub const ASENSOR_DIRECT_RATE_STOP: ASensorDirectRate = ASensorDirectRate::ASENSOR_DIRECT_RATE_STOP;
pub const ASENSOR_DIRECT_RATE_NORMAL: ASensorDirectRate = ASensorDirectRate::ASENSOR_DIRECT_RATE_NORMAL;
pub const ASENSOR_DIRECT_RATE_FAST: ASensorDirectRate = ASensorDirectRate::ASENSOR_DIRECT_RATE_FAST;
pub const ASENSOR_DIRECT_RATE_VERY_FAST: ASensorDirectRate =
    ASensorDirectRate::ASENSOR_DIRECT_RATE_VERY_FAST;
pub const ASENSOR_DIRECT_CHANNEL_TYPE_SHARED_MEMORY: ASensorDirectChannel =
    ASensorDirectChannel::ASENSOR_DIRECT_CHANNEL_TYPE_SHARED_MEMORY;
pub const ASENSOR_DIRECT_CHANNEL_TYPE_HARDWARE_BUFFER: ASensorDirectChannel =
    ASensorDirectChannel::ASENSOR_DIRECT_CHANNEL_TYPE_HARDWARE_BUFFER;
pub const ASENSOR_ADDITIONAL_INFO_BEGIN: ASensorAdditionalInfo =
    ASensorAdditionalInfo::ASENSOR_ADDITIONAL_INFO_BEGIN;
pub const ASENSOR_ADDITIONAL_INFO_END: ASensorAdditionalInfo = ASensorAdditionalInfo::ASENSOR_ADDITIONAL_INFO_END;
pub const ASENSOR_ADDITIONAL_INFO_UNTRACKED_DELAY: ASensorAdditionalInfo =
    ASensorAdditionalInfo::ASENSOR_ADDITIONAL_INFO_UNTRACKED_DELAY;
pub const ASENSOR_ADDITIONAL_INFO_INTERNAL_TEMPERATURE: ASensorAdditionalInfo =
    ASensorAdditionalInfo::ASENSOR_ADDITIONAL_INFO_INTERNAL_TEMPERATURE;
pub const ASENSOR_ADDITIONAL_INFO_VEC3_CALIBRATION: ASensorAdditionalInfo =
    ASensorAdditionalInfo::ASENSOR_ADDITIONAL_INFO_VEC3_CALIBRATION;
pub const ASENSOR_ADDITIONAL_INFO_SENSOR_PLACEMENT: ASensorAdditionalInfo =
    ASensorAdditionalInfo::ASENSOR_ADDITIONAL_INFO_SENSOR_PLACEMENT;
pub const ASENSOR_ADDITIONAL_INFO_SAMPLING: ASensorAdditionalInfo =
    ASensorAdditionalInfo::ASENSOR_ADDITIONAL_INFO_SAMPLING;

#[repr(i32)]
#[doc = " Sensor types.\n\n See\n [android.hardware.SensorEvent#values](https://developer.android.com/reference/android/hardware/SensorEvent.html#values)\n for detailed explanations of the data returned for each of these types."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ASensorType {
    #[doc = " Invalid sensor type. Returned by {@link ASensor_getType} as error value."]
    ASENSOR_TYPE_INVALID = -1,
    #[doc = " {@link ASENSOR_TYPE_ACCELEROMETER}\n reporting-mode: continuous\n\n  All values are in SI units (m/s^2) and measure the acceleration of the\n  device minus the force of gravity."]
    ASENSOR_TYPE_ACCELEROMETER = 1,
    #[doc = " {@link ASENSOR_TYPE_MAGNETIC_FIELD}\n reporting-mode: continuous\n\n  All values are in micro-Tesla (uT) and measure the geomagnetic\n  field in the X, Y and Z axis."]
    ASENSOR_TYPE_MAGNETIC_FIELD = 2,
    #[doc = " {@link ASENSOR_TYPE_GYROSCOPE}\n reporting-mode: continuous\n\n  All values are in radians/second and measure the rate of rotation\n  around the X, Y and Z axis."]
    ASENSOR_TYPE_GYROSCOPE = 4,
    #[doc = " {@link ASENSOR_TYPE_LIGHT}\n reporting-mode: on-change\n\n The light sensor value is returned in SI lux units."]
    ASENSOR_TYPE_LIGHT = 5,
    #[doc = " {@link ASENSOR_TYPE_PRESSURE}\n\n The pressure sensor value is returned in hPa (millibar)."]
    ASENSOR_TYPE_PRESSURE = 6,
    #[doc = " {@link ASENSOR_TYPE_PROXIMITY}\n reporting-mode: on-change\n\n The proximity sensor which turns the screen off and back on during calls is the\n wake-up proximity sensor. Implement wake-up proximity sensor before implementing\n a non wake-up proximity sensor. For the wake-up proximity sensor set the flag\n SENSOR_FLAG_WAKE_UP.\n The value corresponds to the distance to the nearest object in centimeters."]
    ASENSOR_TYPE_PROXIMITY = 8,
    #[doc = " {@link ASENSOR_TYPE_GRAVITY}\n\n All values are in SI units (m/s^2) and measure the direction and\n magnitude of gravity. When the device is at rest, the output of\n the gravity sensor should be identical to that of the accelerometer."]
    ASENSOR_TYPE_GRAVITY = 9,
    #[doc = " {@link ASENSOR_TYPE_LINEAR_ACCELERATION}\n reporting-mode: continuous\n\n  All values are in SI units (m/s^2) and measure the acceleration of the\n  device not including the force of gravity."]
    ASENSOR_TYPE_LINEAR_ACCELERATION = 10,
    #[doc = " {@link ASENSOR_TYPE_ROTATION_VECTOR}"]
    ASENSOR_TYPE_ROTATION_VECTOR = 11,
    #[doc = " {@link ASENSOR_TYPE_RELATIVE_HUMIDITY}\n\n The relative humidity sensor value is returned in percent."]
    ASENSOR_TYPE_RELATIVE_HUMIDITY = 12,
    #[doc = " {@link ASENSOR_TYPE_AMBIENT_TEMPERATURE}\n\n The ambient temperature sensor value is returned in Celcius."]
    ASENSOR_TYPE_AMBIENT_TEMPERATURE = 13,
    #[doc = " {@link ASENSOR_TYPE_MAGNETIC_FIELD_UNCALIBRATED}"]
    ASENSOR_TYPE_MAGNETIC_FIELD_UNCALIBRATED = 14,
    #[doc = " {@link ASENSOR_TYPE_GAME_ROTATION_VECTOR}"]
    ASENSOR_TYPE_GAME_ROTATION_VECTOR = 15,
    #[doc = " {@link ASENSOR_TYPE_GYROSCOPE_UNCALIBRATED}"]
    ASENSOR_TYPE_GYROSCOPE_UNCALIBRATED = 16,
    #[doc = " {@link ASENSOR_TYPE_SIGNIFICANT_MOTION}"]
    ASENSOR_TYPE_SIGNIFICANT_MOTION = 17,
    #[doc = " {@link ASENSOR_TYPE_STEP_DETECTOR}"]
    ASENSOR_TYPE_STEP_DETECTOR = 18,
    #[doc = " {@link ASENSOR_TYPE_STEP_COUNTER}"]
    ASENSOR_TYPE_STEP_COUNTER = 19,
    #[doc = " {@link ASENSOR_TYPE_GEOMAGNETIC_ROTATION_VECTOR}"]
    ASENSOR_TYPE_GEOMAGNETIC_ROTATION_VECTOR = 20,
    #[doc = " {@link ASENSOR_TYPE_HEART_RATE}"]
    ASENSOR_TYPE_HEART_RATE = 21,
    #[doc = " {@link ASENSOR_TYPE_POSE_6DOF}"]
    ASENSOR_TYPE_POSE_6DOF = 28,
    #[doc = " {@link ASENSOR_TYPE_STATIONARY_DETECT}"]
    ASENSOR_TYPE_STATIONARY_DETECT = 29,
    #[doc = " {@link ASENSOR_TYPE_MOTION_DETECT}"]
    ASENSOR_TYPE_MOTION_DETECT = 30,
    #[doc = " {@link ASENSOR_TYPE_HEART_BEAT}"]
    ASENSOR_TYPE_HEART_BEAT = 31,
    #[doc = " A constant describing a dynamic sensor meta event sensor.\n\n A sensor event of this type is received when a dynamic sensor is added to or removed from\n the system. This sensor type should always use special trigger report mode."]
    ASENSOR_TYPE_DYNAMIC_SENSOR_META = 32,
    #[doc = " This sensor type is for delivering additional sensor information aside\n from sensor event data.\n\n Additional information may include:\n     - {@link ASENSOR_ADDITIONAL_INFO_INTERNAL_TEMPERATURE}\n     - {@link ASENSOR_ADDITIONAL_INFO_SAMPLING}\n     - {@link ASENSOR_ADDITIONAL_INFO_SENSOR_PLACEMENT}\n     - {@link ASENSOR_ADDITIONAL_INFO_UNTRACKED_DELAY}\n     - {@link ASENSOR_ADDITIONAL_INFO_VEC3_CALIBRATION}\n\n This type will never bind to a sensor. In other words, no sensor in the\n sensor list can have the type {@link ASENSOR_TYPE_ADDITIONAL_INFO}.\n\n If a device supports the sensor additional information feature, it will\n report additional information events via {@link ASensorEvent} and will\n have the type of {@link ASensorEvent} set to\n {@link ASENSOR_TYPE_ADDITIONAL_INFO} and the sensor of {@link ASensorEvent} set\n to the handle of the reporting sensor.\n\n Additional information reports consist of multiple frames ordered by\n {@link ASensorEvent#timestamp}. The first frame in the report will have\n a {@link AAdditionalInfoEvent#type} of\n {@link ASENSOR_ADDITIONAL_INFO_BEGIN}, and the last frame in the report\n will have a {@link AAdditionalInfoEvent#type} of\n {@link ASENSOR_ADDITIONAL_INFO_END}.\n"]
    ASENSOR_TYPE_ADDITIONAL_INFO = 33,
    #[doc = " {@link ASENSOR_TYPE_LOW_LATENCY_OFFBODY_DETECT}"]
    ASENSOR_TYPE_LOW_LATENCY_OFFBODY_DETECT = 34,
    #[doc = " {@link ASENSOR_TYPE_ACCELEROMETER_UNCALIBRATED}"]
    ASENSOR_TYPE_ACCELEROMETER_UNCALIBRATED = 35,
    #[doc = " {@link ASENSOR_TYPE_HINGE_ANGLE}\n reporting-mode: on-change\n\n The hinge angle sensor value is returned in degrees."]
    ASENSOR_TYPE_HINGE_ANGLE = 36,
    #[doc = " {@link ASENSOR_TYPE_HEAD_TRACKER}\n reporting-mode: continuous\n\n Measures the orientation and rotational velocity of a user's head. Only for internal use\n within the Android system."]
    ASENSOR_TYPE_HEAD_TRACKER = 37,
    #[doc = " {@link ASENSOR_TYPE_ACCELEROMETER_LIMITED_AXES}\n reporting-mode: continuous\n\n The first three values are in SI units (m/s^2) and measure the acceleration of the device\n minus the force of gravity. The last three values indicate which acceleration axes are\n supported. A value of 1.0 means supported and a value of 0 means not supported."]
    ASENSOR_TYPE_ACCELEROMETER_LIMITED_AXES = 38,
    #[doc = " {@link ASENSOR_TYPE_GYROSCOPE_LIMITED_AXES}\n reporting-mode: continuous\n\n The first three values are in radians/second and measure the rate of rotation around the X,\n Y and Z axis. The last three values indicate which rotation axes are supported. A value of\n 1.0 means supported and a value of 0 means not supported."]
    ASENSOR_TYPE_GYROSCOPE_LIMITED_AXES = 39,
    #[doc = " {@link ASENSOR_TYPE_ACCELEROMETER_LIMITED_AXES_UNCALIBRATED}\n reporting-mode: continuous\n\n The first three values are in SI units (m/s^2) and measure the acceleration of the device\n minus the force of gravity. The middle three values represent the estimated bias for each\n axis. The last three values indicate which acceleration axes are supported. A value of 1.0\n means supported and a value of 0 means not supported."]
    ASENSOR_TYPE_ACCELEROMETER_LIMITED_AXES_UNCALIBRATED = 40,
    #[doc = " {@link ASENSOR_TYPE_GYROSCOPE_LIMITED_AXES_UNCALIBRATED}\n reporting-mode: continuous\n\n The first three values are in radians/second and measure the rate of rotation around the X,\n Y and Z axis. The middle three values represent the estimated drift around each axis in\n rad/s. The last three values indicate which rotation axes are supported. A value of 1.0 means\n supported and a value of 0 means not supported."]
    ASENSOR_TYPE_GYROSCOPE_LIMITED_AXES_UNCALIBRATED = 41,
    #[doc = " {@link ASENSOR_TYPE_HEADING}\n reporting-mode: continuous\n\n A heading sensor measures the direction in which the device is pointing\n relative to true north in degrees."]
    ASENSOR_TYPE_HEADING = 42,
}

#[repr(i32)]
#[doc = " Sensor accuracy measure."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ASensorStatus {
    #[doc = " no contact"]
    ASENSOR_STATUS_NO_CONTACT = -1,
    #[doc = " unreliable"]
    ASENSOR_STATUS_UNRELIABLE = 0,
    #[doc = " low accuracy"]
    ASENSOR_STATUS_ACCURACY_LOW = 1,
    #[doc = " medium accuracy"]
    ASENSOR_STATUS_ACCURACY_MEDIUM = 2,
    #[doc = " high accuracy"]
    ASENSOR_STATUS_ACCURACY_HIGH = 3,
}
#[repr(i32)]
#[doc = " Sensor Reporting Modes."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AReportingMode {
    #[doc = " invalid reporting mode"]
    AREPORTING_MODE_INVALID = -1,
    #[doc = " continuous reporting"]
    AREPORTING_MODE_CONTINUOUS = 0,
    #[doc = " reporting on change"]
    AREPORTING_MODE_ON_CHANGE = 1,
    #[doc = " on shot reporting"]
    AREPORTING_MODE_ONE_SHOT = 2,
    #[doc = " special trigger reporting"]
    AREPORTING_MODE_SPECIAL_TRIGGER = 3,
}
#[repr(u32)]
#[doc = " Sensor Direct Report Rates."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ASensorDirectRate {
    #[doc = " stopped"]
    ASENSOR_DIRECT_RATE_STOP = 0,
    #[doc = " nominal 50Hz"]
    ASENSOR_DIRECT_RATE_NORMAL = 1,
    #[doc = " nominal 200Hz"]
    ASENSOR_DIRECT_RATE_FAST = 2,
    #[doc = " nominal 800Hz"]
    ASENSOR_DIRECT_RATE_VERY_FAST = 3,
}
#[repr(u32)]
#[doc = " Sensor Direct Channel Type."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ASensorDirectChannel {
    #[doc = " shared memory created by ASharedMemory_create"]
    ASENSOR_DIRECT_CHANNEL_TYPE_SHARED_MEMORY = 1,
    #[doc = " AHardwareBuffer"]
    ASENSOR_DIRECT_CHANNEL_TYPE_HARDWARE_BUFFER = 2,
}
#[repr(u32)]
#[doc = " Sensor Additional Info Types.\n\n Used to populate {@link AAdditionalInfoEvent#type}."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ASensorAdditionalInfo {
    #[doc = " Marks the beginning of additional information frames"]
    ASENSOR_ADDITIONAL_INFO_BEGIN = 0,
    #[doc = " Marks the end of additional information frames"]
    ASENSOR_ADDITIONAL_INFO_END = 1,
    #[doc = " Estimation of the delay that is not tracked by sensor timestamps. This\n includes delay introduced by sensor front-end filtering, data transport,\n etc.\n float[2]: delay in seconds, standard deviation of estimated value"]
    ASENSOR_ADDITIONAL_INFO_UNTRACKED_DELAY = 65536,
    #[doc = " float: Celsius temperature"]
    ASENSOR_ADDITIONAL_INFO_INTERNAL_TEMPERATURE = 65537,
    #[doc = " First three rows of a homogeneous matrix, which represents calibration to\n a three-element vector raw sensor reading.\n float[12]: 3x4 matrix in row major order"]
    ASENSOR_ADDITIONAL_INFO_VEC3_CALIBRATION = 65538,
    #[doc = " Location and orientation of sensor element in the device frame: origin is\n the geometric center of the mobile device screen surface; the axis\n definition corresponds to Android sensor definitions.\n float[12]: 3x4 matrix in row major order"]
    ASENSOR_ADDITIONAL_INFO_SENSOR_PLACEMENT = 65539,
    #[doc = " float[2]: raw sample period in seconds,\n           standard deviation of sampling period"]
    ASENSOR_ADDITIONAL_INFO_SAMPLING = 65540,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ASensorVector__bindgen_ty_1 {
    pub v: [f32; 3usize],
    pub __bindgen_anon_1: ASensorVector__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: ASensorVector__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AUncalibratedEvent__bindgen_ty_1 {
    pub uncalib: [f32; 3usize],
    pub __bindgen_anon_1: AUncalibratedEvent__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AUncalibratedEvent__bindgen_ty_2 {
    pub bias: [f32; 3usize],
    pub __bindgen_anon_1: AUncalibratedEvent__bindgen_ty_2__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AAdditionalInfoEvent__bindgen_ty_1 {
    pub data_int32: [i32; 14usize],
    pub data_float: [f32; 14usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ALimitedAxesImuEvent__bindgen_ty_1 {
    pub calib: [f32; 3usize],
    pub __bindgen_anon_1: ALimitedAxesImuEvent__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ALimitedAxesImuEvent__bindgen_ty_2 {
    pub supported: [f32; 3usize],
    pub __bindgen_anon_1: ALimitedAxesImuEvent__bindgen_ty_2__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ALimitedAxesImuUncalibratedEvent__bindgen_ty_1 {
    pub uncalib: [f32; 3usize],
    pub __bindgen_anon_1: ALimitedAxesImuUncalibratedEvent__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ALimitedAxesImuUncalibratedEvent__bindgen_ty_2 {
    pub bias: [f32; 3usize],
    pub __bindgen_anon_1: ALimitedAxesImuUncalibratedEvent__bindgen_ty_2__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ALimitedAxesImuUncalibratedEvent__bindgen_ty_3 {
    pub supported: [f32; 3usize],
    pub __bindgen_anon_1: ALimitedAxesImuUncalibratedEvent__bindgen_ty_3__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ASensorEvent__bindgen_ty_1 {
    pub __bindgen_anon_1: ASensorEvent__bindgen_ty_1__bindgen_ty_1,
    pub u64_: ASensorEvent__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ASensorEvent__bindgen_ty_1__bindgen_ty_1 {
    pub data: [f32; 16usize],
    pub vector: ASensorVector,
    pub acceleration: ASensorVector,
    pub gyro: ASensorVector,
    pub magnetic: ASensorVector,
    pub temperature: f32,
    pub distance: f32,
    pub light: f32,
    pub pressure: f32,
    pub relative_humidity: f32,
    pub uncalibrated_acceleration: AUncalibratedEvent,
    pub uncalibrated_gyro: AUncalibratedEvent,
    pub uncalibrated_magnetic: AUncalibratedEvent,
    pub meta_data: AMetaDataEvent,
    pub heart_rate: AHeartRateEvent,
    pub dynamic_sensor_meta: ADynamicSensorEvent,
    pub additional_info: AAdditionalInfoEvent,
    pub head_tracker: AHeadTrackerEvent,
    pub limited_axes_imu: ALimitedAxesImuEvent,
    pub limited_axes_imu_uncalibrated: ALimitedAxesImuUncalibratedEvent,
    pub heading: AHeadingEvent,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ASensorEvent__bindgen_ty_1__bindgen_ty_2 {
    pub data: [u64; 8usize],
    pub step_counter: u64,
}
extern "C" {
    #[doc = " Get a reference to the sensor manager. ASensorManager is a singleton\n per package as different packages may have access to different sensors.\n\n Deprecated: Use ASensorManager_getInstanceForPackage(const char*) instead.\n\n Example:\n\n     ASensorManager* sensorManager = ASensorManager_getInstance();\n"]
    pub fn ASensorManager_getInstance() -> *mut ASensorManager;
    #[doc = " Get a reference to the sensor manager. ASensorManager is a singleton\n per package as different packages may have access to different sensors.\n\n Example:\n\n     ASensorManager* sensorManager = ASensorManager_getInstanceForPackage(\"foo.bar.baz\");\n\n Available since API level 26."]
    pub fn ASensorManager_getInstanceForPackage(
        packageName: *const libc::c_char,
    ) -> *mut ASensorManager;
    #[doc = " Returns the list of available sensors. The returned list is owned by the\n sensor manager and will not change between calls to this function.\n\n \\param manager the {@link ASensorManager} instance obtained from\n                {@link ASensorManager_getInstanceForPackage}.\n \\param list    the returned list of sensors.\n \\return positive number of returned sensors or negative error code.\n         BAD_VALUE: manager is NULL."]
    pub fn ASensorManager_getSensorList(
        manager: *mut ASensorManager,
        list: *mut ASensorList,
    ) -> libc::c_int;
    #[doc = " Returns the list of available dynamic sensors. If there are no dynamic\n sensors available, returns nullptr in list.\n\n Each time this is called, the previously returned list is deallocated and\n must no longer be used.\n\n Clients should call this if they receive a sensor update from\n {@link ASENSOR_TYPE_DYNAMIC_SENSOR_META} indicating the sensors have changed.\n If this happens, previously received lists from this method will be stale.\n\n Available since API level 33.\n\n \\param manager the {@link ASensorManager} instance obtained from\n                {@link ASensorManager_getInstanceForPackage}.\n \\param list    the returned list of dynamic sensors.\n \\return positive number of returned sensors or negative error code.\n         BAD_VALUE: manager is NULL."]
    pub fn ASensorManager_getDynamicSensorList(
        manager: *mut ASensorManager,
        list: *mut ASensorList,
    ) -> isize;
    #[doc = " Returns the default sensor for the given type, or NULL if no sensor\n of that type exists."]
    pub fn ASensorManager_getDefaultSensor(
        manager: *mut ASensorManager,
        type_: libc::c_int,
    ) -> *const ASensor;
    #[doc = " Returns the default sensor with the given type and wakeUp properties or NULL if no sensor\n of this type and wakeUp properties exists.\n\n Available since API level 21."]
    pub fn ASensorManager_getDefaultSensorEx(
        manager: *mut ASensorManager,
        type_: libc::c_int,
        wakeUp: bool,
    ) -> *const ASensor;
    #[doc = " Creates a new sensor event queue and associate it with a looper.\n\n \"ident\" is a identifier for the events that will be returned when\n calling ALooper_pollOnce(). The identifier must be >= 0, or\n ALOOPER_POLL_CALLBACK if providing a non-NULL callback."]
    pub fn ASensorManager_createEventQueue(
        manager: *mut ASensorManager,
        looper: *mut ALooper,
        ident: libc::c_int,
        callback: ALooper_callbackFunc,
        data: *mut libc::c_void,
    ) -> *mut ASensorEventQueue;
    #[doc = " Destroys the event queue and free all resources associated to it."]
    pub fn ASensorManager_destroyEventQueue(
        manager: *mut ASensorManager,
        queue: *mut ASensorEventQueue,
    ) -> libc::c_int;
    #[doc = " Create direct channel based on shared memory\n\n Create a direct channel of {@link ASENSOR_DIRECT_CHANNEL_TYPE_SHARED_MEMORY} to be used\n for configuring sensor direct report.\n\n Available since API level 26.\n\n \\param manager the {@link ASensorManager} instance obtained from\n                {@link ASensorManager_getInstanceForPackage}.\n \\param fd      file descriptor representing a shared memory created by\n                {@link ASharedMemory_create}\n \\param size    size to be used, must be less or equal to size of shared memory.\n\n \\return a positive integer as a channel id to be used in\n         {@link ASensorManager_destroyDirectChannel} and\n         {@link ASensorManager_configureDirectReport}, or value less or equal to 0 for failures."]
    pub fn ASensorManager_createSharedMemoryDirectChannel(
        manager: *mut ASensorManager,
        fd: libc::c_int,
        size: usize,
    ) -> libc::c_int;
    #[doc = " Create direct channel based on AHardwareBuffer\n\n Create a direct channel of {@link ASENSOR_DIRECT_CHANNEL_TYPE_HARDWARE_BUFFER} type to be used\n for configuring sensor direct report.\n\n Available since API level 26.\n\n \\param manager the {@link ASensorManager} instance obtained from\n                {@link ASensorManager_getInstanceForPackage}.\n \\param buffer  {@link AHardwareBuffer} instance created by {@link AHardwareBuffer_allocate}.\n \\param size    the intended size to be used, must be less or equal to size of buffer.\n\n \\return a positive integer as a channel id to be used in\n         {@link ASensorManager_destroyDirectChannel} and\n         {@link ASensorManager_configureDirectReport}, or value less or equal to 0 for failures."]
    pub fn ASensorManager_createHardwareBufferDirectChannel(
        manager: *mut ASensorManager,
        buffer: *const AHardwareBuffer,
        size: usize,
    ) -> libc::c_int;
    #[doc = " Destroy a direct channel\n\n Destroy a direct channel previously created by using one of\n ASensorManager_create*DirectChannel() derivative functions.\n Note that the buffer used for creating the direct channel does not get destroyed with\n ASensorManager_destroyDirectChannel and has to be closed or released separately.\n\n Available since API level 26.\n\n \\param manager the {@link ASensorManager} instance obtained from\n                {@link ASensorManager_getInstanceForPackage}.\n \\param channelId channel id (a positive integer) returned from\n                  {@link ASensorManager_createSharedMemoryDirectChannel} or\n                  {@link ASensorManager_createHardwareBufferDirectChannel}."]
    pub fn ASensorManager_destroyDirectChannel(
        manager: *mut ASensorManager,
        channelId: libc::c_int,
    );
    #[doc = " Configure direct report on channel\n\n Configure sensor direct report on a direct channel: set rate to value other than\n {@link ASENSOR_DIRECT_RATE_STOP} so that sensor event can be directly\n written into the shared memory region used for creating the buffer. It returns a positive token\n which can be used for identify sensor events from different sensors on success. Calling with rate\n {@link ASENSOR_DIRECT_RATE_STOP} will stop direct report of the sensor specified in the channel.\n\n To stop all active sensor direct report configured to a channel, set sensor to NULL and rate to\n {@link ASENSOR_DIRECT_RATE_STOP}.\n\n In order to successfully configure a direct report, the sensor has to support the specified rate\n and the channel type, which can be checked by {@link ASensor_getHighestDirectReportRateLevel} and\n {@link ASensor_isDirectChannelTypeSupported}, respectively.\n\n Example:\n\n     ASensorManager *manager = ...;\n     ASensor *sensor = ...;\n     int channelId = ...;\n\n     ASensorManager_configureDirectReport(manager, sensor, channel_id, ASENSOR_DIRECT_RATE_FAST);\n\n Available since API level 26.\n\n \\param manager   the {@link ASensorManager} instance obtained from\n                  {@link ASensorManager_getInstanceForPackage}.\n \\param sensor    a {@link ASensor} to denote which sensor to be operate. It can be NULL if rate\n                  is {@link ASENSOR_DIRECT_RATE_STOP}, denoting stopping of all active sensor\n                  direct report.\n \\param channelId channel id (a positive integer) returned from\n                  {@link ASensorManager_createSharedMemoryDirectChannel} or\n                  {@link ASensorManager_createHardwareBufferDirectChannel}.\n \\param rate      one of predefined ASENSOR_DIRECT_RATE_... that is supported by the sensor.\n \\return positive token for success or negative error code."]
    pub fn ASensorManager_configureDirectReport(
        manager: *mut ASensorManager,
        sensor: *const ASensor,
        channelId: libc::c_int,
        rate: libc::c_int,
    ) -> libc::c_int;
    #[doc = " Enable the selected sensor with sampling and report parameters\n\n Enable the selected sensor at a specified sampling period and max batch report latency.\n To disable  sensor, use {@link ASensorEventQueue_disableSensor}.\n\n \\param queue {@link ASensorEventQueue} for sensor event to be report to.\n \\param sensor {@link ASensor} to be enabled.\n \\param samplingPeriodUs sampling period of sensor in microseconds.\n \\param maxBatchReportLatencyUs maximum time interval between two batches of sensor events are\n                                delievered in microseconds. For sensor streaming, set to 0.\n \\return 0 on success or a negative error code on failure."]
    pub fn ASensorEventQueue_registerSensor(
        queue: *mut ASensorEventQueue,
        sensor: *const ASensor,
        samplingPeriodUs: i32,
        maxBatchReportLatencyUs: i64,
    ) -> libc::c_int;
    #[doc = " Enable the selected sensor at default sampling rate.\n\n Start event reports of a sensor to specified sensor event queue at a default rate.\n\n \\param queue {@link ASensorEventQueue} for sensor event to be report to.\n \\param sensor {@link ASensor} to be enabled.\n\n \\return 0 on success or a negative error code on failure."]
    pub fn ASensorEventQueue_enableSensor(
        queue: *mut ASensorEventQueue,
        sensor: *const ASensor,
    ) -> libc::c_int;
    #[doc = " Disable the selected sensor.\n\n Stop event reports from the sensor to specified sensor event queue.\n\n \\param queue {@link ASensorEventQueue} to be changed\n \\param sensor {@link ASensor} to be disabled\n \\return 0 on success or a negative error code on failure."]
    pub fn ASensorEventQueue_disableSensor(
        queue: *mut ASensorEventQueue,
        sensor: *const ASensor,
    ) -> libc::c_int;
    #[doc = " Sets the delivery rate of events in microseconds for the given sensor.\n\n This function has to be called after {@link ASensorEventQueue_enableSensor}.\n Note that this is a hint only, generally event will arrive at a higher\n rate. It is an error to set a rate inferior to the value returned by\n ASensor_getMinDelay().\n\n \\param queue {@link ASensorEventQueue} to which sensor event is delivered.\n \\param sensor {@link ASensor} of which sampling rate to be updated.\n \\param usec sensor sampling period (1/sampling rate) in microseconds\n \\return 0 on sucess or a negative error code on failure."]
    pub fn ASensorEventQueue_setEventRate(
        queue: *mut ASensorEventQueue,
        sensor: *const ASensor,
        usec: i32,
    ) -> libc::c_int;
    #[doc = " Determine if a sensor event queue has pending event to be processed.\n\n \\param queue {@link ASensorEventQueue} to be queried\n \\return 1 if the queue has events; 0 if it does not have events;\n         or a negative value if there is an error."]
    pub fn ASensorEventQueue_hasEvents(queue: *mut ASensorEventQueue) -> libc::c_int;
    #[doc = " Retrieve pending events in sensor event queue\n\n Retrieve next available events from the queue to a specified event array.\n\n \\param queue {@link ASensorEventQueue} to get events from\n \\param events pointer to an array of {@link ASensorEvent}.\n \\param count max number of event that can be filled into array event.\n \\return number of events returned on success; negative error code when\n         no events are pending or an error has occurred.\n\n Examples:\n\n     ASensorEvent event;\n     ssize_t numEvent = ASensorEventQueue_getEvents(queue, &event, 1);\n\n     ASensorEvent eventBuffer[8];\n     ssize_t numEvent = ASensorEventQueue_getEvents(queue, eventBuffer, 8);\n"]
    pub fn ASensorEventQueue_getEvents(
        queue: *mut ASensorEventQueue,
        events: *mut ASensorEvent,
        count: usize,
    ) -> isize;
    #[doc = " Request that {@link ASENSOR_TYPE_ADDITIONAL_INFO} events to be delivered on\n the given {@link ASensorEventQueue}.\n\n Sensor data events are always delivered to the {@link ASensorEventQueue}.\n\n The {@link ASENSOR_TYPE_ADDITIONAL_INFO} events will be returned through\n {@link ASensorEventQueue_getEvents}. The client is responsible for checking\n {@link ASensorEvent#type} to determine the event type prior to handling of\n the event.\n\n The client must be tolerant of any value for\n {@link AAdditionalInfoEvent#type}, as new values may be defined in the future\n and may delivered to the client.\n\n Available since API level 29.\n\n \\param queue {@link ASensorEventQueue} to configure\n \\param enable true to request {@link ASENSOR_TYPE_ADDITIONAL_INFO} events,\n        false to stop receiving events\n \\return 0 on success or a negative error code on failure"]
    pub fn ASensorEventQueue_requestAdditionalInfoEvents(
        queue: *mut ASensorEventQueue,
        enable: bool,
    ) -> libc::c_int;
    #[doc = " Returns this sensor's name (non localized)"]
    pub fn ASensor_getName(sensor: *const ASensor) -> *const libc::c_char;
    #[doc = " Returns this sensor's vendor's name (non localized)"]
    pub fn ASensor_getVendor(sensor: *const ASensor) -> *const libc::c_char;
    #[doc = " Return this sensor's type"]
    pub fn ASensor_getType(sensor: *const ASensor) -> libc::c_int;
    #[doc = " Returns this sensors's resolution"]
    pub fn ASensor_getResolution(sensor: *const ASensor) -> f32;
    #[doc = " Returns the minimum delay allowed between events in microseconds.\n A value of zero means that this sensor doesn't report events at a\n constant rate, but rather only when a new data is available."]
    pub fn ASensor_getMinDelay(sensor: *const ASensor) -> libc::c_int;
    #[doc = " Returns the maximum size of batches for this sensor. Batches will often be\n smaller, as the hardware fifo might be used for other sensors.\n\n Available since API level 21."]
    pub fn ASensor_getFifoMaxEventCount(sensor: *const ASensor) -> libc::c_int;
    #[doc = " Returns the hardware batch fifo size reserved to this sensor.\n\n Available since API level 21."]
    pub fn ASensor_getFifoReservedEventCount(sensor: *const ASensor) -> libc::c_int;
    #[doc = " Returns this sensor's string type.\n\n Available since API level 21."]
    pub fn ASensor_getStringType(sensor: *const ASensor) -> *const libc::c_char;
    #[doc = " Returns the reporting mode for this sensor. One of AREPORTING_MODE_* constants.\n\n Available since API level 21."]
    pub fn ASensor_getReportingMode(sensor: *const ASensor) -> libc::c_int;
    #[doc = " Returns true if this is a wake up sensor, false otherwise.\n\n Available since API level 21."]
    pub fn ASensor_isWakeUpSensor(sensor: *const ASensor) -> bool;
    #[doc = " Test if sensor supports a certain type of direct channel.\n\n Available since API level 26.\n\n \\param sensor  a {@link ASensor} to denote the sensor to be checked.\n \\param channelType  Channel type constant, either\n                     {@link ASENSOR_DIRECT_CHANNEL_TYPE_SHARED_MEMORY}\n                     or {@link ASENSOR_DIRECT_CHANNEL_TYPE_HARDWARE_BUFFER}.\n \\returns true if sensor supports the specified direct channel type."]
    pub fn ASensor_isDirectChannelTypeSupported(
        sensor: *const ASensor,
        channelType: libc::c_int,
    ) -> bool;
    #[doc = " Get the highest direct rate level that a sensor supports.\n\n Available since API level 26.\n\n \\param sensor  a {@link ASensor} to denote the sensor to be checked.\n\n \\return a ASENSOR_DIRECT_RATE_... enum denoting the highest rate level supported by the sensor.\n         If return value is {@link ASENSOR_DIRECT_RATE_STOP}, it means the sensor\n         does not support direct report."]
    pub fn ASensor_getHighestDirectReportRateLevel(sensor: *const ASensor) -> libc::c_int;
    #[doc = " Returns the sensor's handle.\n\n The handle identifies the sensor within the system and is included in the\n sensor field of {@link ASensorEvent}, including those sent with type\n {@link ASENSOR_TYPE_ADDITIONAL_INFO}.\n\n A sensor's handle is able to be used to map {@link ASENSOR_TYPE_ADDITIONAL_INFO} events to the\n sensor that generated the event.\n\n It is important to note that the value returned by {@link ASensor_getHandle} is not the same as\n the value returned by the Java API <a href=\"/reference/android/hardware/Sensor#getId()\">\n android.hardware.Sensor's getId()</a> and no mapping exists between the values.\n\n Available since API level 29."]
    pub fn ASensor_getHandle(sensor: *const ASensor) -> libc::c_int;
}
