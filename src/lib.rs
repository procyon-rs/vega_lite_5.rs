// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This crate provides a thin wrapper over Vega-Lite v4, as well as a few helpers to help ingest data from
//! various sources and builders to help build graphs. See
//! [examples](https://github.com/procyon-rs/vega_lite_4.rs/tree/master/examples) for more help on how to use it.
//!
//! ## Documentation Disclaimer
//!
//! Most of this crate has been generated from the json schema of Vega-Lite, so is the documentation on Vega-Lite
//! types. A lot of struct are documented as accepting a string but are actually not. See
//! [`FontWeight`](enum.FontWeight.html) for an example. Those are usually converted to an enum, or may have a
//! `String` variant wrapping the actual string (for example, [`Month`](enum.Month.html)). In those cases,
//! `From<String>` is implemented for the enum, and will automatically wrap the string in the correct variant.

#![deny(
    warnings,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]

mod data;
mod removable_value;
mod schema;
mod string;

#[cfg(feature = "show_vega")]
mod show_vega;
#[cfg(feature = "show_vega")]
pub use showata::Showable;

pub use data::*;
pub use removable_value::RemovableValue;
pub use schema::*;
//pub use string::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_should_not_failed_on_empty() {
        let _chart: Vegalite =
            serde_json::from_str("{}").expect("to support empty json object: {}");
    }
}
