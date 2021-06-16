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
    use std::io::ErrorKind;
    use std::env::temp_dir;
    use std::string::String;

    fn safely_remove_file_if_exists(file: &String) {
        match remove_file(file) {
            Ok(()) => (),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => (),
                other_error => {
                    panic!("Problem removing the file: {:?}", other_error)
                }
            }
        }
    }

    #[test]
    fn create_and_remove_file() {

        let rusty_fname = format!(
            "{}/test.gsd",
            temp_dir().into_os_string().into_string().unwrap()
        );
        safely_remove_file_if_exists(&rusty_fname);

        let fname = CString::new(rusty_fname.clone()).expect("CString::new failed");
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
            assert_eq!(res, gsd_error_GSD_SUCCESS) // checks that file was created without issue
        }

        safely_remove_file_if_exists(&rusty_fname)
    }

    #[test]
    fn write_and_read_file() {
        
    }

}