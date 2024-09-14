# logic

A core shared library used across all clients and servers. It contains shared structures and the core game logic.

- Main limitation is that we are using `uniffi` for generate C++ bindings which doesn't allow methods that takes `&mut self`. Either use interior mutability or consider writing a wrapper which will be `Send+Sync`
- We are targeting iOS, Android, Mac and Linux operating systems - ensure that all code and dependencies are compatible
- We need to support the `amd64` and `arm64` architectures, as well as WebAssembly - check carefully your code and dependencies
- Avoid using `usize` in public interfaces. While most targets are 64-bit, the WASM environment is still 32-bit. We will use binary encoding for data transfer/persistence and using `usize` can lead to hard-to-detect issues
- Currently, we use JSON for serialization because the AWS API WebSocket Gateway doesn’t support binary data. In the future, we will implement a workaround, so write all messages with that in mind
- The core logic should be free of any I/O operations, relying only on pure functions. I/O is highly platform-dependent and should be implemented elsewhere. Only exception are time and random related functionality
- The core library should not contain any logging. Logging should be handled by external code. If you need to provide additional information, consider returning types that describe the operation results instead of logging
- As a core game library, all public interfaces must be fully documented with clear comments
- While panics should generally be avoided, and `Result<T, E>` should be returned in most cases, using `::expect` is acceptable in scenarios where you are certain an error should never occur. It will occur eventually anyway, but hopefully we'll learn something from it
