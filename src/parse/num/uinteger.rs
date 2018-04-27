//! # Digit
//! `
//! <digit 2> -> 0 | 1
//! <digit 8> -> [0-7]
//! <digit 10> -> [0-9]
//! <digit 12> -> [0-9A-F]
//! `
//!
//! Digit parsers deal with the absolute value of the number being parsed
//!
//! ## Parser: octal_digit
//! ### Example
//! ```rust
//! extern crate lisp;
//! extern crate nom;
//!
//! use lisp::parse::num::uinteger::octal_digit;
//! use lisp::parse::num::uinteger::Uinteger;
//! use nom::IResult;
//!
//! # fn main() {
//! assert_eq!(octal_digit(&b"1234567"[..]), IResult::Done(&b""[..], Uinteger::from(342391u32)))
//! # }
//! ```
//!
//! ## Parser: decimal_digit
//! ### Example
//! ```rust
//! extern crate lisp;
//! extern crate nom;
//!
//! use lisp::parse::num::uinteger::decimal_digit;
//! use lisp::parse::num::uinteger::Uinteger;
//! use nom::IResult;
//!
//! # fn main() {
//! assert_eq!(decimal_digit(&b"123456789"[..]), IResult::Done(&b""[..], Uinteger::from(123456789u32)))
//! # }
//! ```
//!
//! ## Parser: hex_digit
//! ### Example
//! ```rust
//! extern crate lisp;
//! extern crate nom;
//! extern crate num_bigint;
//!
//! use lisp::parse::num::uinteger::hex_digit;
//! use lisp::parse::num::uinteger::Uinteger;
//! use nom::IResult;
//! use num_bigint::BigUint;
//!
//! # fn main() {
//! let mut expected = BigUint::from(1375488932539311409u64);
//! expected *= 1000000u32;
//! expected += 843695u32;
//!
//! // No more mutation!
//! let expected = expected;
//! assert_eq!(hex_digit(&b"123456789abcdefABCDEF"[..]), IResult::Done(&b""[..], Uinteger::from(expected)));
//! # }
//! ```

use num_bigint::BigUint;
use num_traits::Num;

use std::str::from_utf8;
use std::str::FromStr;

#[derive(Debug,PartialEq,Eq)]
pub struct Uinteger ( BigUint );

impl FromStr for Uinteger {
    type Err = <BigUint as FromStr>::Err;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match BigUint::from_str(input) {
            Result::Ok(r)  => Result::Ok(Uinteger(r)),
            Result::Err(e) => Result::Err(e)
        }
    }
}

impl<T> From<T> for Uinteger where BigUint: From<T> {
    fn from(t: T) -> Self {
        Uinteger(BigUint::from(t))
    }
}

macro_rules! define_digit {
    ($name:ident, $valid_chars:expr, $radix:expr) => {
        named!{ pub $name<Uinteger>,
            map!(is_a!($valid_chars),
                 |num| Uinteger(<BigUint as Num>::from_str_radix(from_utf8(num).unwrap(), $radix).unwrap())
        )}
    }
}

define_digit! { binary_digit,  "01", 2 }
define_digit! { octal_digit,   "01234567", 8 }
define_digit! { decimal_digit, "0123456789", 10 }
define_digit! { hex_digit,     "0123456789123456789abcdefABCDEF", 16 }
