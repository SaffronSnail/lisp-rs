macro_rules! define_radix {
    ($name:ident, $tag:expr, $number:expr) => (
        named! { pub $name<u8>,
                     map!(tag!($tag),
                          |_| $number)}
        )
}

define_radix!(radix_2, "#b", 2);
define_radix!(radix_8, "#o", 8);
define_radix!(radix_10, "#d", 10);
define_radix!(radix_16, "#x", 16);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    #[test]
    fn radix_2_test() {
        assert_eq!(radix_2(&b"#b"[..]), IResult::Done(&b""[..], 2));
    }
}
