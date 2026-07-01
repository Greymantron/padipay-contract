# Contributing to PadiPay Contracts

Thank you for contributing to the PadiPay Soroban contracts. 

## Rust Coding Standards

- **Use Soroban SDK Types**: Always use `soroban_sdk` native types (`Address`, `Symbol`, `Vec`, `Map`, `String`) instead of standard Rust collections to ensure compatibility with the Soroban environment.
- **Error Handling**: Use explicit `Error` enums for contract failures instead of panicking.
- **Format and Lint**: Ensure your code passes `cargo fmt` and `cargo clippy`.

## Testing Requirements

- Every public contract function MUST have corresponding unit tests in `tests/test.rs`.
- Use the `Env` mock environment provided by the `soroban-sdk` `testutils` feature to simulate blockchain interactions.
- Test both successful paths and edge cases (e.g., unauthorized access, invalid state transitions).

## Working with TODOs

We use `TODO:` comments heavily to outline the blueprint. When resolving a TODO, please remove the comment and implement the requested logic clearly.
