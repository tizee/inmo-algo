use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::{
    ffi::{CString, OsString},
    os::unix::ffi::OsStringExt,
};

use libc::c_char;

// Open template with $EDITOR

struct LibCUtil;

/// wrappers around libc for unix like systems
impl LibCUtil {
    pub fn execvp(cmd: Vec<OsString>) {
        let cstrs: Vec<CString> = cmd
            .into_iter()
            .map(|arg| -> CString { unsafe { CString::from_vec_unchecked(arg.into_vec()) } })
            .collect();
        let args: Vec<*const c_char> = cstrs
            .iter()
            .map(|c| c.as_ptr())
            .chain(Some(std::ptr::null()))
            .collect();

        unsafe { libc::execvp(args[0], args.as_ptr()) };
    }
}

const DEFAULT_EDITOR: &str = "vim";
fn get_editor() -> OsString {
    OsString::from(env::var("EDITOR").unwrap_or_else(|_| DEFAULT_EDITOR.to_string()))
}

pub fn open_with_editor<P: AsRef<Path>>(files: &[P]) {
    let editor = get_editor();
    let mut args = vec![editor, OsStr::new("-p").into()];
    for p in files.iter() {
        let path = p.as_ref();
        args.push(OsString::from(path));
    }
    LibCUtil::execvp(args);
}
