//! Simple hash set wrapper demonstrating Verus patterns:
//! - Hash set with View trait
//! - Iterator implementing std::iter::Iterator INSIDE verus! (no duplication)
//! - Macro for literals
//!
//! ## Key Pattern: Iterator Without Duplication
//!
//! Verus can verify `impl Trait` blocks when inside `verus!`. This means you can
//! implement `std::iter::Iterator` directly inside `verus!` with specs on `next()`:
//!
//! ```ignore
//! verus! {
//!     impl<'a, T: ...> std::iter::Iterator for MyIter<'a, T> {
//!         type Item = &'a T;
//!         
//!         fn next(&mut self) -> (result: Option<&'a T>)
//!             ensures ({ ... specs ... })
//!         {
//!             self.inner.next()
//!         }
//!     }
//! }
//! // NO separate impl needed outside verus!
//! ```
//!
//! This works for both Verus verification AND `cargo test` without code duplication.

use vstd::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;
#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::SetIterAdditionalSpecFns;

verus! {

pub open spec fn valid_set_element<T: View + Clone + Eq + Hash>() -> bool {
    obeys_key_model::<T>()
}

/// Simple hash set wrapper using std::collections::HashSet
#[verifier::reject_recursive_types(T)]
pub struct SetX<T: View + Clone + Eq + Hash> {
    pub m: HashSet<T>,
}

impl<T: View + Clone + Eq + Hash> View for SetX<T> {
    type V = Set<<T as View>::V>;
    open spec fn view(&self) -> Self::V { self.m@.map(|k: T| k@) }
}

/// Iterator over SetX - wraps std::collections::hash_set::Iter
#[verifier::reject_recursive_types(T)]
pub struct SetXIter<'a, T: View + Clone + Eq + Hash> {
    pub inner: std::collections::hash_set::Iter<'a, T>,
}

impl<'a, T: View + Clone + Eq + Hash> View for SetXIter<'a, T> {
    type V = (int, Seq<T>);
    open spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
}

/// Implement std::iter::Iterator INSIDE verus! block.
/// This provides specs for verification AND works with cargo test.
/// No separate impl outside verus! needed - avoids code duplication.
impl<'a, T: View + Clone + Eq + Hash> std::iter::Iterator for SetXIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> (result: Option<&'a T>)
        ensures ({
            let (old_idx, old_seq) = old(self)@;
            match result {
                None => self@ == old(self)@ && old_idx >= old_seq.len(),
                Some(elem) => {
                    let (new_idx, new_seq) = self@;
                    0 <= old_idx < old_seq.len() &&
                    new_seq == old_seq &&
                    new_idx == old_idx + 1 &&
                    elem == old_seq[old_idx]
                },
            }
        })
    {
        self.inner.next()
    }
}

impl<T: View + Clone + Eq + Hash> SetX<T> {
    #[verifier::external_body]
    pub fn empty() -> (s: Self)
        requires valid_set_element::<T>()
        ensures s@ == Set::<<T as View>::V>::empty()
    {
        SetX { m: HashSet::new() }
    }

    #[verifier::external_body]
    pub fn size(&self) -> (n: usize)
        ensures n == self@.len()
    {
        self.m.len()
    }

    #[verifier::external_body]
    pub fn mem(&self, x: &T) -> (b: bool)
        requires valid_set_element::<T>()
        ensures b == self@.contains(x@)
    {
        self.m.contains(x)
    }

    #[verifier::external_body]
    pub fn insert(&mut self, x: T) -> (inserted: bool)
        requires valid_set_element::<T>()
        ensures self@ == old(self)@.insert(x@), inserted == !old(self)@.contains(x@)
    {
        self.m.insert(x)
    }

    #[verifier::external_body]
    pub fn iter<'a>(&'a self) -> (it: SetXIter<'a, T>)
        requires valid_set_element::<T>()
        ensures it@.0 == 0, it@.1.map(|i: int, k: T| k@).to_set() == self@
    {
        SetXIter { inner: self.m.iter() }
    }
}

} // verus!

/// Macro for SetX literals
#[macro_export]
macro_rules! set_x_lit {
    () => { $crate::set_x::SetX::empty() };
    ($($x:expr),* $(,)?) => {{
        let mut s = $crate::set_x::SetX::empty();
        $( let _ = s.insert($x); )*
        s
    }};
}
