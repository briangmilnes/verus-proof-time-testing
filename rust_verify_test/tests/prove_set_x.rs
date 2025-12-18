#[macro_use]
mod common;
use common::*;

// Test using library's SetX - use specs not exec in assertions
test_verify_one_file! {
    #[test] test_set_x_empty verus_code! {
        use vstd::prelude::*;
        use verus_proof_time_testing::set_x::*;

        fn test_empty() {
            let s: SetX<i32> = SetX::empty();
            proof { assert(s@ == Set::<i32>::empty()); }
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] test_set_x_insert verus_code! {
        use vstd::prelude::*;
        use verus_proof_time_testing::set_x::*;

        fn test_insert() {
            let mut s: SetX<i32> = SetX::empty();
            let _ = s.insert(42);
            proof { assert(s@.contains(42)); }
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] test_set_x_insert_two verus_code! {
        use vstd::prelude::*;
        use verus_proof_time_testing::set_x::*;

        fn test_insert_two() {
            let mut s: SetX<i32> = SetX::empty();
            let _ = s.insert(1);
            let _ = s.insert(2);
            proof {
                assert(s@.contains(1));
                assert(s@.contains(2));
                assert(!s@.contains(3));
            }
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] test_set_x_mem verus_code! {
        use vstd::prelude::*;
        use verus_proof_time_testing::set_x::*;

        fn test_mem() {
            let mut s: SetX<i32> = SetX::empty();
            s.insert(10);
            let found = s.mem(&10);
            let not_found = s.mem(&20);
            proof {
                assert(found);
                assert(!not_found);
            }
        }
    } => Ok(())
}
