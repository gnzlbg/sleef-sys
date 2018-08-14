#!/bin/sh

set -ex

rm -rf target/doc
mkdir -p target/doc

dox() {
    local arch=$1
    local target=$2
    local feature=$3

    echo "documenting $arch + $feature"

    rustup target add $target || true

    local dir=target/doc/$arch
    if [ "$feature" != "" ]; then
        dir=target/doc/$arch_$feature
    fi

    rm -rf $dir
    mkdir $dir

    local rustflags=""
    if [ "$feature" != "" ]; then
        rustflags="-C target-feature=+${feature}"
    fi

    RUSTFLAGS=$rustflags

    cargo build --verbose --target $target

    rustdoc --verbose --target $target \
            -o $dir src/lib.rs \
            --crate-name sleef-sys \
            --library-path target/$target/debug/deps \
            --extern cfg_if=`ls target/$target/debug/deps/libcfg_if-*.rlib` \
             --extern libc=`ls target/$target/debug/deps/liblibc-*.rlib`
}

dox x86_64 x86_64-unknown-linux-gnu sse2
dox x86_64 x86_64-unknown-linux-gnu avx
dox aarch64 aarch64-unknown-linux-gnu neon
dox powerpc64le powerpc64le-unknown-linux-gnu vsx

# If we're on travis, not a PR, and on the right branch, publish!
if [ "$TRAVIS_PULL_REQUEST" = "false" ] && [ "$TRAVIS_BRANCH" = "master" ]; then
  pip install ghp_import --install-option="--prefix=$HOME/.local"
  $HOME/.local/bin/ghp-import -n target/doc
  git push -qf https://${GH_PAGES}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
fi
