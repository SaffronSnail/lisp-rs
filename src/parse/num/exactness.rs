//! # Exactness
//! `
//! <exactness> -> #e | #i
//! `
//!
//! Note that the 'empty' exactness is not handled in this parser; it is the
//! responsibility of higher-level syntax to provide reasonale defaults.
//!
//! ## Parser: exact
//! Translates '#e' into Exact
//!
//! ### Examples
//! ```rust
//! # extern crate lisp;
//! # extern crate nom;
//!
//! # use lisp::parse::num::exactness::{Exactness,exact};
//! # use nom::IResult;
//!
//! # fn main() {
//! assert_eq!(exact(&b"#e"[..]),
//!            IResult::Done(&b""[..], Exactness::Exact)
//!           )
//! # }
//! ```
//!
//! ## Parser: inexact
//! Translates '#i' into inexact
//!
//! ### Examples
//! ```rust
//! # extern crate lisp;
//! # extern crate nom;
//!
//! # use lisp::parse::num::exactness::{Exactness,inexact};
//! # use nom::IResult;
//!
//! # fn main() {
//! assert_eq!(inexact(&b"#i"[..]),
//!            IResult::Done(&b""[..], Exactness::Inexact)
//!           )
//! # }
//! ```
//!
//! ## Parser: exactness
//! Parses all possible 'exactness' literals
//!
//! ### Examples
//! ```rust
//! # extern crate lisp;
//! # extern crate nom;
//!
//! # use lisp::parse::num::exactness::{Exactness,exactness};
//! # use nom::IResult;
//!
//! # fn main() {
//! assert_eq!(exactness(&b"#e"[..]),
//!            IResult::Done(&b""[..], Exactness::Exact)
//!           );
//!
//! assert_eq!(exactness(&b"#i"[..]),
//!            IResult::Done(&b""[..], Exactness::Inexact)
//!           )
//! # }
//! ```
//!

#[derive(Debug, PartialEq)]
pub enum Exactness {
    Exact,
    Inexact
}

named! { pub exact<Exactness>,
         map!(tag!("#e"),
              |_| Exactness::Exact)
}

named! { pub inexact<Exactness>,
         map!(tag!("#i"),
              |_| Exactness::Inexact)
}

named! { pub exactness<Exactness>,
         ws!(alt!(exact | inexact))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    #[test]
    fn leading_whitespace() {
        assert_eq!(exactness(&b" \t\t \n #i"[..]), IResult::Done(&b""[..], Exactness::Inexact));
    }

    #[test]
    fn trailing_whitespace() {
        assert_eq!(exactness(&b"#i \t  \n\n  "[..]), IResult::Done(&b""[..], Exactness::Inexact));
    }

    #[test]
    fn surrounding_whitespace() {
        assert_eq!(exactness(&b"   \t\t\t\n\n\n#i\n\n\n\t\t\t    "[..]), IResult::Done(&b""[..], Exactness::Inexact));
    }
}
