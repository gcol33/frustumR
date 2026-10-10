#!/bin/sh

# Write src/rust/vendor.tar.xz with the sources of every crate in Cargo.lock.
# When the archive is present, configure unpacks it and cargo builds offline.

set -e
cd "`dirname "$0"`/../src/rust"
rm -rf vendor vendor.tar.xz
cargo vendor --locked --versioned-dirs vendor > /dev/null
tar -cJf vendor.tar.xz vendor
rm -rf vendor
