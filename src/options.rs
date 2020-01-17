use std::ffi::CString;

use enums::{AddressComponent, DuplicateStatus};
use sys;
use traits::{ToC, ToRust};

use libc::{c_char, free};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddressComponents {
    inner: u16,
}

impl AddressComponents {
    pub fn new() -> AddressComponents {
        AddressComponents { inner: 0 }
    }

    pub fn add(&mut self, component: AddressComponent) -> &mut AddressComponents {
        self.inner |= component.to_c();
        self
    }

    pub fn remove(&mut self, component: AddressComponent) -> &mut AddressComponents {
        let component = component.to_c();
        self.inner &= !component;
        self
    }

    pub(crate) fn from_c(value: u16) -> AddressComponents {
        AddressComponents { inner: value }
    }
}

impl ToC for AddressComponents {
    type Out = u16;

    fn to_c(&self) -> u16 {
        self.inner
    }
}

// This type is used mostly to not forget to free CString memory once we're done.
pub(crate) struct CStringsWrapper(Vec<*mut c_char>);

impl Drop for CStringsWrapper {
    fn drop(&mut self) {
        for x in self.0.drain(..) {
            let _ = unsafe { CString::from_raw(x) };
        }
    }
}

#[derive(Clone, Debug)]
pub struct NormalizeOptions {
    pub languages: Vec<String>,
    pub address_components: AddressComponents,
    pub latin_ascii: bool,
    pub transliterate: bool,
    pub strip_accents: bool,
    pub decompose: bool,
    pub lowercase: bool,
    pub trim_string: bool,
    pub drop_parentheticals: bool,
    pub replace_numeric_hyphens: bool,
    pub delete_numeric_hyphens: bool,
    pub split_alpha_from_numeric: bool,
    pub replace_word_hyphens: bool,
    pub delete_word_hyphens: bool,
    pub delete_final_periods: bool,
    pub delete_acronym_periods: bool,
    pub drop_english_possessives: bool,
    pub delete_apostrophes: bool,
    pub expand_numex: bool,
    pub roman_numerals: bool,
}

impl ToC for NormalizeOptions {
    type Out = (CStringsWrapper, sys::libpostal_normalize_options_t);

    // To prevent cloning and memory leak, we keep the array of pointers to get it back once we
    // used the struct.
    fn to_c(&self) -> Self::Out {
        let languages = self
            .languages
            .iter()
            .map(|s| {
                CString::new(s.as_str())
                    .expect("CString::new failed")
                    .into_raw()
            })
            .collect::<Vec<_>>();
        let ptr = languages.as_ptr();
        let len = languages.len();

        (
            CStringsWrapper(languages),
            sys::libpostal_normalize_options_t {
                languages: ptr as usize as *mut _,
                num_languages: len,
                address_components: self.address_components.to_c(),
                latin_ascii: self.latin_ascii.to_c(),
                transliterate: self.transliterate.to_c(),
                strip_accents: self.strip_accents.to_c(),
                decompose: self.decompose.to_c(),
                lowercase: self.lowercase.to_c(),
                trim_string: self.trim_string.to_c(),
                drop_parentheticals: self.drop_parentheticals.to_c(),
                replace_numeric_hyphens: self.replace_numeric_hyphens.to_c(),
                delete_numeric_hyphens: self.delete_numeric_hyphens.to_c(),
                split_alpha_from_numeric: self.split_alpha_from_numeric.to_c(),
                replace_word_hyphens: self.replace_word_hyphens.to_c(),
                delete_word_hyphens: self.delete_word_hyphens.to_c(),
                delete_final_periods: self.delete_final_periods.to_c(),
                delete_acronym_periods: self.delete_acronym_periods.to_c(),
                drop_english_possessives: self.drop_english_possessives.to_c(),
                delete_apostrophes: self.delete_apostrophes.to_c(),
                expand_numex: self.expand_numex.to_c(),
                roman_numerals: self.roman_numerals.to_c(),
            },
        )
    }
}

impl ToRust for sys::libpostal_normalize_options_t {
    type Out = NormalizeOptions;

