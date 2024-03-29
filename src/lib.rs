// Copyright 2019 Barret Rennie.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! An API to interact with [Searchfox](https://searchfox.org).
//!
//! Searchfox is an index used for Mozilla's source code repositories,
//! including mozilla-central where Firefox is developed.

mod api;

pub use crate::api::response::*;
