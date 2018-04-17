//! # Sign
//! `
//! <sign> -> + | - | <empty>
//! `
//!
//! Note that the 'empty' sign is not handled in this parser; it may be handled
//! at a later date, but I'm not dealing with that until the num module is
//! fully implemented
//!
//! ## Parser: positive
//! Reads in a + symbol and returns Positive
//!
//! ### Example
//! ```rust
//! extern crate lisp;
//! extern crate nom;
//!
//! use lisp::parse::num::sign::{Sign,positive};
//! use nom::IResult;
//!
//! # fn main() {
//! assert_eq!(positive(&b"+"[..]), IResult::Done(&b""[..], Sign::Positive));
//! # }
//! ```
//!
//! ## Parser: negative
//! Reads in a - symbol and returns Negative
//!
//! ### Example
//! ```rust
//! extern crate lisp;
//! extern crate nom;
//!
//! use lisp::parse::num::sign::{Sign,negative};
//! use nom::IResult;
//!
//! # fn main() {
//! assert_eq!(negative(&b"-"[..]), IResult::Done(&b""[..], Sign::Negative));
//! # }
//! ```
//!
//! ## Parser: sign
//! Reads in either a positive or a negative
//!
//! ### Example
//! ```rust
//! extern crate lisp;
//! extern crate nom;
//!
//! use lisp::parse::num::sign::{Sign,sign};
//! use nom::IResult;
//!
//! # fn main() {
//! assert_eq!(sign(&b"-"[..]), IResult::Done(&b""[..], Sign::Negative));
//! assert_eq!(sign(&b"+"[..]), IResult::Done(&b""[..], Sign::Positive));
//! # }
//! ```

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Sign {
    Positive,
    Negative
}

named! ( pub positive<Sign>,
         map!(tag!("+"),
              |_| Sign::Positive)
       );

named! ( pub negative<Sign>,
         map!(tag!("-"),
              |_| Sign::Negative)
       );

named! ( pub sign<Sign>,
         ws!(alt!(positive | negative))
       );

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    #[test]
    fn leading_whitespace() {
        assert_eq!(sign(&b"               +"[..]),
                   IResult::Done(&b""[..], Sign::Positive)
                  );
    }

    #[test]
    fn trailing_whitespace() {
        assert_eq!(sign(&b"+               "[..]),
                   IResult::Done(&b""[..], Sign::Positive)
                  );
    }

    #[test]
    fn surrounding_whitespace() {
        assert_eq!(sign(&b"            +           "[..]),
                   IResult::Done(&b""[..], Sign::Positive)
                  );
    }
}

