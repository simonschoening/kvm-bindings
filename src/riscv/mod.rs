// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "fam-wrappers")]
mod fam_wrappers;

#[allow(clippy::all)]
mod bindings_v5_12_0;

pub mod bindings {
    pub use super::bindings_v5_12_0::*;

    #[cfg(feature = "fam-wrappers")]
    pub use super::fam_wrappers::*;
}
