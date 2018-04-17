//! # Radix
//! `
//! <radix 2> -> #b
//! <radix 8> -> #o
//! <radix 10> -> <empty> | #d
//! <radix 16> -> #x
//! `
//!
//! Note that the 'empty' radix is not handled in this parser; it is the
//! responsibility of higher-level syntax to provide reasonable defaults.
//!
//! ## Parser: radix
//! This is a simple parser: radices are predefined literals that maps to
//! a specific number.
//!
//! ### Examples
//! ```rust
//! # extern crate lisp;
//! # extern crate nom;
//!
//! # use lisp::parse::num::radix::{radix,radix_2,radix_8,radix_10,radix_16};
//! # use nom::IResult;
//!
//! # fn main() {
//! assert_eq!(radix(&b"#b"[..]), IResult::Done(&b""[..], 2));
//! assert_eq!(radix_2(&b"#b"[..]), IResult::Done(&b""[..], 2));
//!
//! assert_eq!(radix(&b"#o"[..]), IResult::Done(&b""[..], 8));
//! assert_eq!(radix_8(&b"#o"[..]), IResult::Done(&b""[..], 8));
//!
//! assert_eq!(radix(&b"#d"[..]), IResult::Done(&b""[..], 10));
//! assert_eq!(radix_10(&b"#d"[..]), IResult::Done(&b""[..], 10));
//!
//! assert_eq!(radix(&b"#x"[..]), IResult::Done(&b""[..], 16));
//! assert_eq!(radix_16(&b"#x"[..]), IResult::Done(&b""[..], 16));
//! # }
//! ```

macro_rules! define_radices {
    ($(($name:ident, $tag:expr, $number:expr, $test_name:ident))* ) => (
        $(
            named! ( pub $name<u8>,
                     map!(tag!($tag),
                          |_| $number));
        )*


        #[cfg(test)]
        mod tests {
            use super::*;
            use nom::IResult;

            $(
                #[test]
                fn $test_name() {
                    assert_eq!($name($tag.as_bytes()), IResult::Done(&b""[..], $number));
                }
            )*

            #[test]
            fn leading_whitespace() {
                assert_eq!(radix(&b" \t\t \n #b"[..]), IResult::Done(&b""[..], 2));
            }

            #[test]
            fn trailing_whitespace() {
                assert_eq!(radix(&b"#b \t  \n\n  "[..]), IResult::Done(&b""[..], 2));
            }

            #[test]
            fn surrounding_whitespace() {
                assert_eq!(radix(&b"   \t\t\t\n\n\n#b\n\n\n\t\t\t    "[..]), IResult::Done(&b""[..], 2));
            }
        }
    )
}

define_radices! (
    (radix_2, "#b", 2, radix_2_test)
    (radix_8, "#o", 8, radix_8_test)
    (radix_10, "#d", 10, radix_10_test)
    (radix_16, "#x", 16, radix_16_test)
);


named! ( pub radix<u8>,
         ws!(alt!(
             radix_2 |
             radix_8 |
             radix_10 |
             radix_16
         ))
);

