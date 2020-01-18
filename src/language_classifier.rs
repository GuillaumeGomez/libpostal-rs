use std::ffi::CString;
use std::path::Path;
use std::sync::{Arc, Mutex};

use sys;
use traits::{ToC, ToRust};

use Address;
use Core;
use DuplicateOptions;
use DuplicateStatus;
use FuzzyDuplicateOptions;
use FuzzyDuplicateStatus;
use NearDupeHashOptions;

static INIT_LANGUAGE_CLASSIFIER: once_cell::sync::Lazy<Arc<Mutex<(usize, Option<CString>)>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new((0, None))));

pub struct LanguageClassifier<'a> {
    #[allow(dead_code)]
    inner: &'a Core,
}

impl<'a> Drop for LanguageClassifier<'a> {
    fn drop(&mut self) {
        if let Ok(ref mut x) = INIT_LANGUAGE_CLASSIFIER.lock() {
            if (**x).0 == 1 {
                unsafe { sys::libpostal_teardown_language_classifier() }
                (**x).1.take();
            }
            (**x).0 -= 1;
        }
    }
}

impl<'a> LanguageClassifier<'a> {
    pub(crate) fn new(core: &'a Core) -> Option<LanguageClassifier<'a>> {
        if let Ok(ref mut x) = INIT_LANGUAGE_CLASSIFIER.lock() {
            if (**x).0 == 0 {
                if unsafe { sys::libpostal_setup_language_classifier() }.to_rust() {
                    (**x).0 += 1;
                    return Some(LanguageClassifier { inner: core });
                }
            } else {
                (**x).0 += 1;
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
            if (**x).0 == 0 {
                let datadir = datadir.as_ref();
                let c = datadir.to_c();
                if unsafe { sys::libpostal_setup_language_classifier_datadir(c.as_ptr()) }.to_rust()
                {
                    (**x).0 += 1;
                    (**x).1 = Some(c);
                    return Some(LanguageClassifier { inner: core });
                }
            } else {
                (**x).0 += 1;
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

    pub fn get_default_duplicate_options(&self) -> DuplicateOptions {
        unsafe { sys::libpostal_get_default_duplicate_options() }.to_rust()
    }

    pub fn get_duplicate_options_with_languages(&self, languages: &[String]) -> DuplicateOptions {
        let (_, languages) = languages.to_c();
        unsafe {
            sys::libpostal_get_duplicate_options_with_languages(
                languages.len() as _,
                languages.as_ptr(),
            )
        }
        .to_rust()
    }

    pub fn is_toponym_duplicate(
        &self,
        addresses1: &[Address],
        addresses2: &[Address],
        options: &DuplicateOptions,
    ) -> DuplicateStatus {
        let (labels1, values1) = addresses1.to_c();
        let (labels2, values2) = addresses2.to_c();
        let (_, _, options) = options.to_c();

        unsafe {
            sys::libpostal_is_toponym_duplicate(
                addresses1.len() as _,
                labels1.as_ptr(),
                values1.as_ptr(),
                addresses2.len(),
                labels2.as_ptr(),
                values2.as_ptr(),
                options,
            )
        }
        .to_rust()
    }

    pub fn is_name_duplicate(
        &self,
        value1: &str,
        value2: &str,
        options: &DuplicateOptions,
    ) -> DuplicateStatus {
        let value1 = value1.to_c();
        let value2 = value2.to_c();
        let (_, _, options) = options.to_c();

        unsafe { sys::libpostal_is_name_duplicate(value1.as_ptr(), value2.as_ptr(), options) }
            .to_rust()
    }

    pub fn is_street_duplicate(
        &self,
        value1: &str,
        value2: &str,
        options: &DuplicateOptions,
    ) -> DuplicateStatus {
        let value1 = value1.to_c();
        let value2 = value2.to_c();
        let (_, _, options) = options.to_c();

        unsafe { sys::libpostal_is_street_duplicate(value1.as_ptr(), value2.as_ptr(), options) }
            .to_rust()
    }

    pub fn is_house_number_duplicate(
        &self,
        value1: &str,
        value2: &str,
        options: &DuplicateOptions,
    ) -> DuplicateStatus {
        let value1 = value1.to_c();
        let value2 = value2.to_c();
        let (_, _, options) = options.to_c();

        unsafe {
            sys::libpostal_is_house_number_duplicate(value1.as_ptr(), value2.as_ptr(), options)
        }
        .to_rust()
    }

    pub fn is_po_box_duplicate(
        &self,
        value1: &str,
        value2: &str,
        options: &DuplicateOptions,
    ) -> DuplicateStatus {
        let value1 = value1.to_c();
        let value2 = value2.to_c();
        let (_, _, options) = options.to_c();

        unsafe { sys::libpostal_is_po_box_duplicate(value1.as_ptr(), value2.as_ptr(), options) }
            .to_rust()
    }

    pub fn is_unit_duplicate(
        &self,
        value1: &str,
        value2: &str,
        options: &DuplicateOptions,
    ) -> DuplicateStatus {
        let value1 = value1.to_c();
        let value2 = value2.to_c();
        let (_, _, options) = options.to_c();

        unsafe { sys::libpostal_is_unit_duplicate(value1.as_ptr(), value2.as_ptr(), options) }
            .to_rust()
    }

    pub fn is_floor_duplicate(
        &self,
        value1: &str,
        value2: &str,
        options: &DuplicateOptions,
    ) -> DuplicateStatus {
        let value1 = value1.to_c();
        let value2 = value2.to_c();
        let (_, _, options) = options.to_c();

        unsafe { sys::libpostal_is_floor_duplicate(value1.as_ptr(), value2.as_ptr(), options) }
            .to_rust()
    }

    pub fn is_postal_code_duplicate(
        &self,
        value1: &str,
        value2: &str,
        options: &DuplicateOptions,
    ) -> DuplicateStatus {
        let value1 = value1.to_c();
        let value2 = value2.to_c();
        let (_, _, options) = options.to_c();

        unsafe {
            sys::libpostal_is_postal_code_duplicate(value1.as_ptr(), value2.as_ptr(), options)
        }
        .to_rust()
    }

    pub fn get_default_fuzzy_duplicate_options(&self) -> FuzzyDuplicateOptions {
        unsafe { sys::libpostal_get_default_fuzzy_duplicate_options() }.to_rust()
    }

    pub fn get_default_fuzzy_duplicate_options_with_languages(
        &self,
        languages: &[String],
    ) -> FuzzyDuplicateOptions {
        let (_, languages) = languages.to_c();
        unsafe {
            sys::libpostal_get_default_fuzzy_duplicate_options_with_languages(
                languages.len() as _,
                languages.as_ptr(),
            )
        }
        .to_rust()
    }

    pub fn is_name_duplicate_fuzzy(
        &self,
        values1: &[(String, f64)],
        values2: &[(String, f64)],
        options: &FuzzyDuplicateOptions,
    ) -> FuzzyDuplicateStatus {
        let converter = |v: &[(String, f64)]| {
            let mut labels = Vec::with_capacity(v.len());
            let mut c_labels = Vec::with_capacity(v.len());
            let mut floats = Vec::with_capacity(v.len());

            for (l, f) in v.iter() {
                let label = l.to_c();
                c_labels.push(label.as_ptr());
                labels.push(label);
                floats.push(*f);
            }
            (labels, c_labels, floats)
        };
        let (_, labels1, v1) = converter(values1);
        let (_, labels2, v2) = converter(values2);
        let (_, _, options) = options.to_c();

        unsafe {
            sys::libpostal_is_name_duplicate_fuzzy(
                values1.len() as _,
                labels1.as_ptr(),
                v1.as_ptr(),
                values2.len(),
                labels2.as_ptr(),
                v2.as_ptr(),
                options,
            )
        }
        .to_rust()
    }
}
