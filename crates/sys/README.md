# `pherrous-sys`

This crate provides a set of low-level bindings to PHP's C API.

By default, it exposes all of PHP's Extension API, but also offers support for the `embed` SAPI.

It is not recommended to use this crate directly, as it is very low-level and error-prone.
Instead, use one of the higher-level Pherrous crates that build on top of this one.

## Features

- `embed`: Build with support for the `embed` SAPI, exposed through the `embed` module.
- `zts`: Build with PHP's ZTS-mode enabled.
- `php83`: Build against PHP 8.3's headers (default).
- `php84`: Build against PHP 8.4's headers.
