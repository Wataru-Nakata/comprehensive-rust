// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

mod ffi {
    use std::os::raw::{c_char, c_int, c_long, c_ulong, c_ushort};

    // Opaque type. See https://doc.rust-lang.org/nomicon/ffi.html.
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // Layout as per readdir(3) and definitions in /usr/include/x86_64-linux-gnu.
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_long,
        pub d_off: c_ulong,
        pub d_reclen: c_ushort,
        pub d_type: c_char,
        pub d_name: [c_char; 256],
    }

    extern "C" {
        pub fn opendir(s: *const c_char) -> *mut DIR;
        pub fn readdir(s: *mut DIR) -> *const dirent;
        pub fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{c_char, CStr, CString, OsStr, OsString};
use std::fs::read_dir;
use std::os::unix::ffi::OsStrExt;
use std::thread::panicking;

use ffi::{opendir, readdir, closedir};

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // Call opendir and return a Ok value if that worked,
        // otherwise return Err with a message.
        let c_str_path = CString::new(path).map_err(|err| format!("Invalid path: {err}"))?;
        let dir = unsafe { opendir(c_str_path.as_ptr() as *const c_char) };
        if dir.is_null() {
            return Err("error".to_string());
        } else {
            return Ok(Self {
                path: c_str_path,
                dir,
            });
        }
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // Keep calling readdir until we get a NULL pointer back.
        let result = unsafe{readdir(self.dir)};
        if result.is_null() {
            return None;
        } else {
            let d_name = unsafe{CStr::from_ptr((*result).d_name.as_ptr())};
            let os_str = OsStr::from_bytes(d_name.to_bytes());
            Some(os_str.to_owned())
        }
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // Call closedir as needed.
        if !self.dir.is_null() {
            if unsafe{closedir(self.dir)} != 0 {
                panic!("Could not close {:?}",self.path);
            }
        }
    }
}

fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("files: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}
