//! # Boolean
//! Definition of Boolean from r7rs:<br/>
//! `<boolean> -> #t | #f | #true | #false`
//!
//! This is a simple parser: it reads one of the four literals
//! listed, returning the appropriate boolean value.
//!
//! ## Parser: true
//! Parses literals that evaluate to "true": `#t` and `#true`
//!
//! ```rust
//! extern crate lisp;
//! extern crate nom;
//!
//! use lisp::parse::boolean::scm_true;
//! use nom::IResult;
//!
//! # fn main() {
//! assert_eq!(scm_true(&b"#t"[..]), IResult::Done(&b""[..], true));
//! assert_eq!(scm_true(&b"#true"[..]), IResult::Done(&b""[..], true));
//! # }
//! ```
//!
//! ## Parser: false
//! Parses literals that evaluate to "false": `#f` and `#false`
//!
//! ```rust
//! extern crate lisp;
//! extern crate nom;
//!
//! use lisp::parse::boolean::scm_false;
//! use nom::IResult;
//!
//! # fn main() {
//! assert_eq!(scm_false(&b"#f"[..]), IResult::Done(&b""[..], false));
//! assert_eq!(scm_false(&b"#false"[..]), IResult::Done(&b""[..], false));
//! # }
//! ```
//!
//! ## Parser: boolean
//! Combines the true and false parsers into a general boolean parser
//!
//! ```rust
//! extern crate lisp;
//! extern crate nom;
//!
//! use lisp::parse::boolean::boolean;
//! use nom::IResult;
//!
//! # fn main() {
//! assert_eq!(boolean(&b"#t"[..]), IResult::Done(&b""[..], true));
//! assert_eq!(boolean(&b"#true"[..]), IResult::Done(&b""[..], true));
//! assert_eq!(boolean(&b"#f"[..]), IResult::Done(&b""[..], false));
//! assert_eq!(boolean(&b"#false"[..]), IResult::Done(&b""[..], false));
//! # }
//! ```

named! { pub scm_true<bool>,
         map!(alt_complete!(tag!("#true") | tag!("#t")),
              |_| true)
}

named! {
    pub scm_false<bool>,
         map!(alt_complete!(tag!("#false") | tag!("#f")),
              |_| false)
}

named! { pub boolean<bool>,
         ws!(alt!(scm_true | scm_false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    #[test]
    fn leading_whitespace() {
        assert_eq!(boolean(&b" \t\t \n #true"[..]), IResult::Done(&b""[..], true));
    }

    #[test]
    fn trailing_whitespace() {
        assert_eq!(boolean(&b"#true \t  \n\n  "[..]), IResult::Done(&b""[..], true));
    }

    #[test]
    fn surrounding_whitespace() {
        assert_eq!(boolean(&b"   \t\t\t\n\n\n#true\n\n\n\t\t\t    "[..]), IResult::Done(&b""[..], true));
    }
}
