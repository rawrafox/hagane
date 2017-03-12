# Hagane: Aurora's Rust / Metal bindings #

Hello and welcome to this work in progress thing!

To build stuff run `rake && cargo build` then you get a folder full of apps to try, you will need Xcode installed and macOS 10.12 (10.11 should mostly work though.)

## Questions ##

 - Is this API stable? No!
 - Is this intended to work roughly like the ObjC API? Yes.
 - Are things going to be split off into crates? Yes.
 - Are you seriously wrapping all of Foundation / Cocoa? It unfortunately seems like that.
 - Is this good enough to write a small demo? Yes, see the examples.
 - Are patches welcome? Probably not right now, but bug reports and constructive critcism most certainly is.

## Basic todo list ##

 - [ ] Split off into various crates.
 - [ ] Some actual tutorials.
 - [ ] Wrap all the things!
 - [ ] Figure out how to handle weak properties.
 - [ ] Write some demoes.
 - [ ] Indexing.
 - [ ] Fast iteration.
 - [ ] Inheriting initializers correctly.
 - [ ] Typed NSArray, NSDictionary, etc.
 - [ ] Actual methods / useful things for the simd stuff.