pub mod boolean;
pub mod num;

use self::boolean::boolean;
use std::fmt::Display;

named! { pub r7rs<Box<Display>>,
         map!(boolean,
              |token| Box::new(token))
}
