use std::ffi::{CStr, CString};

#[link(name = "sketchybar", kind = "static")]
extern "C" {
    fn sketchybar(message: *mut i8) -> *mut i8;
}

pub fn message(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let command = CString::new(message)?;

    let result = unsafe {
        CStr::from_ptr(sketchybar(command.into_raw()))
            .to_string_lossy()
            .into_owned()
    };

    println!("{}", result);

    Ok(result)
}
