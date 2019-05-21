# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2019-05-21
### Changed

- `battery` dependency version was updated to `0.7.2`

## [0.2.1] - 2019-03-13
### Fixed

- Incorrect label for state of charge bar

## [0.2.0] - 2019-03-13
### Added
- CLI arguments handling (see `battop --help`) [#1](https://github.com/svartalf/rust-battop/issues/1)
- CLI argument for measurement units `-u/--units` (available options: `human` or `si`)
- CLI argument for delay between updates `-d/--delay` (1 second by default)
- CLI argument for logs verbosity `-v/-vv/../-vvvvv`, logs are written into `stderr`

### Changed
- Temperature graph shows "Unavailable" label if underline device does not provides temperature data
