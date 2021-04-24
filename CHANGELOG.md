# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Implement `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq` and `Default` traits for hasher structs.

### Changed
- [breaking-change] add `fn` and `struct` keywords in macros to make invocations
  evocative of the output.

## [0.1.0] - 2020-08-09

Initial release to crates.io.

[Unreleased]: https://github.com/eldruin/embedded-crc-macros-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/eldruin/embedded-crc-macros-rs/releases/tag/v0.1.0