    /// Once this one has been called, `self` isn't usable anymore!
    fn to_rust(&self) -> NormalizeOptions {
        let mut languages = Vec::with_capacity(self.num_languages);

        if !self.languages.is_null() {
            unsafe {
                for pos in 0..self.num_languages {
                    let elem = self.languages.offset(pos as _);
                    languages.push((*elem).to_rust());
                    free(*elem as _);
                }
                free(self.languages as _);
            }
        }

        NormalizeOptions {
            languages,
            address_components: AddressComponents::from_c(self.address_components),
            latin_ascii: self.latin_ascii.to_rust(),
            transliterate: self.transliterate.to_rust(),
            strip_accents: self.strip_accents.to_rust(),
            decompose: self.decompose.to_rust(),
            lowercase: self.lowercase.to_rust(),
            trim_string: self.trim_string.to_rust(),
            drop_parentheticals: self.drop_parentheticals.to_rust(),
            replace_numeric_hyphens: self.replace_numeric_hyphens.to_rust(),
            delete_numeric_hyphens: self.delete_numeric_hyphens.to_rust(),
            split_alpha_from_numeric: self.split_alpha_from_numeric.to_rust(),
            replace_word_hyphens: self.replace_word_hyphens.to_rust(),
            delete_word_hyphens: self.delete_word_hyphens.to_rust(),
            delete_final_periods: self.delete_final_periods.to_rust(),
            delete_acronym_periods: self.delete_acronym_periods.to_rust(),
            drop_english_possessives: self.drop_english_possessives.to_rust(),
            delete_apostrophes: self.delete_apostrophes.to_rust(),
            expand_numex: self.expand_numex.to_rust(),
            roman_numerals: self.roman_numerals.to_rust(),
        }
    }
}

// This type is used mostly to not forget to free CString memory once we're done.
pub(crate) struct CStringWrapper(*mut c_char);

impl Drop for CStringWrapper {
    fn drop(&mut self) {
        unsafe { CString::from_raw(self.0) };
    }
}

#[derive(Clone, Debug)]
pub struct AddressParserOptions {
    language: String,
    country: String,
}

impl ToC for AddressParserOptions {
    type Out = (
        (CStringWrapper, CStringWrapper),
        sys::libpostal_address_parser_options_t,
    );

    #[inline]
    fn to_c(&self) -> Self::Out {
        let language = CString::new(self.language.as_str())
            .expect("CString::new failed")
            .into_raw() as usize;
        let country = CString::new(self.language.as_str())
            .expect("CString::new failed")
            .into_raw() as usize;

        (
            (CStringWrapper(language as _), CStringWrapper(country as _)),
            sys::libpostal_address_parser_options_t {
                language: language as _,
                country: country as _,
            },
        )
    }
}

impl ToRust for sys::libpostal_address_parser_options_t {
    type Out = AddressParserOptions;

    #[inline]
    fn to_rust(&self) -> AddressParserOptions {
        let language = self.language.to_rust();
        let country = self.country.to_rust();

        unsafe {
            free(self.language as _);
            free(self.country as _);
        }
        AddressParserOptions { language, country }
    }
}

#[derive(Clone, Debug)]
pub struct NearDupeHashOptions {
    pub with_name: bool,
    pub with_address: bool,
    pub with_unit: bool,
    pub with_city_or_equivalent: bool,
    pub with_small_containing_boundaries: bool,
    pub with_postal_code: bool,
    pub with_latlon: bool,
    pub latitude: f64,
    pub longitude: f64,
    pub geohash_precision: u32,
    pub name_and_address_keys: bool,
    pub name_only_keys: bool,
    pub address_only_keys: bool,
}

impl ToC for NearDupeHashOptions {
    type Out = sys::libpostal_near_dupe_hash_options_t;

    fn to_c(&self) -> Self::Out {
        sys::libpostal_near_dupe_hash_options_t {
            with_name: self.with_name.to_c(),
            with_address: self.with_address.to_c(),
            with_unit: self.with_unit.to_c(),
            with_city_or_equivalent: self.with_city_or_equivalent.to_c(),
            with_small_containing_boundaries: self.with_small_containing_boundaries.to_c(),
            with_postal_code: self.with_postal_code.to_c(),
            with_latlon: self.with_latlon.to_c(),
            latitude: self.latitude,
            longitude: self.longitude,
            geohash_precision: self.geohash_precision,
            name_and_address_keys: self.name_and_address_keys.to_c(),
            name_only_keys: self.name_only_keys.to_c(),
            address_only_keys: self.address_only_keys.to_c(),
        }
    }
}

impl ToRust for sys::libpostal_near_dupe_hash_options_t {
    type Out = NearDupeHashOptions;

