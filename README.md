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
│   └── minmax.rs           # Verus-verified functions
├── tests/
│   └── test_minmax.rs      # Normal Rust runtime tests
├── target/verus/           # Compiled library for proof tests
│   ├── libverus_proof_time_testing.rlib
│   └── verus_proof_time_testing.vir
└── rust_verify_test/       # Proof test harness (nightly)
    ├── Cargo.toml
    ├── rust-toolchain.toml
    └── tests/
        ├── common/mod.rs   # Test harness
        └── prove_me.rs     # Proof tests
```

## From Checkout to Running Everything

### 1. Verify `src/` with Verus

```bash
cd ~/projects/verus-proof-time-testing
verus --crate-type=lib src/lib.rs
```

Expected output:
```
verification results:: 5 verified, 0 errors
```

### 2. Run Runtime Tests

These are normal Rust tests in `tests/` that exercise the library at runtime:

```bash
cargo test
```

Expected output:
```
running 7 tests
test test_max_x_basic ... ok
test test_min_x_basic ... ok
...
test result: ok. 7 passed
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
cargo test
```

Expected output:
```
running 10 tests
test test_use_library_max_x ... ok
test test_use_library_min_x ... ok
test test_use_library_lemmas ... ok
test test_wrong_assertion_fails ... ok
test one_proof_test_that_really_fails ... FAILED
...
test result: FAILED. 9 passed; 1 failed
```

These tests import the library directly:
```rust
use verus_proof_time_testing::minmax::*;

fn test_max_x() {
    let m = max_x(3, 5);
    assert(m == 5);
}
```

## Quick Reference

| Task | Command |
|------|---------|
| Verify library | `verus --crate-type=lib src/lib.rs` |
| Runtime tests | `cargo test` |
| Build for proof tests | See step 3 above |
| Proof tests | `cd rust_verify_test && cargo test` |

## Writing Tests

### Runtime Tests (`tests/`)

Standard Rust tests using the library:

```rust
use verus_proof_time_testing::minmax::*;

#[test]
fn test_max_x() {
    assert_eq!(max_x(3, 5), 5);
}
```

### Proof Tests (`rust_verify_test/tests/`)

Verification tests using `test_verify_one_file!`:

```rust
// Expected to verify successfully
test_verify_one_file! {
    #[test] test_name verus_code! {
        use vstd::prelude::*;
        use verus_proof_time_testing::minmax::*;

        fn test() {
            let m = max_x(3, 5);
            assert(m == spec_max_x(3, 5));
        }
    } => Ok(())
}

// Expected to fail verification (test PASSES when verification fails)
test_verify_one_file! {
    #[test] test_should_fail verus_code! {
        use vstd::prelude::*;

        fn broken() {
            assert(1 == 2); // FAILS
        }
    } => Err(err) => assert_one_fails(err)
}
```

### Understanding "Expected Failure" Tests

The `=> Err(err) => assert_one_fails(err)` pattern means "expect verification to fail".
When verification fails as expected, the **test passes** (shows "ok"):

```
test test_wrong_assertion_fails ... ok    ← Verification failed, as expected
test test_precondition_fails ... ok       ← Verification failed, as expected
```

A test only shows **FAILED** when reality doesn't match expectation:

| Expectation | Reality | Test Result |
|-------------|---------|-------------|
| `=> Ok(())` | Verification succeeds | ok |
| `=> Ok(())` | Verification fails | **FAILED** |
| `=> Err(err) => assert_one_fails(err)` | Verification fails | ok |
| `=> Err(err) => assert_one_fails(err)` | Verification succeeds | **FAILED** |

The test `one_proof_test_that_really_fails` demonstrates the last case — it expects
failure but verification succeeds, so the test itself fails.

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

## License

MIT
