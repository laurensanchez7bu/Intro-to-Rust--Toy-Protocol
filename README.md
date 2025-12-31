Goal: Get acquainted with Rust development and serialization/deserialization

## About Rust

Rust isn't strictly object-oriented like you might be used to. Rather, it can be used in many different ways, depending
on your style. Primarily, it leverages concepts from functional programming.

What does that mean for you, as the programmer? No boilerplate. No header files or class definitions. Abstractions add
zero run-time cost. Below are some tips of mine to help you get acquainted with what I think is an amazing language.

### Structs and impls

We define behavior through structs and implementations (impls). Take the following snippet.

```rust
struct Door {
    opened: bool,
    locked: bool,
}

impl Door {
    fn open(&mut self) {
        if !self.locked {
            self.opened = true;
        }
    }

    fn close(&mut self) {
        self.opened = false;
    }

    fn lock(&mut self) {
        self.locked = true;
    }

    fn unlock(&mut self) {
        self.locked = false;
    }
}
```

The struct definition is akin to those of pure C. We define a name for each struct element, and specify its data type.
The impl simply defines a list of functions that we can call for an instance of our struct.

Now what if we want to define shared behavior for multiple structs? Let's say we add a Window struct, which can be
opened/closed, but *not locked/unlocked*.

```rust
trait OpenClose {
    fn open(&mut self);

    fn close(&mut self);
}
struct Door {
    opened: bool,
    locked: bool,
}

impl OpenClose for Door {
    fn open(&mut self) {
        if !self.locked {
            self.opened = true;
        }
    }

    fn close(&mut self) {
        self.opened = false;
    }
}

impl Door {
    fn lock(&mut self) {
        self.locked = true;
    }

    fn unlock(&mut self) {
        self.locked = false;
    }
}

struct Window {
    opened: bool,
    blinds_down: bool,
}

impl OpenClose for Window {
    fn open(&mut self) {
        if !self.blinds_down {
            self.opened = true;
        }
    }

    fn close(&mut self) {
        if !self.blinds_down {
            self.opened = false;
        }
    }
}

impl fmt::Display for Door {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Door: opened={}, locked={}", self.opened, self.locked)
    }
}

impl fmt::Display for Window {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Window: opened={}, blinds_down={}", self.opened, self.blinds_down)
    }
}
```

We define a trait, `OpenClose`, and implement it for our Door and Window structs. The implementations differ, but the
semantic meaning conveyed by calling `open()` and `close()` will be similar for the programmer. We can implement all
sorts of traits, like `Display`, and two structs that implement one shared trait don't have to implement all the *same*
shared traits. That is, there's not a strict hierarchy of inheritance.

### Safety

Rust is an extremely memory-safe language. This is enforced by the compiler, primarily through the concept of RAII (
Resource acquisition is initialization). This means that you cannot access any memory address that is not associated
with a piece of data that is in scope for your program. Likewise, as soon as a piece of data is no longer accessible to
the program, it is deallocated.

