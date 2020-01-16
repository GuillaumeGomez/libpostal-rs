use sys;
use traits::ToRust;

use Core;

pub struct LanguageClassifier<'a> {
    #[allow(dead_code)]
    pub(crate) inner: &'a Core,
}

impl<'a> Drop for LanguageClassifier<'a> {
    fn drop(&mut self) {
        unsafe { sys::libpostal_teardown_language_classifier() }
    }
}
