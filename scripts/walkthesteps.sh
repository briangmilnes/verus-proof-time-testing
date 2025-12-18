#!/bin/bash
# Walk all steps from clean to verification and testing
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
VERUS="${VERUS:-$HOME/projects/verus-lang/source/target-verus/release/verus}"

cd "$PROJECT_DIR"

echo "════════════════════════════════════════════════════════════════"
echo "Step 0: Clean"
echo "════════════════════════════════════════════════════════════════"
cargo clean
(cd rust_verify_test && cargo clean)
echo "✓ Cleaned"

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "Step 1: Verify src/ with Verus"
echo "════════════════════════════════════════════════════════════════"
$VERUS --crate-type=lib src/lib.rs
echo "✓ Verification passed"

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "Step 2: Run Runtime Tests"
echo "════════════════════════════════════════════════════════════════"
cargo test
echo "✓ Runtime tests passed"

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "Step 3: Build Library for Proof Tests"
echo "════════════════════════════════════════════════════════════════"
mkdir -p target/verus
$VERUS --compile --crate-type=lib --crate-name=verus_proof_time_testing \
    -o target/verus/libverus_proof_time_testing.rlib \
    --export target/verus/verus_proof_time_testing.vir \
    src/lib.rs
echo "✓ Library built"

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "Step 4: Run Proof Tests"
echo "════════════════════════════════════════════════════════════════"
cd rust_verify_test

echo ""
echo "── prove_set_x ──"
cargo test --test prove_set_x

echo ""
echo "── prove_me (includes 1 intentional failure) ──"
# prove_me has one intentional failure, so we expect exit code 101
set +e
cargo test --test prove_me
PROVE_ME_EXIT=$?
set -e

if [ $PROVE_ME_EXIT -eq 101 ]; then
    echo "✓ prove_me: 9 passed, 1 intentionally failed (as expected)"
elif [ $PROVE_ME_EXIT -eq 0 ]; then
    echo "⚠ prove_me: All passed (expected 1 intentional failure)"
else
    echo "✗ prove_me: Unexpected exit code $PROVE_ME_EXIT"
    exit 1
fi

cd "$PROJECT_DIR"

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "Step 5: Run Benchmarks"
echo "════════════════════════════════════════════════════════════════"
cargo bench
echo "✓ Benchmarks complete"

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "All steps completed successfully!"
echo "════════════════════════════════════════════════════════════════"

