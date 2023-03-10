//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Modifications Copyright Kani Contributors
// See GitHub history for details.

//! Defines macros for product type creation, extraction, and the type signature
//! itself. This version uses `frunk_core`. This mechanism is used to be very
//! loosely coupled with `frunk_core` so that only `lib.rs` has to be changed
//! in the event that Rust gets tuple-variadic generics.

macro_rules! product_type {
    ($factor: ty) => {
        Hlist![$factor]
    };
    ($($factor: ty),*) => {
        Hlist![$( $factor, )*]
    };
    ($($factor: ty),*,) => {
        Hlist![$( $factor, )*]
    };
}

macro_rules! product_pack {
    ($factor: expr) => {
        hlist![$factor]
    };
    ($($factor: expr),*) => {
        hlist![$( $factor ),*]
    };
    ($($factor: expr),*,) => {
        hlist![$( $factor ),*]
    };
}

macro_rules! product_unpack {
    ($factor: pat) => {
        hlist_pat![$factor]
    };
    ($($factor: pat),*) => {
        hlist_pat![$( $factor ),*]
    };
    ($($factor: pat),*,) => {
        hlist_pat![$( $factor ),*]
    };
}
