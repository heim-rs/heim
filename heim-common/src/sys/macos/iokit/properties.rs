use core_foundation::base::{CFType, ToVoid};
use core_foundation::boolean::{CFBoolean, CFBooleanGetTypeID};
use core_foundation::dictionary::{CFDictionary, CFDictionaryGetTypeID, CFDictionaryRef};
use core_foundation::number::{CFNumber, CFNumberGetTypeID};
use core_foundation::string::{CFString, CFStringGetTypeID};

pub use core_foundation::base::TCFType;

use crate::{Error, Result};

/// Extends `CFDictionary` with a few methods used by `heim` crates.
pub trait DictionaryProps {
    /// Get the dictionary value with the `raw_key` key.
    fn get_dict(&self, raw_key: &'static str) -> Result<CFDictionary<CFString, CFType>>;

    /// Get the bool value with the `raw_key` key.
    fn get_bool(&self, raw_key: &'static str) -> Result<bool>;

    /// Get the i64 value with the `raw_key` key.
    fn get_i64(&self, raw_key: &'static str) -> Result<i64>;

    /// Get the string value with the `raw_key` key.
    fn get_string(&self, raw_key: &'static str) -> Result<String>;
}

impl DictionaryProps for CFDictionary<CFString, CFType> {
    fn get_dict(&self, raw_key: &'static str) -> Result<CFDictionary<CFString, CFType>> {
        let key = CFString::from_static_string(raw_key);

        self.find(&key)
            .and_then(|value_ref| {
                unsafe {
                    debug_assert!(value_ref.type_of() == CFDictionaryGetTypeID());
                }

                // "Casting" `CFDictionary<*const void, *const void>` into a needed dict type

                // TODO: I think that reference to an original dict is still stored somewhere
                // and it does not decrements here.
                let ptr = value_ref.to_void() as CFDictionaryRef;

                unsafe { Some(CFDictionary::wrap_under_get_rule(ptr)) }
            })
            .ok_or_else(|| Error::missing_entity(raw_key))
    }

    fn get_bool(&self, raw_key: &'static str) -> Result<bool> {
        let key = CFString::from_static_string(raw_key);

        self.find(&key)
            .and_then(|value_ref| {
                unsafe {
                    debug_assert!(value_ref.type_of() == CFBooleanGetTypeID());
                }

                value_ref.downcast::<CFBoolean>()
            })
            .map(Into::into)
            .ok_or_else(|| Error::missing_entity(raw_key))
    }

    fn get_i64(&self, raw_key: &'static str) -> Result<i64> {
        let key = CFString::from_static_string(raw_key);

        self.find(&key)
            .and_then(|value_ref| {
                unsafe {
                    debug_assert!(value_ref.type_of() == CFNumberGetTypeID());
                }

                value_ref.downcast::<CFNumber>()
            })
            .and_then(|number| number.to_i64())
            .ok_or_else(|| Error::missing_entity(raw_key))
    }

    fn get_string(&self, raw_key: &'static str) -> Result<String> {
        let key = CFString::from_static_string(raw_key);

        self.find(&key)
            .and_then(|value_ref| {
                unsafe {
                    debug_assert!(value_ref.type_of() == CFStringGetTypeID());
                }

                value_ref.downcast::<CFString>()
            })
            .map(|cf_string| cf_string.to_string())
            .ok_or_else(|| Error::missing_entity(raw_key))
    }
}