    #[inline]
    fn to_rust(&self) -> Self::Out {
        NearDupeHashOptions {
            with_name: self.with_name.to_rust(),
            with_address: self.with_address.to_rust(),
            with_unit: self.with_unit.to_rust(),
            with_city_or_equivalent: self.with_city_or_equivalent.to_rust(),
            with_small_containing_boundaries: self.with_small_containing_boundaries.to_rust(),
            with_postal_code: self.with_postal_code.to_rust(),
            with_latlon: self.with_latlon.to_rust(),
            latitude: self.latitude,
            longitude: self.longitude,
            geohash_precision: self.geohash_precision,
            name_and_address_keys: self.name_and_address_keys.to_rust(),
            name_only_keys: self.name_only_keys.to_rust(),
            address_only_keys: self.address_only_keys.to_rust(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DuplicateOptions {
    pub languages: Vec<String>,
}

impl ToC for DuplicateOptions {
    // Funnily enough, to not kill performances, we return a tuple containing:
    //
    // (
    //  The original types where Vec<c_char*> points to,
    //  The vector pointing to data (we need to return it or the vec will be freed before we can
    //     even use it),
    //  The actual struct used by C,
    // )
    type Out = (
        Vec<CString>,
        Vec<*const c_char>,
        sys::libpostal_duplicate_options_t,
    );

    #[inline]
    fn to_c(&self) -> Self::Out {
        let (x, languages) = self.languages.as_slice().to_c();
        let ptrs = languages.as_ptr();
        (
            x,
            languages,
            sys::libpostal_duplicate_options_t {
                num_languages: self.languages.len() as _,
                languages: ptrs as usize as _,
            },
        )
    }
}

impl ToRust for sys::libpostal_duplicate_options_t {
    type Out = DuplicateOptions;

    #[inline]
    fn to_rust(&self) -> Self::Out {
        if self.languages.is_null() || self.num_languages == 0 {
            return DuplicateOptions {
                languages: Vec::new(),
            };
        }

        let mut languages = Vec::with_capacity(self.num_languages);
        for i in 0..self.num_languages {
            languages.push(unsafe { (*self.languages.offset(i as _)).to_rust() });
        }
        unsafe {
            sys::libpostal_expansion_array_destroy(self.languages, self.num_languages);
        }
        DuplicateOptions { languages }
    }
}

#[derive(Clone, Debug)]
pub struct FuzzyDuplicateOptions {
    pub languages: Vec<String>,
    pub needs_review_threshold: f64,
    pub likely_dupe_threshold: f64,
}

impl ToC for FuzzyDuplicateOptions {
    // Funnily enough, to not kill performances, we return a tuple containing:
    //
    // (
    //  The original types where Vec<c_char*> points to,
    //  The vector pointing to data (we need to return it or the vec will be freed before we can
    //     even use it),
    //  The actual struct used by C,
    // )
    type Out = (
        Vec<CString>,
        Vec<*const c_char>,
        sys::libpostal_fuzzy_duplicate_options_t,
    );

    #[inline]
    fn to_c(&self) -> Self::Out {
        let (x, languages) = self.languages.as_slice().to_c();
        let ptrs = languages.as_ptr();
        (
            x,
            languages,
            sys::libpostal_fuzzy_duplicate_options_t {
                num_languages: self.languages.len() as _,
                languages: ptrs as usize as _,
                needs_review_threshold: self.needs_review_threshold,
                likely_dupe_threshold: self.likely_dupe_threshold,
            },
        )
    }
}

impl ToRust for sys::libpostal_fuzzy_duplicate_options_t {
    type Out = FuzzyDuplicateOptions;

    #[inline]
    fn to_rust(&self) -> Self::Out {
        if self.languages.is_null() || self.num_languages == 0 {
            return FuzzyDuplicateOptions {
                languages: Vec::new(),
                needs_review_threshold: self.needs_review_threshold,
                likely_dupe_threshold: self.likely_dupe_threshold,
            };
        }

        let mut languages = Vec::with_capacity(self.num_languages);
        for i in 0..self.num_languages {
            languages.push(unsafe { (*self.languages.offset(i as _)).to_rust() });
        }
        unsafe {
            sys::libpostal_expansion_array_destroy(
                self.languages as usize as _,
                self.num_languages,
            );
        }
        FuzzyDuplicateOptions {
            languages,
            needs_review_threshold: self.needs_review_threshold,
            likely_dupe_threshold: self.likely_dupe_threshold,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FuzzyDuplicateStatus {
    status: DuplicateStatus,
    similarity: f64,
}

impl ToC for FuzzyDuplicateStatus {
    type Out = sys::libpostal_fuzzy_duplicate_status_t;

    #[inline]
    fn to_c(&self) -> Self::Out {
        sys::libpostal_fuzzy_duplicate_status_t {
            status: self.status.to_c(),
            similarity: self.similarity,
        }
    }
}

impl ToRust for sys::libpostal_fuzzy_duplicate_status_t {
    type Out = FuzzyDuplicateStatus;

    #[inline]
    fn to_rust(&self) -> Self::Out {
        FuzzyDuplicateStatus {
            status: self.status.to_rust(),
            similarity: self.similarity,
        }
    }
}
