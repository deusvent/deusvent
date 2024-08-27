# deusvent

The unique game designed to guide you toward your lifetime goals.

- `api` - WebSockets based API for the clients. Rust, DynamoDB, AWS Lambdas
- `client-unreal` - Unreal Engine 5 C++ client with support of MacOS and iOS
- `design` - brading, assets, fonts, colors, marketing materials
- `infa` - Terraform files for AWS: API Gateway, Lambdas, S3, DynamoDB, Route53, etc.
- `logic` - Separate library for the core game logic written in Rust
- `logic-binding-cpp` - C++ wrapper for the `logic`. Used by `client-unreal`
- `logic-binding-wasm` - WASM wrapper for the `logic`. Will be used for the upcoming `client-web`
- `story` - Scrivener project for the story: characters, world, quests, progression, etc.
- `www` - Zola based static site for [www.deusvent.com](https://www.deusvent.com)
