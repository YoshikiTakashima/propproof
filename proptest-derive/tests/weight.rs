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
use proptest::prelude::Arbitrary;
use proptest_derive::Arbitrary;

#[derive(Debug, Arbitrary)]
enum T1 {
    #[proptest(weight = "3")]
    V1,
    V2,
}

#[derive(Debug, Arbitrary)]
enum T2 {
    V1,
    #[proptest(weight("3"))]
    V2,
}

#[derive(Debug, Arbitrary)]
enum T3 {
    #[proptest(weight(3))]
    V1,
    V2,
}

#[derive(Debug, Arbitrary)]
enum T4 {
    V1,
    #[proptest(weight = 3)]
    V2,
}

#[test]
fn asserting_arbitrary() {
    fn assert_arbitrary<T: Arbitrary>() {}

    assert_arbitrary::<T1>();
    assert_arbitrary::<T2>();
    assert_arbitrary::<T3>();
    assert_arbitrary::<T4>();
}
