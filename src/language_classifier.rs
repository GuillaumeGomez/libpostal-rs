use std::path::Path;
use std::sync::{Arc, Mutex};
use sys;
use traits::{ToC, ToRust};

use Address;
use Core;
use DuplicateOptions;
use NearDupeHashOptions;

static INIT_LANGUAGE_CLASSIFIER: once_cell::sync::Lazy<Arc<Mutex<usize>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(0)));

pub struct LanguageClassifier<'a> {
    #[allow(dead_code)]
    inner: &'a Core,
}

impl<'a> LanguageClassifier<'a> {
    pub(crate) fn new(core: &'a Core) -> Option<LanguageClassifier<'a>> {
        if let Ok(ref mut x) = INIT_LANGUAGE_CLASSIFIER.lock() {
            if **x == 0 {
                if unsafe { sys::libpostal_setup_language_classifier() }.to_rust() {
                    **x += 1;
                    return Some(LanguageClassifier { inner: core });
                }
            } else {
                **x += 1;
                return Some(LanguageClassifier { inner: core });
            }
        }
        None
    }

    pub(crate) fn new_datadir<P: AsRef<Path>>(
        core: &'a Core,
        datadir: P,
    ) -> Option<LanguageClassifier<'a>> {
        if let Ok(ref mut x) = INIT_LANGUAGE_CLASSIFIER.lock() {
            if **x == 0 {
                let datadir = datadir.as_ref();
                let c = datadir.to_c();
                if unsafe { sys::libpostal_setup_language_classifier_datadir(c.as_ptr()) }.to_rust()
                {
                    **x += 1;
                    return Some(LanguageClassifier { inner: core });
                }
            } else {
                **x += 1;
                return Some(LanguageClassifier { inner: core });
            }
        }
        None
    }

    pub fn get_near_dupe_hash_default_options(&self) -> NearDupeHashOptions {
        unsafe { sys::libpostal_get_near_dupe_hash_default_options() }.to_rust()
    }

    pub fn near_dupe_hashes(
        &self,
        addresses: &[Address],
        options: &NearDupeHashOptions,
    ) -> Vec<String> {
        let (labels, values) = addresses.to_c();
        let mut num_hashes = 0;

        let ptr = unsafe {
            sys::libpostal_near_dupe_hashes(
                addresses.len(),
                labels.as_ptr(),
                values.as_ptr(),
                options.to_c(),
                &mut num_hashes,
            )
        };
        if !ptr.is_null() && num_hashes > 0 {
            let mut ret = Vec::with_capacity(num_hashes);
            for i in 0..num_hashes {
                ret.push(unsafe { (*ptr.offset(i as _)).to_rust() });
            }
            unsafe {
                sys::libpostal_expansion_array_destroy(ptr, num_hashes);
            }
            ret
        } else {
            Vec::new()
        }
    }

    pub fn near_dupe_hashes_languages(
        &self,
        addresses: &[Address],
        options: &NearDupeHashOptions,
        languages: &[String],
    ) -> Vec<String> {
        let (labels, values) = addresses.to_c();
        let mut num_hashes = 0;
        let (_, languages) = languages.to_c();

        let ptr = unsafe {
            sys::libpostal_near_dupe_hashes_languages(
                addresses.len(),
                labels.as_ptr(),
                values.as_ptr(),
                options.to_c(),
                languages.len(),
                languages.as_ptr(),
                &mut num_hashes,
            )
        };
        if !ptr.is_null() && num_hashes > 0 {
            let mut ret = Vec::with_capacity(num_hashes);
            for i in 0..num_hashes {
                ret.push(unsafe { (*ptr.offset(i as _)).to_rust() });
            }
            unsafe {
                sys::libpostal_expansion_array_destroy(ptr, num_hashes);
            }
            ret
        } else {
            Vec::new()
        }
    }
}

impl<'a> Drop for LanguageClassifier<'a> {
    fn drop(&mut self) {
        if let Ok(ref mut x) = INIT_LANGUAGE_CLASSIFIER.lock() {
            if **x == 1 {
                unsafe { sys::libpostal_teardown_language_classifier() }
            }
            **x -= 1;
        }
    }
}
