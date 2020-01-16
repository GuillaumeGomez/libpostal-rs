use std::marker::PhantomData;

use sys;
use traits::{ToC, ToRust};

use AddressParserOptions;

pub struct AddressParser {
    pub(crate) inner: PhantomData<u32>,
}

impl Drop for AddressParser {
    fn drop(&mut self) {
        unsafe { sys::libpostal_teardown_parser() }
    }
}

impl AddressParser {
    pub fn get_default_options(&self) -> AddressParserOptions {
        unsafe { sys::libpostal_get_address_parser_default_options() }.to_rust()
    }

    pub fn parse_address(
        &self,
        address: &str,
        options: &AddressParserOptions,
    ) -> Option<Vec<(String, String)>> {
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
                    ret.push((
                        (*data.components.offset(i as _)).to_rust(),
                        (*data.labels.offset(i as _)).to_rust(),
                    ));
                }
            }
            sys::libpostal_address_parser_response_destroy(data);
            Some(ret)
        }
    }
}
