//! Port types supported by core LV2.
//!
//! These types are the port types that can as type arguments for [`InputPort`](../plugin/ports/struct.InputPort.html) and [`OutputPort`](../plugin/ports/struct.InputPort.html).
use crate::uri::UriBound;
use std::ffi::c_void;
use std::ptr::NonNull;

/// Generalization of port types.
///
/// A port can read input or create a pointer to the output, but the exact type of input/output (pointer) depends on the type of port. This trait generalizes these types and behaviour.
pub trait PortType: 'static + Sized + UriBound {
    /// The type of input read by the port.
    type InputPortType: Sized;
    /// The type of output reference created by the port.
    type OutputPortType: Sized;

    /// Read data from the pointer or create a reference to the input.
    ///
    /// If the resulting data is a slice, `sample_count` is the length of the slice.
    ///
    /// # unsafety
    ///
    /// This method is unsafe because one needs to de-reference a raw pointer to implement this method.
    unsafe fn input_from_raw(pointer: NonNull<c_void>, sample_count: u32) -> Self::InputPortType;

    /// Create a reference to the data where output should be written to.
    ///
    /// If the data is a slice, `sample_count` is the length of the slice.
    ///
    /// # unsafety
    ///
    /// This method is unsafe because one needs to de-reference a raw pointer to implement this method.
    unsafe fn output_from_raw(pointer: NonNull<c_void>, sample_count: u32) -> Self::OutputPortType;
}

/// Audio port type.
///
/// Audio ports are the most common type of input/output ports: Their input is a slice of audio samples, as well as their output.
pub struct Audio;

unsafe impl UriBound for Audio {
    const URI: &'static [u8] = ::lv2_core_sys::LV2_CORE__AudioPort;
}

impl PortType for Audio {
    type InputPortType = &'static [f32];
    type OutputPortType = &'static mut [f32];

    #[inline]
    unsafe fn input_from_raw(pointer: NonNull<c_void>, sample_count: u32) -> Self::InputPortType {
        std::slice::from_raw_parts(pointer.as_ptr() as *const f32, sample_count as usize)
    }

    #[inline]
    unsafe fn output_from_raw(pointer: NonNull<c_void>, sample_count: u32) -> Self::OutputPortType {
        std::slice::from_raw_parts_mut(pointer.as_ptr() as *mut f32, sample_count as usize)
    }
}

/// Control value port type.
///
/// Control ports in general are used to control the behaviour of the plugin. These control value ports only have one value per `run` call and therefore don't have a fixed sampling rate.
pub struct Control;

unsafe impl UriBound for Control {
    const URI: &'static [u8] = ::lv2_core_sys::LV2_CORE__ControlPort;
}

impl PortType for Control {
    type InputPortType = f32;
    type OutputPortType = &'static mut f32;

    #[inline]
    unsafe fn input_from_raw(pointer: NonNull<c_void>, _sample_count: u32) -> f32 {
        *(pointer.cast().as_ref())
    }

    unsafe fn output_from_raw(pointer: NonNull<c_void>, _sample_count: u32) -> &'static mut f32 {
        (pointer.as_ptr() as *mut f32).as_mut().unwrap()
    }
}

/// CV port type.
///
/// Control ports in general are used to control the behaviour of the plugin. CV ports are sampled just like audio data. This means that audio data is often valid CV data, but CV data generally is not audio data, because it may not be within the audio bounds of -1.0 to 1.0.
pub struct CV;

unsafe impl UriBound for CV {
    const URI: &'static [u8] = ::lv2_core_sys::LV2_CORE__CVPort;
}

impl PortType for CV {
    type InputPortType = &'static [f32];
    type OutputPortType = &'static mut [f32];

    #[inline]
    unsafe fn input_from_raw(pointer: NonNull<c_void>, sample_count: u32) -> Self::InputPortType {
        std::slice::from_raw_parts(pointer.as_ptr() as *const f32, sample_count as usize)
    }

    #[inline]
    unsafe fn output_from_raw(pointer: NonNull<c_void>, sample_count: u32) -> Self::OutputPortType {
        std::slice::from_raw_parts_mut(pointer.as_ptr() as *mut f32, sample_count as usize)
    }
}
