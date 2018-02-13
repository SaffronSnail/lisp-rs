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
                assert_eq!(scm_radix(&b" \t\t \n #b"[..]), IResult::Done(&b""[..], 2));
            }

            #[test]
            fn trailing_whitespace() {
                assert_eq!(scm_radix(&b"#b \t  \n\n  "[..]), IResult::Done(&b""[..], 2));
            }

            #[test]
            fn surrounding_whitespace() {
                assert_eq!(scm_radix(&b"   \t\t\t\n\n\n#b\n\n\n\t\t\t    "[..]), IResult::Done(&b""[..], 2));
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


named! ( pub scm_radix<u8>,
         ws!(alt!(
             radix_2 |
             radix_8 |
             radix_10 |
             radix_16
         ))
);
