extern crate libc;

pub use address::Address;
pub use address_parser::AddressParser;
pub use core::Core;
pub use language_classifier::LanguageClassifier;
pub use options::{AddressComponents, AddressParserOptions, NearDupeHashOptions, NormalizeOptions};

mod address;
mod address_parser;
mod core;
mod language_classifier;
mod options;
pub mod sys;
mod traits;
mod utils;
