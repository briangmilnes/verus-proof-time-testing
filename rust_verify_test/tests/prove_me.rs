#[macro_use]
mod common;
use common::*;

// Test importing and using our library's minmax module
test_verify_one_file! {
    #[test] test_use_library_max_x verus_code! {
        use vstd::prelude::*;
        use verus_proof_time_testing::minmax::*;

        fn test_max_x() {
            let m = max_x(3, 5);
            assert(m == 5);
            assert(m == spec_max_x(3, 5));
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] test_use_library_min_x verus_code! {
        use vstd::prelude::*;
        use verus_proof_time_testing::minmax::*;

        fn test_min_x() {
            let m = min_x(10, 3);
            assert(m == 3);
            assert(m == spec_min_x(10, 3));
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] test_use_library_lemmas verus_code! {
        use vstd::prelude::*;
        use verus_proof_time_testing::minmax::*;

        proof fn test_lemmas() {
            // Use the library's lemmas
            lemma_max_x_comm(10u32, 20u32);
            lemma_min_x_comm(10u32, 20u32);
            lemma_max_x_ge_min_x(5u32, 15u32);
            
            // Verify the properties hold
            assert(spec_max_x(10, 20) == spec_max_x(20, 10));
            assert(spec_min_x(10, 20) == spec_min_x(20, 10));
            assert(spec_max_x(5, 15) >= spec_min_x(5, 15));
        }
    } => Ok(())
}

// Original standalone tests (no library import needed)
test_verify_one_file! {
    #[test] test_assert_true verus_code! {
        use vstd::prelude::*;

        fn test() {
            assert(true);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] test_arithmetic verus_code! {
        use vstd::prelude::*;

        proof fn double_is_even(n: nat)
            ensures (2 * n) % 2 == 0
        {
        }

        proof fn add_comm(a: int, b: int)
            ensures a + b == b + a
        {
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] test_loop_simple verus_code! {
        use vstd::prelude::*;

        fn count_up(n: u32) -> (count: u32)
            requires n < 1000
            ensures count == n
        {
            let mut i: u32 = 0;
            while i < n
                invariant i <= n
                decreases n - i
            {
                i = i + 1;
            }
            i
        }
    } => Ok(())
}

// Test that a wrong assertion fails
test_verify_one_file! {
    #[test] test_wrong_assertion_fails verus_code! {
        use vstd::prelude::*;

        fn wrong() {
            assert(1 + 1 == 3); // FAILS
        }
    } => Err(err) => assert_one_fails(err)
}

// Test precondition failure
test_verify_one_file! {
    #[test] test_precondition_fails verus_code! {
        use vstd::prelude::*;

        fn requires_positive(x: i32)
            requires x > 0
        {
        }

        fn caller() {
            requires_positive(-1); // FAILS
        }
    } => Err(err) => assert_one_fails(err)
}

// Test postcondition failure
test_verify_one_file! {
    #[test] test_postcondition_fails verus_code! {
        use vstd::prelude::*;

        fn broken() -> (r: u32)
            ensures r > 10 // FAILS
        {
            5
        }
    } => Err(err) => assert_one_fails(err)
}

// This test ACTUALLY FAILS - verification fails but test expects success.
test_verify_one_file! {
    #[test] one_proof_test_that_really_fails verus_code! {
        use vstd::prelude::*;

        fn this_is_wrong() {
            assert(1 + 1 == 3); // Verus FAILS this
        }
    } => Ok(())  // But test expects success - real test failure
}
