#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(deref_nullptr)]

include!(concat!(env!("OUT_DIR"), concat!("/", "bindings", ".rs")));
mod bindings;
pub use crate::bindings::*;

#[cfg(test)]
mod tests {
    use crate::bindings as gsd;
    use std::ffi::CString;
    use std::fs::remove_file;  
    use std::io::ErrorKind;
    use std::env::temp_dir;
    use std::string::String;
    use std::os::raw::c_void;
    use std::mem;
    use rand::prelude::*;

    fn get_test_file_name() -> String {
        let rusty_fname = format!(
            "{}/test.gsd",
            temp_dir().into_os_string().into_string().unwrap()
        );
        rusty_fname
    }

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

        let rusty_fname = get_test_file_name();

        safely_remove_file_if_exists(&rusty_fname);

        let fname = CString::new(rusty_fname.clone()).expect("CString::new failed");
        let app = CString::new("gsd-sys").expect("CString::new failed");
        let schema = CString::new("test").expect("CString::new failed");
        let schema_version: u32 = 0;

        unsafe {
            let res = gsd::gsd_create(
                fname.as_ptr(),
                app.as_ptr(),
                schema.as_ptr(),
                schema_version
            );
            assert_eq!(res, gsd::gsd_error_GSD_SUCCESS); // checks that file was created without issue
        }

