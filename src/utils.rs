use traits::ToRust;

use libc::c_char;

pub(crate) fn ptr_to_rust(ptr: *mut *mut c_char, len: usize) -> Vec<String> {
    if ptr.is_null() || len == 0 {
        return Vec::new();
    }
    let mut ret = Vec::with_capacity(len);

    unsafe {
        for pos in 0..len {
            let elem = ptr.offset(pos as _);
            ret.push((*elem).to_rust());
        }
    }
    ret
}
