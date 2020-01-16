use std::ffi::{CStr, CString};
use std::path::Path;

use libc::c_char;

use sys;

pub(crate) trait ToRust {
    type Out;

    fn to_rust(&self) -> Self::Out;
}

impl ToRust for sys::c_bool {
    type Out = bool;

    #[inline]
    fn to_rust(&self) -> Self::Out {
        *self != 0
    }
}

impl ToRust for *const c_char {
    type Out = String;

    #[inline]
    fn to_rust(&self) -> Self::Out {
        if self.is_null() {
            return String::new();
        }
        String::from_utf8_lossy(unsafe { CStr::from_ptr(*self) }.to_bytes()).into_owned()
    }
}

impl ToRust for *mut c_char {
    type Out = String;

    #[inline]
    fn to_rust(&self) -> Self::Out {
        if self.is_null() {
            return String::new();
        }
        String::from_utf8_lossy(unsafe { CStr::from_ptr(*self as *const c_char) }.to_bytes())
            .into_owned()
    }
}

pub(crate) trait ToC {
    type Out;

    fn to_c(&self) -> Self::Out;
}

impl ToC for bool {
    type Out = sys::c_bool;

    #[inline]
    fn to_c(&self) -> Self::Out {
        if *self {
            1
        } else {
            0
        }
    }
}

impl ToC for Path {
    type Out = CString;

    #[cfg(not(windows))]
    #[inline]
    fn to_c(&self) -> Self::Out {
        use std::os::unix::ffi::OsStrExt;

        CString::new(self.as_os_str().as_bytes()).expect("Invalid path with '\0' bytes")
    }

    #[cfg(windows)]
    #[inline]
    fn to_c(path: &Path) -> CString {
        let path_str = self
            .to_str()
            .expect("Path can't be represented as UTF-8")
            .to_owned();

        if path_str.starts_with("\\\\?\\") {
            CString::new(path_str[4..].as_bytes())
        } else {
            CString::new(path_str.as_bytes())
        }
        .expect("Invalid path with '\0' bytes")
    }
}

impl ToC for str {
    type Out = CString;

    #[inline]
    fn to_c(&self) -> Self::Out {
        CString::new(self).expect("Invalid string with '\0' byte")
    }
}

impl<'a> ToC for &'a [String] {
    type Out = (Vec<CString>, Vec<*const c_char>);

    #[inline]
    fn to_c(&self) -> Self::Out {
        let mut strs = Vec::with_capacity(self.len());
        let mut c_strs = Vec::with_capacity(self.len());

        for s in self.iter() {
            let cs = CString::new(s.as_str()).expect("unexpected '\0' in string");
            let ptr = cs.as_ptr();

            strs.push(cs);
            c_strs.push(ptr);
        }
        (strs, c_strs)
    }
}
