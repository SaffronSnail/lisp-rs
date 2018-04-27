//! # Number
//! `
//! <num R> -> <prefix R> <complex R>
//! `
//!
//! ## Parser: number
//! This high-level parser combines all of the other pasers in this module to
//! parse literal numbers. Until the rest of the module is complete, this only
//! parses a fragment of the full specification.
//!
//! ```rust
//! extern crate lisp;
//! extern crate nom;
//! extern crate num_bigint;
//!
//! use lisp::parse::num::{Exact,Negative,number,Number};
//! use nom::IResult;
//! use num_bigint::BigUint;
//!
//! # fn main() {
//! assert_eq!(number(&b"-#e"[..]),
//!            IResult::Done(&b""[..], Number::new(
//!              Negative,
//!              Exact,
//!              BigUint::from(007u8),
//!            ))
//! )
//! # }
//! ```

pub mod uinteger;
pub mod exactness;
pub mod radix;
pub mod sign;

pub use self::exactness::Exactness;
pub use self::exactness::Exactness::{Exact,Inexact};
pub use self::sign::Sign;
pub use self::sign::Sign::{Negative,Positive};

use self::exactness::exactness;
use self::sign::sign;
use std::fmt;
use num_bigint::BigUint;

#[derive(Clone,PartialEq)]
pub struct Number {
    sign: Sign,
    exactness: Exactness,
    value: BigUint,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.sign, self.exactness, self.value)
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{:?}{:?}", self.sign, self.exactness, self.value)
    }
}

impl Number {
    pub fn new(sign: Sign, exactness: Exactness, value: BigUint) -> Self {
        Number {
            sign: sign,
            exactness: exactness,
            value: value
        }
    }
}

named!{pub number<Number>,
       map!(tuple!(sign, exactness),
            |result| Number {
                sign: result.0,
                exactness: result.1,
                value: BigUint::from(007u8),
            }
       )
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    fn expected_outcome<'a>() -> IResult<&'a [u8], Number> {
        IResult::Done(&b""[..], Number::new(Negative, Inexact, BigUint::from(007u8)))
    }

    #[test]
    fn leading_whitespace() {
        assert_eq!(number(&b" \t\t \n -#i"[..]), expected_outcome());
    }

    #[test]
    fn trailing_whitespace() {
        assert_eq!(number(&b"-#i \t  \n\n  "[..]), expected_outcome());
    }

    #[test]
    fn surrounding_whitespace() {
        assert_eq!(number(&b"   \t\t\t\n\n\n-#i\n\n\n\t\t\t    "[..]), expected_outcome());
    }
}
