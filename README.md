# Rust-argon2

Rust library for hashing passwords using
[Argon2](https://github.com/P-H-C/phc-winner-argon2), the password-hashing
function that won the
[Password Hashing Competition (PHC)](https://password-hashing.net).
Fork of sru-systems/rust-argon2 to make it accessible via WebAssembly.

## Examples

Create a password hash using the defaults and verify it:

```javascript
import {
  hash_encoded_js,
  create_default_config,
  verify_encoded_js,
} from "rust-argon2-wasm";

const res = JSON.parse(
  hash_encoded_js("password", "salt11bytes", create_default_config())
);

console.log(res.hash);

console.log(verify_encoded_js(res.hash, "password"));
```

## Requirements

- rust toolchain
- wasm32-unknown-unknown target
- wasm-pack

## Build Argon2 WASM Package

```sh
wasm-pack build
```

## Limitations

This crate has the same limitation as the `blake2-rfc` crate that it uses.
It does not attempt to clear potentially sensitive data from its work
memory. To do so correctly without a heavy performance penalty would
require help from the compiler. It's better to not attempt to do so than to
present a false assurance.

This version uses the standard implementation and does not yet implement
optimizations. Therefore, it is not the fastest implementation available.

## License

Rust-argon2 is dual licensed under the [MIT](LICENSE-MIT) and
[Apache 2.0](LICENSE-APACHE) licenses, the same licenses as the Rust compiler.

## Contributions

Contributions are welcome. By submitting a pull request you are agreeing to
make you work available under the license terms of the Rust-argon2 project.
