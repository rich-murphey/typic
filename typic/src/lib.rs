#![feature(marker_trait_attr)]
#![feature(const_generics)]
pub use typic_derive::repr;

mod alignedto;
mod hlist;
mod layout;
mod transmutation;

pub use layout::Layout;

/// Types used to represent the structure of compound types.
/// For internal use.
pub mod structure {
    pub use frunk_core::hlist::{HCons as Fields, HList as FieldList, HNil as Empty};
    pub use frunk_core::coproduct::{Coproduct as Variants, CNil as None};
}

/// Marker types for the padding mode of compound types.
/// For internal use.
pub mod padding {
    /// A marker indicating that a compound type is `#[repr(packed)]`
    pub struct Packed;

    /// A marker indicating that a compound type is not `#[repr(packed)]`.
    pub struct Padded;

    /// A trait defining the set of possible padding modes.
    pub trait Padding {}

    impl Padding for Packed {}

    impl Padding for Padded {}
}

/// Types for safe transmutation.
///
/// ## Examples
/// ### Unrestricted and Restricted Transmutations
/// ```ignore
/// use typic::{self, transmute::{Invariants, Valid, TransmuteFrom}};
///
/// #[typic::repr(C)]
/// #[derive(Default)]
/// struct Struct1 {
///   a: u16,
///   b: u16,
/// }
///
/// // If all fields are public, it is assumed that there are no additional
/// // invariants placed on the fields beyond what they individually have.
/// #[typic::repr(C)]
/// #[derive(Default)]
/// struct Struct2 {
///   pub a: u16,
///   pub b: u8,
///   pub c: u8,
/// }
///
/// // We can transmute safely and without checks from `Struct1` to `Struct2`.
/// let _ = Struct2::transmute_from(Struct1::default());
///
/// // Let's place some invariants on `Struct`.
/// unsafe impl Invariants for Struct1 {
///     type Error = &'static str;
///
///     #[inline(always)]
///     fn check(candidate: &Self::Candidate) -> Result<Valid, Self::Error>
///     where
///         Self: Sized,
///     {
///         if candidate.a % 2 == 0 {
///           Ok(Valid)
///         } else {
///           Err("`a` must be even")
///         }
///     }
/// }
///
/// // Now let's try to go in the other direction:
/// assert!(Struct1::try_transmute_from(Struct2 {a: 0, b: 0, c: 0}).is_ok());
/// assert!(Struct1::try_transmute_from(Struct2 {a: 1, b: 0, c: 0}).is_err());
/// ```
///
/// ### Lifetime Contraction
/// ```ignore
/// use static_assertions::*;
/// use typic::{self, transmute::{Invariants, Valid, TransmuteFrom}};
///
/// fn contract<'long, 'short>(long: &'long u8) -> &'short u8
/// where 'long: 'short
/// {
///   TransmuteFrom::<&'short u8>::transmute_from(long)
/// }
/// ```
///
/// ### Lifetime Expansion
/// Typic cannot be used to expand lifetimes. This produces a compilation error:
/// ```compile_fail
/// use static_assertions::*;
/// use typic::{self, transmute::{Invariants, Valid, TransmuteFrom}};
///
/// fn expand<'short>(short: &'short u8) -> &'static u8
/// {
///   <&'static u8 as TransmuteFrom::<&'short u8>>::transmute_from(short)
/// }
/// ```
pub mod transmute {
    pub use crate::transmutation::{Invariants, TransmuteFrom, Valid};

    /// A candidate of a type is a doppelganger sharing that type's structure, but
    /// not its methods or invariants.
    pub trait Candidate {
        type Candidate;
    }
}

/// A generic representation of a type.
pub trait Type: transmute::Candidate {
    /// The padding mode of the type.
    type Padding: padding::Padding;

    /// An abstract representation of the type's structure.
    type Representation;
}

fn foo() {
  use frunk_core::*;

  #[derive(Default)]
  struct T<const N: usize> {
    marker: core::marker::PhantomData<[(); N]>
  };

  type Tags = Coprod!(T<{1}>, T<{2}>, T<{3}>);

  let a = Tags::inject(<T<{1}>>::default());
  let a = Tags::inject(<T<{3}>>::default());
  

  type I32BoolF32 = Coprod!(i32, bool, f32);
  type I32F32 = Coprod!(i32, f32);

  let co1 = I32BoolF32::inject(42_f32);
  let co2 = I32BoolF32::inject(true);

  let sub1: Result<Coprod!(i32, f32), _> = co1.subset();
  let sub2: Result<Coprod!(i32, f32), _> = co2.subset();
  assert!(sub1.is_ok());
  assert!(sub2.is_err());

  // Turbofish syntax for specifying the target subset is also supported.
  // The Indices parameter should be left to type inference using `_`.
  assert!(co1.subset::<Coprod!(i32, f32), _>().is_ok());
  assert!(co2.subset::<Coprod!(i32, f32), _>().is_err());

  // Order doesn't matter.
  assert!(co1.subset::<Coprod!(f32, i32), _>().is_ok());
}

//frunk_core::coproduct::Coproduct<frunk_core::hlist::HCons<u8, frunk_core::hlist::HNil>, frunk_core::coproduct::CNil>


