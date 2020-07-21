# 🧣 `blanket` [![Star me](https://img.shields.io/github/stars/althonos/blanket.svg?style=social&label=Star&maxAge=3600)](https://github.com/althonos/blanket/stargazers)

*A simple macro to derive blanket implementations for your traits.*

[![TravisCI](https://img.shields.io/travis/com/althonos/blanket/master.svg?maxAge=600&style=flat-square)](https://travis-ci.com/althonos/blanket/branches)
[![Codecov](https://img.shields.io/codecov/c/gh/althonos/blanket/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/althonos/blanket)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/blanket)
[![Crate](https://img.shields.io/crates/v/blanket.svg?maxAge=600&style=flat-square)](https://crates.io/crates/blanket)
[![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/blanket)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/blanket.rs/blob/master/CHANGELOG.md)
[![GitHub issues](https://img.shields.io/github/issues/althonos/blanket.svg?style=flat-square&maxAge=600)](https://github.com/althonos/blanket/issues)

## 🗺️ Overview

The Rust standard library has plenty of traits, but they shine in how well
they integrate with new types. Declare an implementation of
[`std::io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html) for
a type `T`, and you also get it for `&mut T` and `Box<T>`! This however
translates into a [lot of boilerplate code](https://doc.rust-lang.org/src/std/io/impls.rs.html#49-79)
that can be hard to maintain, which is why many crates don't over the same
convenience implementations.

This is where `blanket` comes in! This crate helps you build the same kind
of blanket implementations for your own traits with as least additional code
as possible: in fact, this is as close as what a `derive` macro would look
like for a `trait` item.

## 🔌 Usage

`blanket` exports a single eponymous attribute macro, which can be imported
simply after the crate has been added to the `Cargo.toml` dependencies:

```rust
extern crate blanket;
use blanket::blanket;
```

### `#[blanket(derive(...))]`

Use this macro attribute to derive a blanket implementation for a trait,
provided the trait methods fit the constraints for that derive, such as
only declaring methods with `&self` of `&mut self` as their receiver.

Given a trait `T`, the following derives are supported:

- **`Box`**: implement `T` for any `Box<X>` where `X` implements `T`.
- **`Ref`**: implement `T` for any `&X` where `X` implements `T`.
  *This requires all trait methods to be declared as `fn (&self, ...)`*

### `#[blanket(default = "...")]`

`blanket` can delegate default implementations of trait methods to functions
of another module. This can be useful for some traits such as
[visitors](https://github.com/rust-unofficial/patterns/blob/master/patterns/visitor.md)
to provide a default behaviour as an external function, such as what
[`syn::visit`](https://docs.rs/syn/latest/syn/visit/index.html) is doing.

The following example implements a very simple visitor trait for types
able to process a `&str` char-by-char.

```rust,ignore
extern crate blanket;
use blanket::blanket;

#[blanket(default = "visitor")]
trait Visitor {
    fn visit_string(&self, s: &str);
    fn visit_char(&self, c: char);
}

mod visitor {
    use super::Visitor;

    pub fn visit_string<V: Visitor + ?Sized>(v: &V, s: &str) {
        for c in s.chars() {
            v.visit_char(c);
        }
    }

    pub fn visit_char<V: Visitor + ?Sized>(v: &V, c: char) {}
}
```

`blanket` will check that all methods are declared without a default block,
and then create a default implementation for all of the declared methods,
generating the following code:

```rust,ignore
trait Visitor {
    fn visit_string(&self, s: &str) {
      visitor::visit_string(self, s)
    }
    fn visit_char(&self, c: char) {
      visitor::visit_char(self, c)
    }
}
```

## 📋 Changelog

This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html)
and provides a [changelog](https://github.com/althonos/blanket/blob/master/CHANGELOG.md)
in the [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) format.


## 📜 License

This library is provided under the open-source
[MIT license](https://choosealicense.com/licenses/mit/).
