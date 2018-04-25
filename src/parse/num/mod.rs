pub mod exactness;
pub mod radix;
pub mod sign;

pub use self::exactness::Exactness;
pub use self::sign::Sign;

pub struct Number {
    sign: Sign,
    exactness: Exactness,
}

use self::exactness::exactness;
use self::sign::sign;

named!{pub num<Number>,
       map!(tuple!(sign, exactness),
            |result| Number {
                sign: result.0,
                exactness: result.1
            }
       )
}
