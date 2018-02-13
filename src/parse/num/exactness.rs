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
