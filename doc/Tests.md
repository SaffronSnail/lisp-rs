# Unit Tests
In lisp-rs unit tests are divided into two categories: core tests and edge
cases. Core tests check that a piece of functionality works properly under
perfect conditions. For example, a core test for a parsing function checks that
the correct result is returned when it receives well-formed input. These tests
are useful for demonstrating the intended use of the function, so they should
always be placed in documentation.

Edge cases check what happens when conditions are less than ideal. The unit
under test may be expected to deal with the conditions or not. For instance,
edge cases for a parsing function include not only malformed input, which the
function may not be expected to handle, but also expected input with whitespaces
around it or expected input with data left over for further processing, which
the parser should handle gracefully.
