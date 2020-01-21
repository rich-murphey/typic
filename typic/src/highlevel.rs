//! The byte-level representation of a type.

pub mod coproduct;
pub mod padding;
pub mod product;

pub use product::{Cons as PCons, Nil as PNil};

pub trait Type {
    /// `align(N)`
    type Align;

    /// `packed(N)`
    type Packed;

    /// An abstract representation of the type's structure.
    type HighLevel;
}

pub type HighLevelOf<T> = <T as Type>::HighLevel;
pub type ReprAlignOf<T> = <T as Type>::Align;
pub type ReprPackedOf<T> = <T as Type>::Packed;
