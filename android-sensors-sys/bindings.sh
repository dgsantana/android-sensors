#!/bin/bash
ANDROID_NDK_ROOT="/mnt/f/devtools/android/ndk/26.2.11394342"
BINDGEN_ARGS_LOOPER="--no-doc-comments --generate-cstr --impl-debug --default-enum-style rust --ctypes-prefix libc --merge-extern-blocks --sort-semantically --generate-block --opaque-type ALooper"
BINDGEN_ARGS_SENSORS="--use-core --generate-cstr --default-enum-style rust --impl-debug --ctypes-prefix libc --merge-extern-blocks --sort-semantically --generate-block"

# echo "Building bindings for looper"
# bindgen -o src/ffi/looper.rs $BINDGEN_ARGS_LOOPER $ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/windows-x86_64/sysroot/usr/include/android/looper.h -- --sysroot="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/windows-x86_64/sysroot" -I"$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/windows-x86_64/sysroot/usr/include/i686-linux-android/"

echo "Building bindings for sensor"
bindgen -o src/sensors.rs \
    --use-core \
    --generate-cstr \
    --default-enum-style rust_non_exhaustive \
    --ctypes-prefix libc \
    --allowlist-item ALooper.* \
    --allowlist-function ALooper.* \
    --allowlist-var ALOOPER.* \
    --allowlist-item ALOOPER.* \
    --allowlist-type ASensor.* \
    --allowlist-item ASensor.* \
    --allowlist-function ASensor.* \
    --allowlist-type AHardwareBuffer.* \
    --allowlist-item AHardwareBuffer.* \
    --allowlist-function AHardwareBuffer.* \
    --allowlist-type .*Event.* \
    --allowlist-item .*Event.* \
    --allowlist-function .*Event.* \
    --allowlist-var ASENSOR.* \
    --allowlist-var AREPORTING.* \
    --raw-line "pub type ASensorType = _bindgen_ty_4;" \
    --raw-line "pub type ALooperEvent = _bindgen_ty_3;" \
    --raw-line "pub type ALooperPoll = _bindgen_ty_2;" \
    --merge-extern-blocks \
    --wrap-static-fns \
    --sort-semantically \
    --generate-block \
    --experimental \
    --no-layout-tests \
    wrapper.h \
    -- \
    --sysroot="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/windows-x86_64/sysroot/" \
    -I"$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/windows-x86_64/sysroot/usr/include/" \
    -I"$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/windows-x86_64/sysroot/usr/include/i686-linux-android/"

# echo "Building bindings for logging"
# bindgen -o src/logging.rs \
#     --use-core \
#     --generate-cstr \
#     --default-enum-style rust_non_exhaustive \
#     --ctypes-prefix libc \
#     --allowlist-item android_.* \
#     --allowlist-item __android.* \
#     --allowlist-item log_.* \
#     --allowlist-item ANDROID_LOG.* \
#     --rustified-enum A.* \
#     --no-default A.* \
#     --no-prepend-enum-name \
#     --merge-extern-blocks \
#     --wrap-static-fns \
#     --sort-semantically \
#     --generate-block \
#     --experimental \
#     --no-layout-tests \
#     $ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/windows-x86_64/sysroot/usr/include/android/log.h \
#     -- \
#     --sysroot="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/windows-x86_64/sysroot/" \
#     -I"$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/windows-x86_64/sysroot/usr/include/" \
#     -I"$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/windows-x86_64/sysroot/usr/include/i686-linux-android/"

