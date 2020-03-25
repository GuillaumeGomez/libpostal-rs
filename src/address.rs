use traits::ToC;

use std::ffi::CString;

use libc::c_char;

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Address {
    pub label: CString,
    pub value: CString,
}

impl ToC for Address {
    type Out = (*const c_char, *const c_char);

    fn to_c(&self) -> Self::Out {
        (self.label.as_ptr(), self.value.as_ptr())
    }
}

impl<'a> ToC for &'a [Address] {
    type Out = (Vec<*const c_char>, Vec<*const c_char>);

    fn to_c(&self) -> Self::Out {
        let mut labels = Vec::with_capacity(self.len());
        let mut values = Vec::with_capacity(self.len());

        for addr in self.iter() {
            labels.push(addr.label.as_ptr());
            values.push(addr.value.as_ptr());
        }
        (labels, values)
    }
}
