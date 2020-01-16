use std::marker::PhantomData;

use sys;
use traits::ToRust;

pub struct LanguageClassifier {
    pub(crate) inner: PhantomData<u32>,
}

impl Drop for LanguageClassifier {
    fn drop(&mut self) {
        unsafe { sys::libpostal_teardown_language_classifier() }
    }
}
