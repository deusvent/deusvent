# Build arm64 builds for both MacOS and iOS
cargo build --release --target=aarch64-apple-darwin \
                      --target=aarch64-apple-ios
cp ../target/aarch64-apple-darwin/release/liblogic.a ../client-unreal/deusvent/ThirdParty/liblogic.darwin.ios.a
cp ../target/aarch64-apple-ios/release/liblogic.a ../client-unreal/deusvent/ThirdParty/liblogic.arm64.ios.a

# Generate C++ wrapper
rm -rf ../client-unreal/deusvent/Source/deusvent/logic/*
uniffi-bindgen-cpp src/logic.udl --out-dir ../client-unreal/deusvent/Source/deusvent/logic

# HACK Fix for compiling error, see https://github.com/NordSecurity/uniffi-bindgen-cpp/pull/42
sed -i "" 's/streambuf(RustStreamBuffer(buf)), std::basic_iostream<char>(&streambuf)/std::basic_iostream<char>(\&streambuf), streambuf(RustStreamBuffer(buf))/' ../client-unreal/deusvent/Source/deusvent/logic/logic.hpp

# Format using our style
(cd .. && clang-format --Werror -i -style=file client-unreal/deusvent/Source/deusvent/logic/logic.cpp)
