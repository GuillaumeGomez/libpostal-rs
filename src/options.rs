use std::ffi::CString;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AddressComponent {
    Any,
    Name,
    HouseNumber,
    Street,
    Unit,
    Level,
    Staircase,
    Entrance,
    Category,
    Near,
    Toponym,
    PostalCode,
    POBox,
    All,
}

impl ToC for AddressComponent {
    type Out = u16;

    fn to_c(&self) -> u16 {
        match *self {
            AddressComponent::Any => sys::LIBPOSTAL_ADDRESS_ANY,
            AddressComponent::Name => sys::LIBPOSTAL_ADDRESS_NAME,
            AddressComponent::HouseNumber => sys::LIBPOSTAL_ADDRESS_HOUSE_NUMBER,
            AddressComponent::Street => sys::LIBPOSTAL_ADDRESS_STREET,
            AddressComponent::Unit => sys::LIBPOSTAL_ADDRESS_UNIT,
            AddressComponent::Level => sys::LIBPOSTAL_ADDRESS_LEVEL,
            AddressComponent::Staircase => sys::LIBPOSTAL_ADDRESS_STAIRCASE,
            AddressComponent::Entrance => sys::LIBPOSTAL_ADDRESS_ENTRANCE,
            AddressComponent::Category => sys::LIBPOSTAL_ADDRESS_CATEGORY,
            AddressComponent::Near => sys::LIBPOSTAL_ADDRESS_NEAR,
            AddressComponent::Toponym => sys::LIBPOSTAL_ADDRESS_TOPONYM,
            AddressComponent::PostalCode => sys::LIBPOSTAL_ADDRESS_POSTAL_CODE,
            AddressComponent::POBox => sys::LIBPOSTAL_ADDRESS_PO_BOX,
            AddressComponent::All => sys::LIBPOSTAL_ADDRESS_ALL,
        }
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
