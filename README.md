# deusvent

The unique game designed to guide you toward your lifetime goals.

- `api` - WebSockets based API for the clients. Rust, DynamoDB, AWS Lambdas
- `client-unreal` - Unreal Engine 5 C++ client with support of MacOS and iOS
- `design` - branding, assets, fonts, colors, marketing materials
- `infra` - Terraform files for AWS: API Gateway, Lambdas, S3, DynamoDB, Route53, etc.
- `logic` - Separate library for the core game logic written in Rust with C++ and WASM binding
- `www` - Zola based static site for [www.deusvent.com](https://www.deusvent.com)
