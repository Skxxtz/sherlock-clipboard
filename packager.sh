#!/bin/bash

read -p "Current version: " version
rm -rf ~/.tmp/sherlock-clipboard-release/
mkdir -p ~/.tmp/sherlock-clipboard-release/
cargo build --release
cp target/release/confetti ~/.tmp/sherlock-clipboard-release/
cp LICENSE ~/.tmp/confetti-release/LICENSE

cd ~/.tmp/confetti-release/
tar -czf confetti-v${version}-bin-linux-x86_64.tar.gz sherlock-clp LICENSE
