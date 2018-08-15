#!/usr/bin/env bash

set -ex

: ${TARGET?"The TARGET environment variable must be set."}

# Tests are all super fast anyway, and they fault often enough on travis that
# having only one thread increases debuggability to be worth it.
export RUST_TEST_THREADS=1
export RUST_BACKTRACE=full
export RUST_TEST_NOCAPTURE=1
export RUSTFLAGS="${RUSTFLAGS} -C codegen-units=1"

export CARGO_INCREMENTAL=0
export CARGO_BUILD_JOBS=1

export CARGO_SUBCMD=test
if [[ "${NORUN}" == "1" ]]; then
    export CARGO_SUBCMD=build
fi

if [[ ${TARGET} == "x86_64-apple-ios" ]] || [[ ${TARGET} == "i386-apple-ios" ]]; then
    export RUSTFLAGS="${RUSTFLAGS} -Clink-arg=-mios-simulator-version-min=7.0"
    rustc ./ci/deploy_and_run_on_ios_simulator.rs -o $HOME/runtest
    export CARGO_TARGET_X86_64_APPLE_IOS_RUNNER=$HOME/runtest
    export CARGO_TARGET_I386_APPLE_IOS_RUNNER=$HOME/runtest
fi

# The source directory is read-only. Need to copy internal crates to the target
# directory for their Cargo.lock to be properly written.
mkdir target || true

rustc --version
cargo --version
echo "TARGET=${TARGET}"
echo "RUSTFLAGS=${RUSTFLAGS}"
echo "FEATURES=${FEATURES}"
echo "NORUN=${NORUN}"
echo "CARGO_SUBCMD=${CARGO_SUBCMD}"
echo "CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS}"
echo "CARGO_INCREMENTAL=${CARGO_INCREMENTAL}"
echo "RUST_TEST_THREADS=${RUST_TEST_THREADS}"
echo "RUST_BACKTRACE=${RUST_BACKTRACE}"
echo "RUST_TEST_NOCAPTURE=${RUST_TEST_NOCAPTURE}"
echo "CMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}"

cargo_test() {
    cmd="cargo ${CARGO_SUBCMD} -vv --target=${TARGET} ${@}"
    mkdir target || true
    ${cmd} 2>&1 | tee > target/output
    if [[ ${PIPESTATUS[0]} != 0 ]]; then
        cat target/output
        return 1
    fi
}

cargo_test
cargo_test --release
