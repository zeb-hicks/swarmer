#!/bin/bash

cargo build --target=x86_64-pc-windows-gnu --release
cargo build --target=x86_64-unknown-linux-gnu --release

rm -r target/publish/windows
rm -r target/publish/linux
rm target/publish/game.zip
rm target/publish/game.tar.gz

mkdir -p target/publish/windows

cp target/x86_64-pc-windows-gnu/release/game.exe target/publish/windows/
cp -r assets target/publish/windows/

cd target/publish/windows
zip -r ../game.zip .
cd -

mkdir -p target/publish/linux

cp target/x86_64-unknown-linux-gnu/release/game target/publish/linux/
cp -r assets target/publish/linux/

chmod +x target/publish/linux/game

cd target/publish/linux
tar -czf ../game.tar.gz .
cd -