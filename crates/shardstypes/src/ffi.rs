#[repr(C)]
pub struct FfiString {
    pub ptr: *mut u8,
    pub len: usize,
}

impl From<String> for FfiString {
    fn from(value: String) -> Self {
        let mut value = value;
        value.shrink_to_fit();
        let buf = value.leak();
        let ptr = buf.as_mut_ptr();
        let len = buf.len();
        Self { ptr, len }
    }
}

impl TryFrom<FfiString> for String {
    type Error = String;

    fn try_from(value: FfiString) -> Result<Self, Self::Error> {
        if value.ptr.is_null() {
            return Err(format!("Pointer was Null."));
        }

        Ok(unsafe { String::from_raw_parts(value.ptr, value.len, value.len) })
    }
}

impl Drop for FfiString {
    fn drop(&mut self) {
        panic!("FfiString Droped while value still leaked. Use FfiString::try_into::<String>()")
    }
}
