## Echor

This is an implementation of echo written in Rust for practice purposes.
The book Command-Line Rust from O'Reilly guided my practice,
but I took smaller steps and started from tests
differently than the book happens to.

Note that the `echo` design here is different from real echo,
at the moment.

It gets several default behaviors from flags, such as `-h` and `-v`.
Real `echo` exits 0 and prints nothing if you pass it no strings.
This one exits 1 and prints usage.

We're going to document all of this with tests
as an exercise.
