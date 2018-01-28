# Lisp-rs

[![Build Status](https://travis-ci.org/SaffronSnail/lisp-rs.svg?branch=master)](https://travis-ci.org/SaffronSnail/lisp-rs)

Lisp-rs will be a set of libraries to facilitate the implementaiton of Lisps in Rust.

The first milestone is a parser for r7rs. It is being written using the nom library, which facilitates the creation of small, specific parsers that can be composed into more complex parsers.

## Project Organization

The r7rs parser has a structure based on the BNF description of the language. Each entry in the BNF description corresponds to a reusable subparser. A subparser belongs to one of three categories: a terminal parser or a compound parser.

### Terminal Subparsers
Terminal subparsers transform literal characters into the values that those characters stand for. Terminal parsers may require complex logic to analyze the text. They do not take factors such as whitespace into consideration. An example of a terminal parser is scm_true: it is responsible for parsing `#t` and `#true`, and always returns the rust value `true` as the result of it's parsing.

### Compound Subparsers
Compound subparsers combine terminal subparsers into more sophisticated subparsers. They may not directly analyze the data being parsed; rather, they simply `alt!` (or similar) between other subparsers (terimnal or compound), and deal with issues like whitespace so that terminal parsers can focus on the data that matters. An example of a compound parser is boolean: it `alt_complete!`'s between scm_true and scm_false, and eats the whitepace around them.

## Example in Depth: Boolean Parser
To better understand how this sytem of parsing works it is helpful to look at some actual code. But before that, let's look at the BNF description of `boolean` from page 62 of the r7rs:

```BNF
<boolean> -> #t | #f | #true | #false
```

This describes one symbol, `boolean`, which may be represented by one of 4 tokens, `#t`, `#f`, `#true`, or `#false`. This is implemented by having two terminal parsers: one for parsing tokens that evaluate to `true` and one for parsing tokens that evaluate to `false`.

```
// the named macro creates a new named parser which is defined using a convenient shorthand using more macros
named! { pub scm_true<bool>,
         // the alt_complete macro works like the alt macro (see the definition of the boolean parser, below) except that it ignores parsers that return Incomplete; to learn more, try replacing alt_complete with alt and see which tests fail and why. The, change it back to alt_complete, switch the order of the tag statements, and run the tests one last time.
         map!(alt_complete!(tag!("#true") | tag!("#t")),
              |_| true)
}

named! { pub scm_false<bool>,
         map!(alt_complete!(tag!("#false") | tag!("#f")),
              |_| false)
}

named! { pub boolean<bool>,
         // the ws macro silently eats surrounding whitespace
         // the alt macro tries a set of parsers and returns the first one to return a success or an error, if any
         ws!(alt!(scm_true | scm_false))
}
```

## Why not use YACC?
The goal of this project is not to create the most efficient lisp parser, but to create a hackable set of libraries that can be used to implement lisps, or languages that borrow conceptes from lisp. The combinatorial structure of the code makes it easier to understand, modify, and embed into other programs, as opposed to the monolithic code of parser generators.
