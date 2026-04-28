#![deny(clippy::pedantic)]

use std::{
    error::Error,
    ffi::{CStr, CString},
    fmt,
    os::raw::c_char,
};

#[derive(Debug)]
pub enum SketchybarError {
    MessageConversionError,
    Other(String),
}

impl fmt::Display for SketchybarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SketchybarError::MessageConversionError => {
                write!(f, "Failed to convert message to CString")
            }
            SketchybarError::Other(description) => {
                write!(f, "Sketchybar error: {description}")
            }
        }
    }
}

impl Error for SketchybarError {}

#[link(name = "sketchybar", kind = "static")]
extern "C" {
    fn sketchybar(message: *mut c_char, bar_name: *mut c_char) -> *mut c_char;
}

/// Sends a message to `SketchyBar` and returns the response.
///
/// # Arguments
///
/// * `message` - A string slice containing the message to be sent to
/// `SketchyBar`.
/// * `bar_name` - An optional string slice containing the name of the process
/// of the target bar. This defaults to `sketchybar` however, if you're using a
/// secondary bar (eg. a `bottombar`) you can override the default there to pass
/// a message to this other bar.
///
/// # Returns
///
/// * `Ok(String)` - A `Result` containing a `String` with the response from
/// `SketchyBar` upon success.
/// * `Err(Box<dyn std::error::Error>)` - A `Result` containing an error if any
/// error occurs during the operation.
///
/// # Errors
///
/// This function will return an error if:
/// * The provided message cannot be converted to a `CString`.
/// * Any other unexpected condition occurs.
///
/// # Safety
///
/// This function contains unsafe code that calls into a C library (sketchybar).
/// Ensure the C library is correctly implemented to avoid undefined behavior.
///
/// # Memory ownership
///
/// The C side returns a `malloc`'d buffer (or `NULL`). After copying it into
/// a Rust `String`, this function `libc::free`s the original to avoid leaking
/// the response buffer on every call. A `NULL` response is mapped to an empty
/// string for backwards compatibility.
///
/// # Examples
///
/// ```no-run
/// use sketchybar_rs::message;
///
/// fn main() {
///     let response = message("--query bar").unwrap();
///
///     println!("Response from SketchyBar: {}", response);
/// }
/// ```
pub fn message(
    message: &str,
    bar_name: Option<&str>,
) -> Result<String, SketchybarError> {
    let command = CString::new(message)
        .map_err(|_| SketchybarError::MessageConversionError)?;

    let bar_name = CString::new(bar_name.unwrap_or("sketchybar"))
        .map_err(|_| SketchybarError::MessageConversionError)?;

    let raw_cmd = command.into_raw();
    let raw_bar = bar_name.into_raw();

    let response_ptr = unsafe { sketchybar(raw_cmd, raw_bar) };

    // Reclaim the CStrings we handed to C via into_raw().
    unsafe {
        let _ = CString::from_raw(raw_cmd);
        let _ = CString::from_raw(raw_bar);
    }

    let result = if response_ptr.is_null() {
        String::new()
    } else {
        let s = unsafe { CStr::from_ptr(response_ptr) }
            .to_string_lossy()
            .into_owned();
        // Free the malloc'd buffer the C side handed us.
        unsafe { libc::free(response_ptr.cast::<libc::c_void>()) };
        s
    };

    Ok(result)
}
