// Android Looper API
// This module provides a safe wrapper around the Android Looper API.
// To make easier to develop on Windows, it "hides" the AsRawFd, BorrowedFd and RawFd
// on Windows since there isn't this file descriptor concept on Windows.
use std::{os::raw::c_void, ptr, time::Duration};

#[cfg(not(target_os = "windows"))]
use std::{
    mem::ManuallyDrop,
    os::fd::{AsRawFd, BorrowedFd, RawFd},
};

use android_sensors_sys::ffi::sensors as ffi;
use thiserror::Error;

#[cfg(not(target_os = "windows"))]
use crate::utils::abort_on_panic;

#[derive(Debug)]
pub struct ThreadLooper {
    _marker: std::marker::PhantomData<*mut ()>, // Not send or sync
    foreign: ForeignLooper,
}

impl ThreadLooper {
    /// Prepares a looper for the current thread and returns it
    pub fn prepare() -> Self {
        unsafe {
            let ptr = ffi::ALooper_prepare(ffi::ALOOPER_PREPARE_ALLOW_NON_CALLBACKS as _);
            let foreign = ForeignLooper::from_ptr(ptr::NonNull::new(ptr).expect("looper non null"));
            Self {
                _marker: std::marker::PhantomData,
                foreign,
            }
        }
    }

    /// Returns the looper associated with the current thread, if any.
    pub fn for_thread() -> Option<Self> {
        Some(Self {
            _marker: std::marker::PhantomData,
            foreign: ForeignLooper::for_thread()?,
        })
    }

    /// Polls the looper, blocking on processing an event, but with a timeout in milliseconds.
    /// Give a timeout of `0` to make this non-blocking.
    fn poll_once_ms(&self, ms: i32) -> Result<Poll<'_>, LooperError> {
        let mut fd = -1;
        let mut events = -1;
        let mut data: *mut c_void = ptr::null_mut();
        unsafe {
            match ffi::ALooper_pollOnce(ms, &mut fd, &mut events, &mut data) {
                i if i == ffi::ALOOPER_POLL_WAKE as i32 => Ok(Poll::Wake),
                i if i == ffi::ALOOPER_POLL_CALLBACK as i32 => Ok(Poll::Callback),
                i if i == ffi::ALOOPER_POLL_TIMEOUT as i32 => Ok(Poll::Timeout),
                i if i == ffi::ALOOPER_POLL_ERROR as i32 => Err(LooperError),
                ident if ident >= 0 => Ok(Poll::Event {
                    ident,
                    // SAFETY: Even though this FD at least shouldn't outlive self, a user could have
                    // closed it after calling add_fd or add_fd_with_callback.
                    #[cfg(not(target_os = "windows"))]
                    fd: BorrowedFd::borrow_raw(fd),
                    #[cfg(target_os = "windows")]
                    _fd: std::marker::PhantomData,
                    events: FdEvent::from_bits(events as u32)
                        .expect("poll event contains unknown bits"),
                    data,
                }),
                _ => unreachable!(),
            }
        }
    }

    /// Polls the looper, blocking on processing an event.
    #[inline]
    pub fn poll_once(&self) -> Result<Poll<'_>, LooperError> {
        self.poll_once_ms(-1)
    }

    /// Polls the looper, blocking on processing an event, but with a timeout.  Give a timeout of
    /// [`Duration::ZERO`] to make this non-blocking.
    ///
    /// It panics if the timeout is larger than expressible as an [`i32`] of milliseconds (roughly 25
    /// days).
    #[inline]
    pub fn poll_once_timeout(&self, timeout: Duration) -> Result<Poll<'_>, LooperError> {
        self.poll_once_ms(
            timeout
                .as_millis()
                .try_into()
                .expect("Supplied timeout is too large"),
        )
    }

    /// Repeatedly polls the looper, blocking on processing an event, but with a timeout in
    /// milliseconds.  Give a timeout of `0` to make this non-blocking.
    ///
    /// This function will never return [`Poll::Callback`].
    fn poll_all_ms(&self, ms: i32) -> Result<Poll<'_>, LooperError> {
        let mut fd = -1;
        let mut events = -1;
        let mut data: *mut c_void = ptr::null_mut();
        unsafe {
            match ffi::ALooper_pollAll(ms, &mut fd, &mut events, &mut data) {
                i if i == ffi::ALOOPER_POLL_WAKE as i32 => Ok(Poll::Wake),
                i if i == ffi::ALOOPER_POLL_TIMEOUT as i32 => Ok(Poll::Timeout),
                i if i == ffi::ALOOPER_POLL_ERROR as i32 => Err(LooperError),
                ident if ident >= 0 => Ok(Poll::Event {
                    ident,
                    // SAFETY: Even though this FD at least shouldn't outlive self, a user could have
                    // closed it after calling add_fd or add_fd_with_callback.
                    #[cfg(not(target_os = "windows"))]
                    fd: BorrowedFd::borrow_raw(fd),
                    #[cfg(target_os = "windows")]
                    _fd: std::marker::PhantomData,
                    events: FdEvent::from_bits(events as u32)
                        .expect("poll event contains unknown bits"),
                    data,
                }),
                _ => unreachable!(),
            }
        }
    }

    /// Repeatedly polls the looper, blocking on processing an event.
    ///
    /// This function will never return [`Poll::Callback`].
    #[inline]
    pub fn poll_all(&self) -> Result<Poll<'_>, LooperError> {
        self.poll_all_ms(-1)
    }

    /// Repeatedly polls the looper, blocking on processing an event, but with a timeout.  Give a
    /// timeout of [`Duration::ZERO`] to make this non-blocking.
    ///
    /// This function will never return [`Poll::Callback`].
    ///
    /// It panics if the timeout is larger than expressible as an [`i32`] of milliseconds (roughly 25
    /// days).
    #[inline]
    pub fn poll_all_timeout(&self, timeout: Duration) -> Result<Poll<'_>, LooperError> {
        self.poll_all_ms(
            timeout
                .as_millis()
                .try_into()
                .expect("Supplied timeout is too large"),
        )
    }

    /// Returns a reference to the [`ForeignLooper`] that is associated with the current thread.
    pub fn as_foreign(&self) -> &ForeignLooper {
        &self.foreign
    }

    pub fn into_foreign(self) -> ForeignLooper {
        self.foreign
    }
}

