use libc::{c_char, c_double, c_int, size_t};

#[allow(non_camel_case_types)]
pub type c_bool = c_int;

extern "C" {
    pub fn libpostal_setup() -> c_bool;
    pub fn libpostal_setup_datadir(datadir: *const c_char) -> c_bool;
    pub fn libpostal_setup_parser() -> c_bool;
    pub fn libpostal_setup_parser_datadir(datadir: *const c_char) -> c_bool;
    pub fn libpostal_setup_language_classifier() -> c_bool;
    pub fn libpostal_setup_language_classifier_datadir(datadir: *const c_char) -> c_bool;
    pub fn libpostal_teardown();
    pub fn libpostal_teardown_parser();
    pub fn libpostal_teardown_language_classifier();
    pub fn libpostal_place_languages(
        num_components: size_t,
        labels: *const *const c_char,
        values: *const *const c_char,
        num_languages: *mut size_t,
    ) -> *mut *mut c_char;
    pub fn libpostal_get_default_options() -> libpostal_normalize_options_t;
    pub fn libpostal_expand_address(
        input: *const c_char,
        options: libpostal_normalize_options_t,
        n: *mut size_t,
    ) -> *mut *mut c_char;
    pub fn libpostal_expand_address_root(
        input: *const c_char,
        options: libpostal_normalize_options_t,
        n: *mut size_t,
    ) -> *mut *mut c_char;
    pub fn libpostal_expansion_array_destroy(expansions: *mut *mut c_char, n: size_t);
    pub fn libpostal_get_address_parser_default_options() -> libpostal_address_parser_options_t;
    pub fn libpostal_parse_address(
        address: *const c_char,
        options: libpostal_address_parser_options_t,
    ) -> *mut libpostal_address_parser_response_t;
    pub fn libpostal_address_parser_response_destroy(
        this: *mut libpostal_address_parser_response_t,
    );
    pub fn libpostal_get_near_dupe_hash_default_options() -> libpostal_near_dupe_hash_options_t;
    pub fn libpostal_near_dupe_hashes(
        num_components: size_t,
        labels: *const *const c_char,
        values: *const *const c_char,
        options: libpostal_near_dupe_hash_options_t,
        num_hashes: *mut size_t,
    ) -> *mut *mut c_char;
    pub fn libpostal_near_dupe_hashes_languages(
        num_components: size_t,
        labels: *const *const c_char,
        values: *const *const c_char,
        options: libpostal_near_dupe_hash_options_t,
        num_languages: size_t,
        languages: *const *const c_char,
        num_hashes: *mut size_t,
    ) -> *mut *mut c_char;
    pub fn libpostal_get_default_duplicate_options() -> libpostal_duplicate_options_t;
    pub fn libpostal_get_duplicate_options_with_languages(
        num_languages: size_t,
        languages: *const *const c_char,
    ) -> libpostal_duplicate_options_t;
    pub fn libpostal_is_toponym_duplicate(
        num_components1: size_t,
        labels1: *const *const c_char,
        values1: *const *const c_char,
        num_components2: size_t,
        labels2: *const *const c_char,
        values2: *const *const c_char,
        options: libpostal_duplicate_options_t,
    ) -> libpostal_duplicate_status_t;
    pub fn libpostal_is_name_duplicate(
        value1: *const c_char,
        value2: *const c_char,
        options: libpostal_duplicate_options_t,
    ) -> libpostal_duplicate_status_t;
    pub fn libpostal_is_street_duplicate(
        value1: *const c_char,
        value2: *const c_char,
        options: libpostal_duplicate_options_t,
    ) -> libpostal_duplicate_status_t;
    pub fn libpostal_is_house_number_duplicate(
        value1: *const c_char,
        value2: *const c_char,
        options: libpostal_duplicate_options_t,
    ) -> libpostal_duplicate_status_t;
    pub fn libpostal_is_po_box_duplicate(
        value1: *const c_char,
        value2: *const c_char,
        options: libpostal_duplicate_options_t,
    ) -> libpostal_duplicate_status_t;
    pub fn libpostal_is_unit_duplicate(
        value1: *const c_char,
        value2: *const c_char,
        options: libpostal_duplicate_options_t,
    ) -> libpostal_duplicate_status_t;
    pub fn libpostal_is_floor_duplicate(
        value1: *const c_char,
        value2: *const c_char,
        options: libpostal_duplicate_options_t,
    ) -> libpostal_duplicate_status_t;
    pub fn libpostal_is_postal_code_duplicate(
        value1: *const c_char,
        value2: *const c_char,
        options: libpostal_duplicate_options_t,
    ) -> libpostal_duplicate_status_t;
    pub fn libpostal_get_default_fuzzy_duplicate_options() -> libpostal_fuzzy_duplicate_options_t;
    pub fn libpostal_get_default_fuzzy_duplicate_options_with_languages(
        num_languages: size_t,
        languages: *const *const c_char,
    ) -> libpostal_fuzzy_duplicate_options_t;
    pub fn libpostal_is_name_duplicate_fuzzy(
        num_tokens1: size_t,
        tokens1: *const *const c_char,
        token_scores1: *const c_double,
        num_tokens2: size_t,
        tokens2: *const *const c_char,
        token_scores2: *const c_double,
        options: libpostal_fuzzy_duplicate_options_t,
    ) -> libpostal_fuzzy_duplicate_status_t;
    pub fn libpostal_is_street_duplicate_fuzzy(
        num_tokens1: size_t,
        tokens1: *const *const c_char,
        token_scores1: *const c_double,
        num_tokens2: size_t,
        tokens2: *const *const c_char,
        token_scores2: *const c_double,
        options: libpostal_fuzzy_duplicate_options_t,
    ) -> libpostal_fuzzy_duplicate_status_t;
    pub fn libpostal_normalize_string(input: *const c_char, options: u64) -> *mut c_char;
    pub fn libpostal_normalize_string_languages(
        input: *const c_char,
        options: u64,
        num_languages: size_t,
        languages: *const *const c_char,
    ) -> *mut c_char;
    pub fn libpostal_tokenize(
        input: *const c_char,
        whitespace: c_bool,
        n: *mut size_t,
    ) -> *mut libpostal_token_t;
    pub fn libpostal_normalized_tokens(
        input: *const c_char,
        string_options: u64,
        token_options: u64,
        whitespace: c_bool,
        n: *mut size_t,
    ) -> *mut libpostal_normalized_token_t;
    pub fn libpostal_normalized_tokens_languages(
        input: *const c_char,
        string_options: u64,
        token_options: u64,
        whitespace: c_bool,
        num_languages: size_t,
        languages: *const *const c_char,
        n: *mut size_t,
    ) -> *mut libpostal_normalized_token_t;
    pub fn libpostal_parser_print_features(print_features: c_bool) -> c_bool;
}

