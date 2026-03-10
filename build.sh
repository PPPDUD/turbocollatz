#!/usr/bin/bash
# Create a Debian package from the turbocollatz source code using fpm and ronn-ng.
ronn man/*.ronn    # create the manpages
gzip -9 -f man/*.? # gzip the manpages
mkdir -p dist
cargo build --release
mv target/x86_64-unknown-linux-musl/release/turbocollatz dist/turbocollatz
cargo build --release --features u64-turbocollatz
mv target/x86_64-unknown-linux-musl/release/turbocollatz dist/turbocollatz-u64
rm -f dist/turbocollatz.deb
fpm \
	-s dir -t deb \
	-p dist/turbocollatz.deb \
	--name turbocollatz \
	--license MIT \
	--version 0.3.0 \
	--architecture amd64 \
	--description "A package for verifying the Collatz conjecture." \
	--url "https://github.com/PPPDUD/turbocollatz" \
	--maintainer "PPPDUD <mojavesoft@gmail.com>" \
	dist/=/usr/bin/ \
	man/turbocollatz.1.gz=/usr/share/man/man1/turbocollatz.1.gz
