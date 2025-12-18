# Verus Proof Time Testing

A project demonstrating Verus verification with both runtime tests and proof tests.

## Prerequisites

- Verus built at `~/projects/verus-lang/source/target-verus/release/`
- Rust 1.91.0 (for runtime tests)
- Rust nightly (for proof tests in `rust_verify_test/`)

## Project Structure

```
verus-proof-time-testing/
├── Cargo.toml              # Main crate config (uses vstd)
├── rust-toolchain.toml     # Rust 1.91.0
├── src/
│   ├── lib.rs              # Module bindings
│   ├── minmax.rs           # Verus-verified min/max functions
│   └── set_x.rs            # Hash set wrapper with iterator and macro
├── tests/
│   ├── test_minmax.rs      # Runtime tests for minmax
│   └── test_set_x.rs       # Runtime tests for SetX
├── target/verus/           # Compiled library for proof tests
│   ├── libverus_proof_time_testing.rlib
│   └── verus_proof_time_testing.vir
└── rust_verify_test/       # Proof test harness (nightly)
    ├── Cargo.toml
    ├── rust-toolchain.toml
    └── tests/
        ├── common/mod.rs   # Test harness
        ├── prove_me.rs     # Proof tests for minmax
        └── prove_set_x.rs  # Proof tests for SetX
```

## From Checkout to Running Everything

### 1. Verify `src/` with Verus

```bash
cd ~/projects/verus-proof-time-testing
verus --crate-type=lib src/lib.rs
```

Expected output:
```
verification results:: 6 verified, 0 errors
```

### 2. Run Runtime Tests

These are normal Rust tests in `tests/` that exercise the library at runtime:

```bash
cargo test
```

Expected output:
```
running 7 tests (test_minmax)
running 8 tests (test_set_x)
test result: ok. 15 passed
```

### 3. Build Library for Proof Tests

The proof tests import the library. First, compile it with Verus to produce `.rlib` and `.vir` files:

```bash
mkdir -p target/verus
verus --compile --crate-type=lib --crate-name=verus_proof_time_testing \
    -o target/verus/libverus_proof_time_testing.rlib \
    --export target/verus/verus_proof_time_testing.vir \
    src/lib.rs
```

This creates:
- `target/verus/libverus_proof_time_testing.rlib` — compiled Rust library
- `target/verus/verus_proof_time_testing.vir` — Verus specifications for import

### 4. Run Proof Tests

```bash
cd rust_verify_test
cargo test --test prove_me
cargo test --test prove_set_x
```

Expected output:
```
prove_me: 9 passed; 1 failed (intentional)
prove_set_x: 4 passed
```

## Library Features

### minmax.rs — Simple verified functions

```rust
use verus_proof_time_testing::minmax::*;

let m = max_x(3, 5);  // Returns 5
assert_eq!(m, 5);
```

### set_x.rs — Hash set with iterator and macro

Demonstrates:
- **View trait** for spec-level reasoning
- **Iterator** implementing `std::iter::Iterator` inside `verus!` (no duplication)
- **Conditional compilation** with `#[cfg(verus_keep_ghost)]`
- **Macro** for convenient literals

```rust
use verus_proof_time_testing::set_x::*;
use verus_proof_time_testing::set_x_lit;

// Using the macro
let s = set_x_lit![1, 2, 3];
assert_eq!(s.size(), 3);
assert!(s.mem(&2));

// Using the iterator (works with cargo test)
for &x in s.iter() {
    println!("{}", x);
}
let sum: i32 = s.iter().sum();
```

## Quick Reference

| Task | Command |
|------|---------|
| Verify library | `verus --crate-type=lib src/lib.rs` |
| Runtime tests | `cargo test` |
| Build for proof tests | See step 3 above |
| Proof tests (minmax) | `cd rust_verify_test && cargo test --test prove_me` |
| Proof tests (set_x) | `cd rust_verify_test && cargo test --test prove_set_x` |

## Writing Tests

### Runtime Tests (`tests/`)

Standard Rust tests using the library:

```rust
use verus_proof_time_testing::set_x::*;
use verus_proof_time_testing::set_x_lit;

#[test]
fn test_set_operations() {
    let s = set_x_lit![1, 2, 3];
    assert_eq!(s.size(), 3);
    assert!(s.mem(&2));
}
```

### Proof Tests (`rust_verify_test/tests/`)

Verification tests using `test_verify_one_file!`:

```rust
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
```

### Understanding "Expected Failure" Tests

The `=> Err(err) => assert_one_fails(err)` pattern means "expect verification to fail".
When verification fails as expected, the **test passes** (shows "ok"):

```
test test_wrong_assertion_fails ... ok    ← Verification failed, as expected
```

A test only shows **FAILED** when reality doesn't match expectation:

| Expectation | Reality | Test Result |
|-------------|---------|-------------|
| `=> Ok(())` | Verification succeeds | ok |
| `=> Ok(())` | Verification fails | **FAILED** |
| `=> Err(err) => assert_one_fails(err)` | Verification fails | ok |
| `=> Err(err) => assert_one_fails(err)` | Verification succeeds | **FAILED** |

The test `one_proof_test_that_really_fails` demonstrates the last case.

## Verus Patterns

### Iterator Without Code Duplication

Verus can verify `impl Trait` blocks inside `verus!`. This means you can implement
`std::iter::Iterator` directly inside `verus!` with specs on `next()`:

```rust
verus! {
    impl<'a, T: ...> std::iter::Iterator for SetXIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> (result: Option<&'a T>)
            ensures ({
                // Verus specs here
                let (old_idx, old_seq) = old(self)@;
                match result {
                    None => old_idx >= old_seq.len(),
                    Some(elem) => elem == old_seq[old_idx],
                }
            })
        {
            self.inner.next()
        }
    }
}
// NO separate impl needed outside verus!
```

This works for **both** Verus verification **and** `cargo test` without duplicating the
`next()` implementation.

**Anti-pattern (avoid):**

```rust
verus! {
    impl<'a, T> MyIter<'a, T> {
        pub fn next(&mut self) -> ... { self.inner.next() }  // Verus method
    }
}
// WRONG: Duplicates the body
impl<'a, T> std::iter::Iterator for MyIter<'a, T> {
    fn next(&mut self) -> ... { self.inner.next() }  // Same code again!
}
```

### Conditional Imports

Some `vstd` modules only exist during Verus verification:

```rust
#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::SetIterAdditionalSpecFns;
```

### external_body for stdlib operations

Use `#[verifier::external_body]` for operations that call stdlib methods without Verus specs:

```rust
#[verifier::external_body]
pub fn size(&self) -> (n: usize)
    ensures n == self@.len()
{
    self.m.len()  // HashSet::len() - trust the postcondition
}
```

## Configuration

### Different Verus Build

```bash
VERUS_TARGET_PATH=/path/to/verus/target-verus/release cargo test
```

### Sequential Proof Tests (for timing)

```bash
cd rust_verify_test
RUST_TEST_THREADS=1 cargo test
```

## Run All Steps

A script automates the entire workflow:

```bash
./scripts/walkthesteps.sh

# Or with custom Verus path:
VERUS=/path/to/verus ./scripts/walkthesteps.sh
```

This cleans, verifies, runs all tests, and reports results.

**Note:** You'll see "running 0 tests" for `src/lib.rs`—this is normal. Cargo looks for
unit tests inside `#[cfg(test)]` modules in `src/`, but all tests here are integration
tests in the `tests/` directory.

## License

MIT
