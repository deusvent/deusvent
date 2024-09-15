#!/bin/bash
set -euo pipefail

# Build first cdylib platform for current platform as we need cdylib file to generate bindings
cargo rustc --release --crate-type=cdylib

# Generate C++ wrapper
rm -rf ../client-unreal/deusvent/Source/deusvent/logic/*
uniffi-bindgen-cpp --library ../target/release/liblogic.dylib --out-dir ../client-unreal/deusvent/Source/deusvent/logic

# Format using our style
(cd .. && clang-format --Werror -i -style=file client-unreal/deusvent/Source/deusvent/logic/logic.cpp)

# Build logic for all needed platforms
cargo rustc --release --crate-type=staticlib --target=x86_64-unknown-linux-gnu
cargo rustc --release --crate-type=staticlib --target=aarch64-apple-ios
cargo rustc --release --crate-type=staticlib --target=aarch64-apple-darwin
cargo rustc --release --crate-type=staticlib --target=aarch64-linux-android

# Copy static libraries to client-unreal
mkdir -p ../client-unreal/deusvent/ThirdParty
cp ../target/x86_64-unknown-linux-gnu/release/liblogic.a ../client-unreal/deusvent/ThirdParty/liblogic.amd64.linux.a
cp ../target/aarch64-apple-ios/release/liblogic.a ../client-unreal/deusvent/ThirdParty/liblogic.arm64.ios.a
cp ../target/aarch64-apple-darwin/release/liblogic.a ../client-unreal/deusvent/ThirdParty/liblogic.arm64.darwin.a
cp ../target/aarch64-linux-android/release/liblogic.a ../client-unreal/deusvent/ThirdParty/liblogic.arm64.android.a




