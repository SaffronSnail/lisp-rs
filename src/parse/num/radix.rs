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