This means that (safe) Rust code avoids a plethora of memory issues that are
either [footguns](https://en.wiktionary.org/wiki/footgun#:~:text=Noun,shooting%20themselves%20in%20the%20foot.) in
languages like C++, or that are handled non-deterministically by a garbage collector in languages like Java and Go. You
can't segfault, you can't have a doublefree, you can't perform an array access with an invalid index.

This safety extends even to parallel programming. Multithreading in most languages is typically a dangerous endeavor (
what happens if you try writing to one memory location from two threads at the same time?), but Rust makes it easy to
ensure safety. Supporting parallel operations will be crucial for later projects.

#### A note on `unsafe {}`

You can get around some of Rust's safety rules by manually bookending a block of code with `unsafe { }`. As a
programmer, you should only do this if *absolutely necessary*. Such a situation would be when you know, absolutely, that
an operation will be safe, but you can't prove it to the compiler. Unsafe code is often found in the open-source
libraries that we use, but that code has (ideally) been vetted for memory safety.

**For this course, you should never use `unsafe { }`.**

### Lifetimes

One unique aspect of Rust is the lifetime system. Typically, you can't hold a reference to a piece of data that is not
in scope. If you do, the Rust compiler enforces the requirement that your *reference* will be "alive" at least as long
as the data that it's pointing to.

Most of the time, lifetimes are inferred by the Rust compiler and elided from you, meaning that you don't have to write
them down manually. If you find yourself getting into a situation where you're having to specify a lot of lifetimes, *
*you're probably going down an incorrect design path.** Maybe you're thinking too strictly within the object-oriented
paradigm. Try restructuring your code such that manual lifetimes are not necessary, and your life will become a lot
easier.

### Fighting with the compiler

It can be difficult to get Rust code to compile, but that's a good thing. If anything is wrong, the compiler will not
only complain, but it will offer helpful advice for how to fix your code. Simply following the advice of the compiler
will often get you to a state where your code does indeed compile and run. This makes debugging your code much easier.

### The build system

Rust has a nice, modern build system in the form of `cargo`. Cargo manages your dependencies (defined in `Cargo.toml`),
command line arguments, and compiler optimization levels. You can browse dependencies
on [crates.io](https://www.crates.io). My own crates are available [here](https://crates.io/users/ac-freeman).

### The Rust Books

With that basic overview, I highly recommend that you check out the online "
books" [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
and [The Rust Programming Language](https://doc.rust-lang.org/book/). These resources will prove invaluable over the
course of the semester.

# Project1 instructions

As stated above, your goal here is to get familiar with some basic Rust programming and testing, as well as learn how to
serialize and deserialize data.

I've provided some scaffolding for the project, including the following:

- `User` struct. This is the data that we'll be (de)serializing. It contains some self-explanatory attributes about a
  user of some arbitrary service.
- `UserError` enum. This provides meaningful error types for a programmer who leverages our (de)serialization code.
- `main()` function. This includes calls to serialize and deserialize a `User` struct.
- `tests` module. This demonstrates how to write unit tests in Rust. You'll have to write additional tests to make sure
  that the protocol is correctly implemented. You can run all the tests for a given project with the
  command `cargo test`.
- A `Cargo.toml` file, specifying the dependencies needed for this project. You may not use additional dependencies (for
  future projects, you can)

## The User protocol

We're using a "toy" protocol for "packets" of `User` data. It doesn't bear real-world significance, but it will help you
get used to implementing a protocol specification. Here is the spec:

- The first 3 octets (bytes) form the packet header
    - The first 2 octets of the header are a `u16` (unsigned 16-bit integer) conveying the `User` ID. This field `MUST`
      be in Network Byte Order (Big-Endian).
    - Bits 16-20 form are a 5-bit unsigned integer conveying the `access_level` of the `User`. If you try to serialize a
      packet with an `access_level` that does not fit within 5 bits, you must return a `FieldTooLarge` error
    - Bit 21 is a boolean `active` flag.
    - Bits 22-23 are unused padding bits. You `SHOULD` make these bits 0.
        - > Note that `SHOULD` here conveys a recommendation, but not a requirement. You will not have points taken off
          if you do something different here.
- Octets 4+ form the body of the packet
    - The body `MUST` be of variable length
    - The first octet `MUST` convey the length in bytes, *n* of the `name` as a `u8`
    - The following *n* octets `MUST` express the `name`
        - You `MAY` serialize the `name` as a utf8 byte array
        - You `MAY` disallow the encoding of non-ASCII characters
            - > `MAY` here conveys an optional part of the spec. I'm not recommending one thing or the other. Think
              about what the pros and cons are of doing this.
        - You `MUST` return a `FieldTooSmall` error if attempting to serialize a `name` with length 0
        - You `MUST` return a `FieldTooLarge` error if attempting to serialize a `name` with length greater than what
          can be properly deserialized with this protocol
        - You `MUST` return a `MalformedBody` error if attempting to deserialize a `name` with length 0
    - The next octet `MUST` convey the length in bytes, *m* of the `email` as a `u8`
    - The following *n* octets `MUST` express the `email`
        - You `SHOULD` serialize the `email` as a utf8 byte array
        - You `MAY` disallow the encoding of non-ASCII characters
        - You `MUST` return a `FieldTooSmall` error if attempting to serialize an `email` with length 0
        - You `MUST` return a `FieldTooLarge` error if attempting to serialize an `email` with length greater than what
          can be properly deserialized with this protocol
        - You `MUST` return a `MalformedBody` error if attempting to deserialize an `email` with length 0
    - You `MUST` return a `MalformedBody` error if attempting to deserialize a packet with length exceeding what
      the `name_length` and `email_length` fields specify

## Tips

Look at the dependencies listed in `Cargo.toml`. Consult the online documentation for those dependencies to figure out
how to use them, and to learn how to do some manual serialization and deserialization.

# Submission

## Questions
- Define "serialize" and "deserialize" in this context.
- In Rust, how do we define shared behavior for multiple "objects" (structs or enums)?
- In the Rust compiler, what _must_ happen when memory is allocated for a variable?
- Why might we want to pack multiple pieces of data into a single byte for a communication protocol?

## What to submit
- Push your working code to the main branch of your GitHub Repository before the deadline
- Edit the README to answer the above questions

## Grading

Below is a prior rubric, although it is subject to change.

- All my tests pass (10 tests)	60%
- Code quality	20%
- Documentation quality	10%
- Readme questions (4 questions)	10%