        safely_remove_file_if_exists(&rusty_fname)
    }

    #[test]
    fn write_and_read_file() {
        
        let rusty_fname = get_test_file_name();

        safely_remove_file_if_exists(&rusty_fname);

        let fname = CString::new(rusty_fname.clone()).expect("CString::new failed");
        let app = CString::new("gsd-sys").expect("CString::new failed");
        let schema = CString::new("test").expect("CString::new failed");
        let schema_version: u32 = 0;

        let chunk_name = CString::new("chunk").unwrap();
        const data: [f64; 4] = [0.1, 0.2, 0.3, 0.4]; // just some test data
        let raw_data = Box::into_raw(Box::new(data)) as *const c_void;

        unsafe {

            let mut file_handle: gsd::gsd_handle = Default::default();
            let raw_file_handle = &mut file_handle as *mut gsd::gsd_handle;

            // create and open file handle
            let res = gsd::gsd_create_and_open(
                raw_file_handle,
                fname.as_ptr(),
                app.as_ptr(),
                schema.as_ptr(),
                schema_version,
                gsd::gsd_open_flag_GSD_OPEN_READWRITE,
                0
            );
            assert_eq!(res, gsd::gsd_error_GSD_SUCCESS); // checks that file was created without issue

            // write a chunk of data
            let res = gsd::gsd_write_chunk(
                raw_file_handle, 
                chunk_name.as_ptr(), 
                gsd::gsd_type_GSD_TYPE_DOUBLE, 
                2, 
                2, 
                0, 
                raw_data
            );
            assert_eq!(res, gsd::gsd_error_GSD_SUCCESS); // checks that the data was successfully written

            let res = gsd::gsd_end_frame(raw_file_handle);
            assert_eq!(res, gsd::gsd_error_GSD_SUCCESS);

            let entry = gsd::gsd_find_chunk(
                raw_file_handle,
                0,
                chunk_name.as_ptr()
            );

            let new_data: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
            let new_raw_data = Box::into_raw(Box::new(new_data)) as *mut c_void;

            let res = gsd::gsd_read_chunk(
                raw_file_handle,
                new_raw_data,
                entry
            );
            assert_eq!(res, gsd::gsd_error_GSD_SUCCESS); // check that chunk was found and written to new_raw_data

            let test_1: &[f64; 4] = mem::transmute(raw_data);
            let test_2: &[f64; 4] = mem::transmute(new_raw_data);

            assert_eq!(&test_1, &test_2);
            println!("{:?} == {:?}", &test_1, &test_2);
        }

        safely_remove_file_if_exists(&rusty_fname);
    }

    fn write_and_read_file_random() {

        fn gsd_create_and_open(fname: &String) -> Result<gsd::gsd_handle,()> {

            let fname = CString::new(fname.clone()).expect("CString::new failed");
            let mut file_handle: gsd::gsd_handle = Default::default();
            let raw_file_handle = &mut file_handle as *mut gsd::gsd_handle;
            let app = CString::new("gsd-sys").expect("CString::new failed");
            let schema = CString::new("test").expect("CString::new failed");
            let schema_version: u32 = 0;
            unsafe {
                let res = gsd::gsd_create_and_open(
                    raw_file_handle,
                    fname.as_ptr(),
                    app.as_ptr(),
                    schema.as_ptr(),
                    schema_version,
                    gsd::gsd_open_flag_GSD_OPEN_READWRITE,
                    0
                );
                assert_eq!(res, gsd::gsd_error_GSD_SUCCESS);
            }
            Ok(file_handle)
        }

        fn gsd_write_chunk(
                file_handle: &mut gsd::gsd_handle, 
                chunk_name: &String,
                data: &[&[f64]]
                ) {
            let raw_file_handle = file_handle as *mut gsd::gsd_handle;
            let raw_data = Box::into_raw(Box::new(data)) as *const c_void;
            let N = data.len() as u64;
            let c_chunk = CString::new(chunk_name.clone()).unwrap();
            unsafe {
                let res = gsd::gsd_write_chunk(
                    raw_file_handle, 
                    c_chunk.as_ptr(), 
                    gsd::gsd_type_GSD_TYPE_DOUBLE, 
                    N, 
                    2, 
                    0, 
                    raw_data
                );
                assert_eq!(res, gsd::gsd_error_GSD_SUCCESS); // checks that the data was successfully written
            }
        }

        fn gsd_end_frame(file_handle: &mut gsd::gsd_handle) {
            let raw_file_handle = file_handle as *mut gsd::gsd_handle;
            unsafe {
                let res = gsd::gsd_end_frame(raw_file_handle);
                assert_eq!(res, gsd::gsd_error_GSD_SUCCESS);
            }
        }

        fn gsd_find_chunk(
                file_handle: &mut gsd::gsd_handle, 
                frame: usize, 
                chunk_name: &String) -> *const gsd::gsd_index_entry {
            let raw_file_handle = file_handle as *mut gsd::gsd_handle;
            let c_chunk_name = CString::new(chunk_name.clone()).unwrap();
            unsafe {
                let entry = gsd::gsd_find_chunk(
                    raw_file_handle,
                    frame as u64,
                    c_chunk_name.as_ptr()
                );
                assert_ne!(entry, std::ptr::null());
                entry
            }
        }

        fn gsd_read_chunk(
                file_handle: &mut gsd::gsd_handle, 
                data: &[&[f64]],
                entry: *const gsd::gsd_index_entry) {
            let raw_file_handle = file_handle as *mut gsd::gsd_handle;
            let raw_data = Box::into_raw(Box::new(data)) as *mut c_void;
            unsafe {
                let res = gsd::gsd_read_chunk(
                    raw_file_handle,
                    raw_data,
                    entry
                );
                assert_eq!(res, gsd::gsd_error_GSD_SUCCESS);
            }
        }

        struct Snapshot<const N: usize> {
            time: usize,
            positions: [[f64; 3]; N],
            velocities: [[f64; 3]; N]
        }

        impl<const N: usize> Snapshot<N> {
            fn new_rand(time: usize, rng: &mut ThreadRng) -> Self {
                let mut pos = [[0.0f64; 3]; N];
                let mut vel = [[0.0f64; 3]; N];
                for i in 0..N {
                    for j in 0..3 {
                        pos[i][j] = rng.gen();
                        vel[i][j] = rng.gen();
                    }
                }
                Self {
                    time: time.clone(),
                    positions: pos,
                    velocities: vel
                }
            }
        }

        struct Trajectory<const N: usize> {
            current_time: usize,
            rng: ThreadRng,
            frames: Vec<Snapshot<N>>
        }

        impl<const N: usize> Trajectory<N> {

            fn new() -> Self {
                Self {
                    current_time: 0,
                    rng: rand::thread_rng(),
                    frames: Vec::<Snapshot<N>>::new()
                }
            }

            fn progress_fake_sim_and_add_frame(&mut self) {
                self.current_time += 1000;
                let snap = Snapshot::<N>::new_rand(self.current_time, &mut self.rng);
                self.frames.push(snap);
            }

            fn run_fake_sim(&mut self) {
                for _ in 0..100 {
                    self.progress_fake_sim_and_add_frame();
                }
            }

            fn dump_to_gsd(&self, handle: &mut gsd::gsd_handle) {
                
                todo!();
                
            }

            fn new_from_gsd(handle: &mut gsd::gsd_handle) -> Self {
                
                todo!()
                
            }
        }

        let mut traj = Trajectory::<10>::new();
        traj.run_fake_sim();
        
        let rusty_fname = get_test_file_name();

        safely_remove_file_if_exists(&rusty_fname);

        let mut file_handle = gsd_create_and_open(&rusty_fname).unwrap();

        todo!();

        safely_remove_file_if_exists(&rusty_fname);
    }


}