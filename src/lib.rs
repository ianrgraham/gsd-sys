#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::fs::remove_file;

    #[test]
    fn create_and_remove_file() {

        let rusty_fname = "src/test.gsd";
        let fname = CString::new(rusty_fname).expect("CString::new failed");
        let app = CString::new("gsd-sys").expect("CString::new failed");
        let schema = CString::new("test").expect("CString::new failed");
        let schema_version: u32 = 0;
        unsafe {
            let res = gsd_create(
                fname.as_ptr(),
                app.as_ptr(),
                schema.as_ptr(),
                schema_version
            );
            assert_eq!(res, 0) // checks that file was created without issue
        }

        remove_file(rusty_fname).expect("Failed to remove gsd file");
    }
}