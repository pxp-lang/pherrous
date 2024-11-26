# Pherrous

Integrate PHP with your Rust application, or integrate Rust with your PHP application.

Use Pherrous to execute PHP code from Rust using PHP's `embed` SAPI, or bind your Rust code to PHP by writing an extension.

## Overview

Pherrous has multiple components that all work together to provide a simple and expressive API. 

The [`pherrous-sys`](/crates/sys/) crate is responsible for providing a set of raw bindings to PHP's various C APIs. This includes the `embed` SAPI and the extension API. It's not _really_ designed for usage in userland because it can be rather verbose.

The [`pherrous-core`](/crates/core) crate provides a basic set of less verbose APIs. Most of these APIs are wrappers around PHP's C API. The goal is to provide a better set of named structs and functions. It exposes things such as the `Zval` enumeration and various PHP output related functions. It also provides support for embedding and exploring PHAR files inside of your Rust programs when the `phar` feature is enabled.

The [`pherrous-ext`](/crates/ext) crate provides a declarative API for developing PHP extensions in Rust. You can use it to write functions and build classes with Rust.

The [`pherrous-embed`](/crates/embed) crate is used to execute PHP code from Rust. Paired with the `pherrous-ext` crate, you can even register functions and classes that are only available when executing code through the `embed` API. If you enable the `phar` feature flag, you're also able to execute embedded PHARs from your Rust applications.

The [`pherrous-cli`](/crates/cli) crate provides a set of utility commands for generating PHP stubs for your Rust-backed extension, as well as support for bundling a PHP application into a standalone executable.

The top-level [`pherrous`](/src) crate re-exports all of the crates above so that you can use all of the features at once.