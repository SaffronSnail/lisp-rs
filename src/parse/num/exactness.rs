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
