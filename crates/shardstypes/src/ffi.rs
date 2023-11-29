use std::{alloc::Layout, ptr::Alignment};

#[repr(C)]
#[derive(Debug)]
pub struct FfiString {
    pub ptr: *mut u8,
    pub len: usize,
    pub cap: usize,
}

impl From<String> for FfiString {
    fn from(value: String) -> Self {
        let mut value = value;
        let (ptr, len, cap) = unsafe { value.into_raw_parts() };
        Self { ptr, len, cap }
    }
}

impl TryFrom<FfiString> for String {
    type Error = String;

    fn try_from(value: FfiString) -> Result<Self, Self::Error> {
        if value.ptr.is_null() {
            return Err(format!("Pointer was Null."));
        }

        Ok(unsafe { String::from_raw_parts(value.ptr, value.len, value.cap) })
    }
}

impl Drop for FfiString {
    fn drop(&mut self) {
        panic!("FfiString Droped while value still leaked. Use FfiString::try_into::<String>()")
    }
}
