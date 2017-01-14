use std::path::Path;
use std::ffi::{CString};
use std::os::unix::ffi::OsStrExt;
use libc::{self, uid_t, gid_t, c_char};
use std::io;
use rand::{thread_rng, Rng};

pub fn generate_password() -> String {
    thread_rng().gen_ascii_chars().take(16).collect::<String>()
}

fn get_gid_by_name(name: &str) -> Option<gid_t> {
    unsafe {
        let ptr = libc::getgrnam(CString::new(name).unwrap().as_ptr() as *const c_char);
        if ptr.is_null() {
            None
        } else {
            let s = &*ptr;
            Some(s.gr_gid)
        }
    }
}

fn get_uid_by_name(name: &str) -> Option<uid_t> {
    unsafe {
        let ptr = libc::getpwnam(CString::new(name).unwrap().as_ptr() as *const c_char);
        if ptr.is_null() {
            None
        } else {
            let s = &*ptr;
            Some(s.pw_uid)
        }
    }
}

pub fn chown_by_name(path: &Path, owner: &str, group: &str) -> Result<(), io::Error> {
    let uid = get_uid_by_name(owner);
    let gid = get_gid_by_name(group);

    if uid.is_none() || gid.is_none() {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Failed to get uid or gid."))
    } else {
        chown(&path, uid, gid)
    }
}

pub fn chown(path: &Path, owner: Option<uid_t>, group: Option<gid_t>) -> Result<(), io::Error> {

    let cstr = CString::new(path.as_os_str().as_bytes()).unwrap();
    let res = unsafe {
        libc::chown(cstr.as_ptr(),
                    owner.unwrap_or((0 as uid_t).wrapping_sub(1)),
                    group.unwrap_or((0 as gid_t).wrapping_sub(1)))
    };

    if res != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