bitflags::bitflags! {
    /// Flags for file descriptor events that a looper can monitor.
    ///
    /// These flag bits can be combined to monitor multiple events at once.
    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FdEvent : u32 {
        /// The file descriptor is available for read operations.
        #[doc(alias = "ALOOPER_EVENT_INPUT")]
        const INPUT = ffi::ALOOPER_EVENT_INPUT as u32;
        /// The file descriptor is available for write operations.
        #[doc(alias = "ALOOPER_EVENT_OUTPUT")]
        const OUTPUT = ffi::ALOOPER_EVENT_OUTPUT as u32;
        /// The file descriptor has encountered an error condition.
        ///
        /// The looper always sends notifications about errors; it is not necessary to specify this
        /// event flag in the requested event set.
        #[doc(alias = "ALOOPER_EVENT_ERROR")]
        const ERROR = ffi::ALOOPER_EVENT_ERROR as u32;
        /// The file descriptor was hung up.
        ///
        /// For example, indicates that the remote end of a pipe or socket was closed.
        ///
        /// The looper always sends notifications about hangups; it is not necessary to specify this
        /// event flag in the requested event set.
        #[doc(alias = "ALOOPER_EVENT_HANGUP")]
        const HANGUP = ffi::ALOOPER_EVENT_HANGUP as u32;
        /// The file descriptor is invalid.
        ///
        /// For example, the file descriptor was closed prematurely.
        ///
        /// The looper always sends notifications about invalid file descriptors; it is not
        /// necessary to specify this event flag in the requested event set.
        #[doc(alias = "ALOOPER_EVENT_INVALID")]
        const INVALID = ffi::ALOOPER_EVENT_INVALID as u32;

        // https://docs.rs/bitflags/latest/bitflags/#externally-defined-flags
        const _ = !0;
    }
}

/// The poll result from a [`ThreadLooper`].
#[derive(Debug)]
pub enum Poll<'fd> {
    /// This looper was woken using [`ForeignLooper::wake()`]
    Wake,
    /// For [`ThreadLooper::poll_once*()`][ThreadLooper::poll_once()], an event was received and processed using a callback.
    Callback,
    /// For [`ThreadLooper::poll_*_timeout()`][ThreadLooper::poll_once_timeout()], the requested timeout was reached before any events.
    Timeout,
    /// An event was received
    Event {
        ident: i32,
        /// # Safety
        /// The caller should guarantee that this file descriptor remains open after it was added
        /// via [`ForeignLooper::add_fd()`] or [`ForeignLooper::add_fd_with_callback()`].
        #[cfg(not(target_os = "windows"))]
        fd: BorrowedFd<'fd>,
        #[cfg(target_os = "windows")]
        _fd: std::marker::PhantomData<&'fd ()>,
        events: FdEvent,
        data: *mut c_void,
    },
}

#[derive(Debug)]
pub struct ForeignLooper {
    ptr: ptr::NonNull<ffi::ALooper>,
}

unsafe impl Send for ForeignLooper {}
unsafe impl Sync for ForeignLooper {}

impl Clone for ForeignLooper {
    fn clone(&self) -> Self {
        unsafe { ffi::ALooper_acquire(self.ptr.as_ptr()) };
        ForeignLooper { ptr: self.ptr }
    }
}

impl Drop for ForeignLooper {
    fn drop(&mut self) {
        unsafe { ffi::ALooper_release(self.ptr.as_ptr()) }
    }
}

