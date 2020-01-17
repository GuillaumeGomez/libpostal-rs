use sys;
use traits::{ToC, ToRust};

use Address;
use AddressParserOptions;
use Core;

use std::ffi::CString;
use std::path::Path;
use std::sync::{Arc, Mutex};

static INIT_ADDRESS_PARSER: once_cell::sync::Lazy<Arc<Mutex<usize>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(0)));

pub struct AddressParser<'a> {
    #[allow(dead_code)]
    inner: &'a Core,
}

impl<'a> Drop for AddressParser<'a> {
    fn drop(&mut self) {
        if let Ok(ref mut x) = INIT_ADDRESS_PARSER.lock() {
            if **x == 1 {
                unsafe { sys::libpostal_teardown_parser() }
            }
            **x -= 1;
        }
    }
}

impl<'a> AddressParser<'a> {
    pub(crate) fn new(core: &'a Core) -> Option<AddressParser<'a>> {
        if let Ok(ref mut x) = INIT_ADDRESS_PARSER.lock() {
            if **x == 0 {
                if unsafe { sys::libpostal_setup_parser() }.to_rust() {
                    **x += 1;
                    return Some(AddressParser { inner: core });
                }
            } else {
                **x += 1;
                return Some(AddressParser { inner: core });
            }
        }
        None
    }

    pub(crate) fn new_datadir<P: AsRef<Path>>(
        core: &'a Core,
        datadir: P,
    ) -> Option<AddressParser<'a>> {
        if let Ok(ref mut x) = INIT_ADDRESS_PARSER.lock() {
            if **x == 0 {
                let datadir = datadir.as_ref();
                let c = datadir.to_c();
                if unsafe { sys::libpostal_setup_parser_datadir(c.as_ptr()) }.to_rust() {
                    **x += 1;
                    return Some(AddressParser { inner: core });
                }
            } else {
                **x += 1;
                return Some(AddressParser { inner: core });
            }
        }
        None
    }

    pub fn get_default_options(&self) -> AddressParserOptions {
        unsafe { sys::libpostal_get_address_parser_default_options() }.to_rust()
    }

    pub fn parse_address(
        &self,
        address: &str,
        options: &AddressParserOptions,
    ) -> Option<Vec<Address>> {
        let address = address.to_c();
        let (_, options) = options.to_c();

        let data = unsafe { sys::libpostal_parse_address(address.as_ptr(), options) };
        if data.is_null() {
            return None;
        }
        unsafe {
            let mut ret = Vec::with_capacity((*data).num_components);
            {
                let data = &*data;

                for i in 0..data.num_components {
                    ret.push(Address {
                        label: CString::new((*data.labels.offset(i as _)).to_rust())
                            .expect("unexpected '\0' in label"),
                        value: CString::new((*data.components.offset(i as _)).to_rust())
                            .expect("unexpected '\0' in value"),
                    });
                }
            }
            sys::libpostal_address_parser_response_destroy(data);
            Some(ret)
        }
    }
}
