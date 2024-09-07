#!/bin/bash
set -euo pipefail

# Build logic for all needed platforms
cargo build --release --target=x86_64-unknown-linux-gnu \
                      --target=aarch64-apple-ios \
                      --target=aarch64-apple-darwin \
                      --target=aarch64-linux-android

mkdir -p ../client-unreal/deusvent/ThirdParty # Ensure directory exists

cp ../target/x86_64-unknown-linux-gnu/release/liblogic.a ../client-unreal/deusvent/ThirdParty/liblogic.amd64.linux.a
cp ../target/aarch64-apple-ios/release/liblogic.a ../client-unreal/deusvent/ThirdParty/liblogic.arm64.ios.a
cp ../target/aarch64-apple-darwin/release/liblogic.a ../client-unreal/deusvent/ThirdParty/liblogic.arm64.darwin.a
cp ../target/aarch64-linux-android/release/liblogic.a ../client-unreal/deusvent/ThirdParty/liblogic.arm64.android.a

# Generate C++ wrapper
rm -rf ../client-unreal/deusvent/Source/deusvent/logic/*
uniffi-bindgen-cpp src/logic.udl --out-dir ../client-unreal/deusvent/Source/deusvent/logic

# Format using our style
(cd .. && clang-format --Werror -i -style=file client-unreal/deusvent/Source/deusvent/logic/logic.cpp)
