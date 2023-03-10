// Copyright 2018 The proptest developers
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
#[macro_use]
extern crate proptest_derive;

fn main() {}

// Show non-fatal:
#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors:
                            //~| [proptest_derive, E0033]
                            //~| [proptest_derive, E0008]
enum T0<#[proptest(skip)] T> {
    #[proptest(weight = 4294967290)]
    V0(T),
    #[proptest(weight = 5)]
    V1,
    V2,
}
