# Changelog

All notable changes to this project are documented here.

The project follows Semantic Versioning while the public DSL is still young:
patch releases should not intentionally break existing templates, and minor
releases may add syntax or tighten diagnostics.

## 0.5.0


### Added


- Support method calls in component markup



## 0.4.0


### Added


- Add generic components support



## 0.3.0


### Maintenance


- Add CI release automation and project hardening


## Unreleased

- Added CI coverage for formatting, Clippy, tests, and documentation.
- Added release automation for crates.io publishing.
- Added Taskfile and git-cliff configuration for changelog generation.
- Added security and syntax documentation to the README.
- Added runnable examples for basic markup, components, and fragment rendering.

## 0.2.0

- Added short-form attribute values for function calls, field chains, and string
  literal attribute names.
- Standardized statement blocks as Rust block expressions: `({ ... })`.
- Replaced side-by-side README example tables with vertical Rust/HTML examples.
- Added compile-fail doctests for rejected statement-only parentheses.

## 0.1.1

- Initial public crate metadata and README.
