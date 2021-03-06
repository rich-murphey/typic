//! Compute the byte-level layout from a generic representation of a type.

use crate::private::num::{Unsigned, U0};

pub mod field;
pub mod primitives;
pub mod product;

pub trait IntoByteLevel<ReprAlign, ReprPacked, Visibility, Offset = U0> {
    /// The byte-level representation of the type.
    type Output;

    /// The size of the type.
    type Offset: Unsigned;

    /// The actual alignment of the type.
    type Align: Unsigned;
}