pub const LIBPOSTAL_ADDRESS_NONE: u16 = 0;
pub const LIBPOSTAL_ADDRESS_ANY: u16 = (1 << 0);
pub const LIBPOSTAL_ADDRESS_NAME: u16 = (1 << 1);
pub const LIBPOSTAL_ADDRESS_HOUSE_NUMBER: u16 = (1 << 2);
pub const LIBPOSTAL_ADDRESS_STREET: u16 = (1 << 3);
pub const LIBPOSTAL_ADDRESS_UNIT: u16 = (1 << 4);
pub const LIBPOSTAL_ADDRESS_LEVEL: u16 = (1 << 5);
pub const LIBPOSTAL_ADDRESS_STAIRCASE: u16 = (1 << 6);
pub const LIBPOSTAL_ADDRESS_ENTRANCE: u16 = (1 << 7);

pub const LIBPOSTAL_ADDRESS_CATEGORY: u16 = (1 << 8);
pub const LIBPOSTAL_ADDRESS_NEAR: u16 = (1 << 9);

pub const LIBPOSTAL_ADDRESS_TOPONYM: u16 = (1 << 13);
pub const LIBPOSTAL_ADDRESS_POSTAL_CODE: u16 = (1 << 14);
pub const LIBPOSTAL_ADDRESS_PO_BOX: u16 = (1 << 15);
pub const LIBPOSTAL_ADDRESS_ALL: u16 = ((1u32 << 16) - 1) as u16;

