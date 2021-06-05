# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]

[Unreleased]: https://github.com/althonos/blanket/compare/v0.2.0...HEAD


## [v0.2.0] - 2021-05-06

[v0.2.0]: https://github.com/althonos/blanket/compare/v0.1.5...v0.2.0

### Added
- Implementation for `#[blanket(derive(Arc))]` ([#4](https://github.com/althonos/blanket/pull/4))
- Support for associated type in derived traits ([#6](https://github.com/althonos/blanket/pull/6)).

### Fixed
- Missing features for the `syn` crate preventing the crate to compile without leaking dev-dependencies ([#5](https://github.com/althonos/blanket/pull/5)).


## [v0.1.5] - 2021-05-31

[v0.1.5]: https://github.com/althonos/blanket/compare/v0.1.4...v0.1.5

### Fixed
- Regression in `v0.1.4` causing trait-associated lifetimes to be erased.


## [v0.1.4] - 2021-05-31 - YANKED

[v0.1.4]: https://github.com/althonos/blanket/compare/v0.1.3...v0.1.4

### Fixed
- Generics being erroneously repeated when deriving a trait with 
  bounded generic arguments ([#2](https://github.com/althonos/blanket/issues/2)).


## [v0.1.3] - 2020-10-13

[v0.1.3]: https://github.com/althonos/blanket/compare/v0.1.2...v0.1.3

### Fixed

- Handling of where clauses for traits with generic parameters,
  by @alexanderlinne ([#1](https://github.com/althonos/blanket/pull/1)).


## [v0.1.2] - 2020-07-22

[v0.1.2]: https://github.com/althonos/blanket/compare/v0.1.1...v0.1.2

### Changed

- `syn` now only compiles with [`full`](https://docs.rs/syn/latest/syn/#optional-features)
  feature in release mode.

### Removed
- Unused `darling` dependency.


## [v0.1.1] - 2020-07-22

[v0.1.1]: https://github.com/althonos/blanket/compare/v0.1.0...v0.1.1

### Added

- Support for generic arguments in trait definition.
- Implementation of `#[blanket(derive(Rc))]`.

### Fixed
- Error messages of `#[blanket(derive(Mut))]` referring `Ref` erroneously.
- Implementation of `fn(self)` methods when deriving for `Box`.

### Removed
- Unused `strum` dependency.


## [v0.1.0] - 2020-07-21

[v0.1.0]: https://github.com/althonos/blanket/compare/3e6065c9...v0.1.0

Initial release.
