/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod array;
mod dictionary;
mod extend_buffer;
mod packed_array;

// Re-export in godot::builtin.
pub(crate) mod containers {
    pub use super::array::{Array, VariantArray};
    pub use super::dictionary::Dictionary;
    pub use super::packed_array::*;
}

// Re-export in godot::builtin::iter.
pub(crate) mod iterators {
    pub use super::array::Iter as ArrayIter;
    pub use super::dictionary::Iter as DictIter;
    pub use super::dictionary::Keys as DictKeys;
    pub use super::dictionary::TypedIter as DictTypedIter;
    pub use super::dictionary::TypedKeys as DictTypedKeys;
}