#[derive(Debug)]
#[repr(C)]
pub struct libpostal_normalize_options_t {
    pub languages: *mut *mut c_char,
    pub num_languages: size_t,
    pub address_components: u16,
    pub latin_ascii: c_bool,
    pub transliterate: c_bool,
    pub strip_accents: c_bool,
    pub decompose: c_bool,
    pub lowercase: c_bool,
    pub trim_string: c_bool,
    pub drop_parentheticals: c_bool,
    pub replace_numeric_hyphens: c_bool,
    pub delete_numeric_hyphens: c_bool,
    pub split_alpha_from_numeric: c_bool,
    pub replace_word_hyphens: c_bool,
    pub delete_word_hyphens: c_bool,
    pub delete_final_periods: c_bool,
    pub delete_acronym_periods: c_bool,
    pub drop_english_possessives: c_bool,
    pub delete_apostrophes: c_bool,
    pub expand_numex: c_bool,
    pub roman_numerals: c_bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct libpostal_address_parser_options_t {
    pub language: *mut c_char,
    pub country: *mut c_char,
}

#[derive(Debug)]
#[repr(C)]
pub struct libpostal_address_parser_response_t {
    pub num_components: size_t,
    pub components: *mut *mut c_char,
    pub labels: *mut *mut c_char,
}

#[derive(Debug)]
#[repr(C)]
pub struct libpostal_near_dupe_hash_options_t {
    pub with_name: c_bool,
    pub with_address: c_bool,
    pub with_unit: c_bool,
    pub with_city_or_equivalent: c_bool,
    pub with_small_containing_boundaries: c_bool,
    pub with_postal_code: c_bool,
    pub with_latlon: c_bool,
    pub latitude: c_double,
    pub longitude: c_double,
    pub geohash_precision: u32,
    pub name_and_address_keys: c_bool,
    pub name_only_keys: c_bool,
    pub address_only_keys: c_bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct libpostal_duplicate_options_t {
    pub num_languages: size_t,
    pub languages: *mut *mut c_char,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum libpostal_duplicate_status_t {
    LIBPOSTAL_NULL_DUPLICATE_STATUS = -1,
    LIBPOSTAL_NON_DUPLICATE = 0,
    LIBPOSTAL_POSSIBLE_DUPLICATE_NEEDS_REVIEW = 3,
    LIBPOSTAL_LIKELY_DUPLICATE = 6,
    LIBPOSTAL_EXACT_DUPLICATE = 9,
}

#[derive(Debug)]
#[repr(C)]
pub struct libpostal_fuzzy_duplicate_options_t {
    pub num_languages: size_t,
    pub languages: *const *const c_char,
    pub needs_review_threshold: c_double,
    pub likely_dupe_threshold: c_double,
}

#[derive(Debug)]
#[repr(C)]
pub struct libpostal_fuzzy_duplicate_status_t {
    pub status: libpostal_duplicate_status_t,
    pub similarity: c_double,
}

#[derive(Debug)]
#[repr(C)]
pub struct libpostal_token_t {
    pub offset: size_t,
    pub len: size_t,
    pub type_: u16,
}

#[derive(Debug)]
#[repr(C)]
pub struct libpostal_normalized_token_t {
    pub str: *mut c_char,
    pub token: libpostal_token_t,
}