impl ForeignLooper {
    /// Construct a [`ForeignLooper`] object from the given pointer.
    ///
    /// # Safety
    /// By calling this function, you guarantee that the pointer is a valid, non-null pointer to an
    /// NDK [`ffi::ALooper`].
    pub unsafe fn from_ptr(ptr: ptr::NonNull<ffi::ALooper>) -> Self {
        ForeignLooper { ptr }
    }

    /// Returns the looper associated with the current thread, if any.
    #[inline]
    pub fn for_thread() -> Option<ForeignLooper> {
        ptr::NonNull::new(unsafe { ffi::ALooper_forThread() })
            .map(|ptr| unsafe { Self::from_ptr(ptr) })
    }

    /// Returns a pointer to the NDK `ALooper` object.
    #[inline]
    pub fn ptr(&self) -> ptr::NonNull<ffi::ALooper> {
        self.ptr
    }

    /// Wakes the looper.  An event of [`Poll::Wake`] will be sent.
    pub fn wake(&self) {
        unsafe { ffi::ALooper_wake(self.ptr.as_ptr()) }
    }

    /// Adds a file descriptor to be polled, without a callback.
    ///
    /// See also [the NDK
    /// docs](https://developer.android.com/ndk/reference/group/looper.html#alooper_addfd).
    ///
    /// # Safety
    /// The caller should guarantee that this file descriptor stays open until it is removed via
    /// [`remove_fd()`][Self::remove_fd()], and for however long the caller wishes to use this file
    /// descriptor when it is returned in [`Poll::Event::fd`].

    // `ALooper_addFd` won't dereference `data`; it will only pass it on to the event.
    // Optionally dereferencing it there already enforces `unsafe` context.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[cfg(not(target_os = "windows"))]
    pub fn add_fd(
        &self,
        fd: BorrowedFd<'_>,
        ident: i32,
        events: FdEvent,
        data: *mut c_void,
    ) -> Result<(), LooperError> {
        match unsafe {
            ffi::ALooper_addFd(
                self.ptr.as_ptr(),
                fd.as_raw_fd(),
                ident,
                events.bits() as i32,
                None,
                data,
            )
        } {
            1 => Ok(()),
            -1 => Err(LooperError),
            _ => unreachable!(),
        }
    }

    /// Adds a file descriptor to be polled, with a callback.
    ///
    /// The callback takes as an argument the file descriptor, and should return [`true`] to
    /// continue receiving callbacks, or [`false`] to have the callback unregistered.
    ///
    /// See also [the NDK
    /// docs](https://developer.android.com/ndk/reference/group/looper.html#alooper_addfd).
    ///
    /// Note that this will leak a [`Box`] unless the callback returns [`false`] to unregister
    /// itself.
    ///
    /// # Safety
    /// The caller should guarantee that this file descriptor stays open until it is removed via
    /// [`remove_fd()`][Self::remove_fd()] or by returning [`false`] from the callback, and for
    /// however long the caller wishes to use this file descriptor inside and after the callback.
    #[doc(alias = "ALooper_addFd")]
    #[cfg(not(target_os = "windows"))]
    pub fn add_fd_with_callback<F: FnMut(BorrowedFd<'_>, FdEvent) -> bool>(
        &self,
        fd: BorrowedFd<'_>,
        events: FdEvent,
        callback: F,
    ) -> Result<(), LooperError> {
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
        match unsafe {
            ffi::ALooper_addFd(
                self.ptr.as_ptr(),
                fd.as_raw_fd(),
                ffi::ALOOPER_POLL_CALLBACK as i32,
                events.bits() as i32,
                Some(cb_handler::<F>),
                data,
            )
        } {
            1 => Ok(()),
            -1 => Err(LooperError),
            _ => unreachable!(),
        }
    }

    /// Removes a previously added file descriptor from the looper.
    ///
    /// Returns [`true`] if the file descriptor was removed, [`false`] if it was not previously
    /// registered.
    ///
    /// # Safety
    /// When this method returns, it is safe to close the file descriptor since the looper will no
    /// longer have a reference to it. However, it is possible for the callback to already be
    /// running or for it to run one last time if the file descriptor was already signalled.
    /// Calling code is responsible for ensuring that this case is safely handled. For example, if
    /// the callback takes care of removing itself during its own execution either by returning `0`
    /// or by calling this method, then it can be guaranteed to not be invoked again at any later
    /// time unless registered anew.
    ///
    /// Note that unregistering a file descriptor with callback will leak a [`Box`] created in
    /// [`add_fd_with_callback()`][Self::add_fd_with_callback()]. Consider returning [`false`]
    /// from the callback instead to drop it.
    #[cfg(not(target_os = "windows"))]
    pub fn remove_fd(&self, fd: BorrowedFd<'_>) -> Result<bool, LooperError> {
        match unsafe { ffi::ALooper_removeFd(self.ptr.as_ptr(), fd.as_raw_fd()) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => Err(LooperError),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Error)]
#[error("Android Looper error")]
pub struct LooperError;
