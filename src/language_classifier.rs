use sys;
use traits::{ToC, ToRust};

use Address;
use Core;
use NearDupeHashOptions;

pub struct LanguageClassifier<'a> {
    #[allow(dead_code)]
    pub(crate) inner: &'a Core,
}

impl<'a> LanguageClassifier<'a> {
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
        let mut ret = Vec::with_capacity(num_hashes);
        if !ptr.is_null() && num_hashes > 0 {
            for i in 0..num_hashes {
                ret.push(unsafe { (*ptr.offset(i as _)).to_rust() });
            }
            unsafe {
                sys::libpostal_expansion_array_destroy(ptr, num_hashes);
            }
        }
        ret
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
        let mut ret = Vec::with_capacity(num_hashes);
        if !ptr.is_null() && num_hashes > 0 {
            for i in 0..num_hashes {
                ret.push(unsafe { (*ptr.offset(i as _)).to_rust() });
            }
            unsafe {
                sys::libpostal_expansion_array_destroy(ptr, num_hashes);
            }
        }
        ret
    }
}

impl<'a> Drop for LanguageClassifier<'a> {
    fn drop(&mut self) {
        unsafe { sys::libpostal_teardown_language_classifier() }
    }
}
