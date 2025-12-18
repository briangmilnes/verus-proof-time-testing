//! Min/Max functions for proof time testing

use vstd::prelude::*;

verus! {

/// Spec: maximum of two values
pub open spec fn spec_max_x(a: u32, b: u32) -> u32 {
    if a >= b { a } else { b }
}

/// Spec: minimum of two values
pub open spec fn spec_min_x(a: u32, b: u32) -> u32 {
    if a <= b { a } else { b }
}

/// Maximum of two values
pub fn max_x(a: u32, b: u32) -> (result: u32)
    ensures result == spec_max_x(a, b)
{
    if a >= b { a } else { b }
}

/// Minimum of two values
pub fn min_x(a: u32, b: u32) -> (result: u32)
    ensures result == spec_min_x(a, b)
{
    if a <= b { a } else { b }
}

/// Proof: max is commutative
pub proof fn lemma_max_x_comm(a: u32, b: u32)
    ensures spec_max_x(a, b) == spec_max_x(b, a)
{
}

/// Proof: min is commutative  
pub proof fn lemma_min_x_comm(a: u32, b: u32)
    ensures spec_min_x(a, b) == spec_min_x(b, a)
{
}

/// Proof: max >= min
pub proof fn lemma_max_x_ge_min_x(a: u32, b: u32)
    ensures spec_max_x(a, b) >= spec_min_x(a, b)
{
}

} // verus!
