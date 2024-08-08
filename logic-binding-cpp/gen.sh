cargo build --release
rm -rf ../client-unreal/deusvent/Source/deusvent/logic/*
uniffi-bindgen-cpp src/logic.udl --out-dir ../client-unreal/deusvent/Source/deusvent/logic
cp ../target/release/liblogic.a ../client-unreal/deusvent/ThirdParty/liblogic.a

# HACK Fix for compiling error, see https://github.com/NordSecurity/uniffi-bindgen-cpp/pull/42
sed -i "" 's/streambuf(RustStreamBuffer(buf)), std::basic_iostream<char>(&streambuf)/std::basic_iostream<char>(\&streambuf), streambuf(RustStreamBuffer(buf))/' ../client-unreal/deusvent/Source/deusvent/logic/logic.hpp
