use std::marker::PhantomData;

use sys;
use traits::ToRust;

pub struct AddressParser {
    pub(crate) inner: PhantomData<u32>,
}

impl Drop for AddressParser {
    fn drop(&mut self) {
        unsafe { sys::libpostal_teardown_parser() }
    }
}
