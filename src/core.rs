use std::marker::PhantomData;
use std::path::Path;

use sys;
use traits::{ToC, ToRust};
use utils::ptr_to_rust;

use AddressParser;
use LanguageClassifier;
use NormalizeOptions;

pub struct Core {
    inner: PhantomData<u32>,
}

impl Core {
    pub fn setup() -> Option<Core> {
        if unsafe { sys::libpostal_setup() }.to_rust() {
            Some(Core { inner: PhantomData })
        } else {
            None
        }
    }

    pub fn setup_datadir<P: AsRef<Path>>(datadir: P) -> Option<Core> {
        let datadir = datadir.as_ref();
        let c = datadir.to_c();
        if unsafe { sys::libpostal_setup_datadir(c.as_ptr()) }.to_rust() {
            Some(Core { inner: PhantomData })
        } else {
            None
        }
    }

    pub fn setup_parser<'a>(&'a self) -> Option<AddressParser<'a>> {
        if unsafe { sys::libpostal_setup_parser() }.to_rust() {
            Some(AddressParser { inner: self })
        } else {
            None
        }
    }

    pub fn setup_parser_datadir<'a, P: AsRef<Path>>(
        &'a self,
        datadir: P,
    ) -> Option<AddressParser<'a>> {
        let datadir = datadir.as_ref();
        let c = datadir.to_c();
        if unsafe { sys::libpostal_setup_parser_datadir(c.as_ptr()) }.to_rust() {
            Some(AddressParser { inner: self })
        } else {
            None
        }
    }

    pub fn setup_language_classifier<'a>(&'a self) -> Option<LanguageClassifier<'a>> {
        if unsafe { sys::libpostal_setup_language_classifier() }.to_rust() {
            Some(LanguageClassifier { inner: self })
        } else {
            None
        }
    }

    pub fn setup_language_classifier_datadir<'a, P: AsRef<Path>>(
        &'a self,
        datadir: P,
    ) -> Option<LanguageClassifier<'a>> {
        let datadir = datadir.as_ref();
        let c = datadir.to_c();
        if unsafe { sys::libpostal_setup_language_classifier_datadir(c.as_ptr()) }.to_rust() {
            Some(LanguageClassifier { inner: self })
        } else {
            None
        }
    }

    pub fn get_default_options(&self) -> NormalizeOptions {
        unsafe { sys::libpostal_get_default_options() }.to_rust()
    }

    pub fn expand_address(&self, input: &str, options: NormalizeOptions) -> Vec<String> {
        let input = input.to_c();
        let (_, options) = options.to_c();
        let mut size = 0;

        let ptr = unsafe { sys::libpostal_expand_address(input.as_ptr(), options, &mut size) };
        let ret = ptr_to_rust(ptr, size);
        // Apparently we have to free memory of a char** using THIS function so let's go...
        unsafe {
            sys::libpostal_expansion_array_destroy(ptr, size);
        }
        ret
    }

    pub fn expand_address_root(&self, input: &str, options: NormalizeOptions) -> Vec<String> {
        let input = input.to_c();
        let (_, options) = options.to_c();
        let mut size = 0;

        let ptr = unsafe { sys::libpostal_expand_address_root(input.as_ptr(), options, &mut size) };
        let ret = ptr_to_rust(ptr, size);
        // Apparently we have to free memory of a char** using THIS function so let's go...
        unsafe {
            sys::libpostal_expansion_array_destroy(ptr, size);
        }
        ret
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        unsafe { sys::libpostal_teardown() }
    }
}
