# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]
[Unreleased]: https://github.com/fastobo/fastobo/compare/v0.1.1...HEAD

## [v0.1.1] - 2020-07-22
[v0.1.1]: https://github.com/fastobo/fastobo/compare/v0.1.0...v0.1.1
### Added
- Support for generic arguments in trait definition.
- Implementation of `#[blanket(derive(Rc))]`.
### Fixed
- Error messages of `#[blanket(derive(Mut))]` referring `Ref` erroneously.
- Implementation of `fn(self)` methods when deriving for `Box`.
### Removed
- Unused `strum` dependency.

## [v0.1.0] - 2020-07-21
[v0.1.0]: https://github.com/fastobo/fastobo/compare/3e6065c9...v0.1.0
Initial release.
