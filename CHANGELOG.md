# Changelog

## v0.0.5

[compare changes](https://github.com/rhaiscript/rhai-url/compare/v0.0.4...v0.0.5)

### Changes

Fixed the build.rs script to handle newer versions of Rhai starting from 1.16. In particular, the JSON metadata output from Rhai now contains other sections in addition to functions, which causes build.rs to fail.

## v0.0.4

[compare changes](https://github.com/rhaiscript/rhai-url/compare/v0.0.3...v0.0.4)

### Changes

We have update `query_gets` to return a `rhai::Array` in this release, therefore we have introduced the `array` feature (enabled by default).

- `query_gets` return a `rhai::Array` instead of a `Vec<ImmutableString>`

## v0.0.3

[compare changes](https://github.com/rhaiscript/rhai-url/compare/v0.0.2...v0.0.3)

No API changes, the documentation has been updated.

## v0.0.2

[compare changes](https://github.com/rhaiscript/rhai-url/compare/v0.0.1...v0.0.2)

No API changes, the documentation has been updated.

We have also improved the [tests](tests/url.rs)!

## v0.0.1

First Release
