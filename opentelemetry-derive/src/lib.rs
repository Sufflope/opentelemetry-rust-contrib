//! # Usage
//!
//! Add the crate to your `Cargo.toml`:
//!
//! ```toml
//! opentelemetry_derive = "0.1"
//! ```
//!
//! You also need to have `opentelemetry` as a dependency, because the macros will generate code
//! that references `opentelemetry::{Key, KeyValue, StringValue, Value}`. The version does not
//! matter as long as it publishes those types.
//!
//! ## `Key`
//!
//! If you do not set the key explicitly, the macro will autogenerate one,
//! which will be your type's name, lowercased:
//!
//! ```rust
//! use opentelemetry_derive::Key;
//!
//! #[derive(Key)]
//! struct Auto;
//!
//! #[derive(Key)]
//! #[otel(key = "custom")]
//! struct Overriden;
//! ```
//!
//! ## `Value`
//!
//! You must specify an intermediate type, into which your own type will be converted,
//! that is itself already convertible into a [Value]:
//!
//! ```rust
//! use opentelemetry_derive::Value;
//!
//! #[derive(Value)]
//! #[otel(variant = i64)]
//! struct Counter {
//!     count: i64,
//! }
//!
//! impl From<&Counter> for i64 {
//!     fn from(value: &Counter) -> Self {
//!         value.count
//!     }
//! }
//! ```
//!
//! ## `StringValue`
//!
//! Your type must implement [ToString] (probably through [Display](std::fmt::Display)):
//!
//! ```rust
//! use std::fmt;
//!
//! use opentelemetry_derive::StringValue;
//!
//! #[derive(StringValue)]
//! enum Method {
//!     Get,
//!     Post,
//! }
//!
//! impl fmt::Display for Method {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         write!(
//!             f,
//!             "{}",
//!             match self {
//!                 Self::Get => "get",
//!                 Self::Post => "post",
//!             }
//!         )
//!     }
//! }
//! ```
//!
//! ## `KeyValue`
//!
//! References to your type must be both `Into<Key>` and `Into<Value>`:
//!
//! ```rust
//! use opentelemetry::{Key, Value};
//! use opentelemetry_derive::KeyValue;
//!
//! #[derive(KeyValue)]
//! struct Config {
//!     value: bool,
//! }
//!
//! const KEY: &str = "config";
//!
//! impl From<&Config> for Key {
//!     fn from(_: &Config) -> Self {
//!         Self::from(KEY)
//!     }
//! }
//!
//! impl From<&Config> for Value {
//!     fn from(value: &Config) -> Self {
//!         Value::from(value.value)
//!     }
//! }
//! ```
//!
//! Of course you can combine all the derives instead of manually implementing the required conversions:
//!
//! ```rust
//! use std::fmt;
//!
//! use opentelemetry::StringValue;
//! use opentelemetry_derive::{Key, KeyValue, StringValue, Value};
//!
//! #[derive(Key, KeyValue, StringValue, Value)]
//! #[otel(key = "req", variant = StringValue)]
//! struct Request {
//!     query: String,
//! }
//!
//! impl fmt::Display for Request {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         write!(f, "{}", self.query)
//!     }
//! }
//! ```
//!
//! [Value]: https://docs.rs/opentelemetry/latest/opentelemetry/enum.Value.html

/// Derive conversion into [Key].
///
/// The optional `key` attribute overrides the autogenerated key (type name, lowercased).
///
/// [Key]: https://docs.rs/opentelemetry/latest/opentelemetry/struct.Key.html
pub use opentelemetry_derive_impl::Key;

/// Derive conversion into [KeyValue].
///
/// [KeyValue]: https://docs.rs/opentelemetry/latest/opentelemetry/struct.KeyValue.html
pub use opentelemetry_derive_impl::KeyValue;

/// Derive conversion into [StringValue].
///
/// [StringValue]: https://docs.rs/opentelemetry/latest/opentelemetry/struct.StringValue.html
pub use opentelemetry_derive_impl::StringValue;

/// Derive conversion into [Value].
///
/// The mandatory `variant` attribute is the intermediate type, into which your value will be converted
/// (e.g. [StringValue]
/// if your type should be represented as a string, or [i64]).
/// This variant should itself be one of the types than can be implicitly converted to [Value].
///
/// [StringValue]: https://docs.rs/opentelemetry/latest/opentelemetry/struct.StringValue.html
/// [Value]: https://docs.rs/opentelemetry/latest/opentelemetry/enum.Value.html
pub use opentelemetry_derive_impl::Value;

#[cfg(test)]
mod tests {
    extern crate self as opentelemetry_derive;

    use std::fmt;

    use opentelemetry::{Key, KeyValue, StringValue, Value};

    use crate::{Key, KeyValue, StringValue, Value};

    #[test]
    fn test_key() {
        #[derive(Key)]
        struct Auto;

        assert_eq!(Key::from(Auto).as_str(), "auto");
        assert_eq!(Key::from(&Auto).as_str(), "auto");

        #[derive(Key)]
        #[otel(key = "custom")]
        struct Overriden;

        assert_eq!(Key::from(Overriden).as_str(), "custom");
        assert_eq!(Key::from(&Overriden).as_str(), "custom");
    }

    #[test]
    fn test_value() {
        #[derive(Value)]
        #[otel(variant = i64)]
        struct Counter {
            count: i64,
        }

        impl From<&Counter> for i64 {
            fn from(value: &Counter) -> Self {
                value.count
            }
        }

        let count = 3;
        let counter = Counter { count };

        assert_eq!(Value::from(&counter).as_str(), count.to_string());
        assert_eq!(Value::from(counter).as_str(), count.to_string());
    }

    #[test]
    fn test_string_value() {
        #[derive(StringValue)]
        enum Method {
            Get,
            Post,
        }

        impl fmt::Display for Method {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        Self::Get => "get",
                        Self::Post => "post",
                    }
                )
            }
        }

        assert_eq!(
            StringValue::from(&Method::Get).as_str(),
            Method::Get.to_string()
        );
        assert_eq!(
            StringValue::from(Method::Post).as_str(),
            Method::Post.to_string()
        );
    }

    #[test]
    fn test_key_value() {
        #[derive(KeyValue)]
        struct Config {
            value: bool,
        }

        const KEY: &str = "config";

        impl From<&Config> for Key {
            fn from(_: &Config) -> Self {
                Self::from(KEY)
            }
        }

        impl From<&Config> for Value {
            fn from(value: &Config) -> Self {
                Value::from(value.value)
            }
        }

        let value = true;
        let config = Config { value };

        assert_eq!(KeyValue::from(&config), KeyValue::new(KEY, value));
        assert_eq!(KeyValue::from(config), KeyValue::new(KEY, value));
    }

    #[test]
    fn test_all() {
        #[derive(Key, KeyValue, StringValue, Value)]
        #[otel(key = "req", variant = StringValue)]
        struct Request {
            query: String,
        }

        impl fmt::Display for Request {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.query)
            }
        }

        let query = "foo=bar";
        let request = Request {
            query: query.to_string(),
        };

        assert_eq!(KeyValue::from(&request), KeyValue::new("req", query));
        assert_eq!(KeyValue::from(request), KeyValue::new("req", query));
    }
}