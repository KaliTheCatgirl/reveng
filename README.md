# reveng
A small reverse engineering framework built for decoding file formats.

## Instructions
reveng's main features are the `Readable` and `FromBytes` traits. A struct that implements `Readable` can be read from anything that implements `Read`, and a struct that implements `FromBytes` can be created from a fixed-size byte array. Both traits are `derive`able, where each member is decoded sequentially. Anything that implements `FromBytes` will automatically have a `Readable` implementation.

## Existing Implementations
All numeric types implement `FromBytes`, including floating-point types.

The `readables` module contains some common non-trivial types, such as padding and specialised strings. Note that `String`s do *not* implement `Readable` or `FromBytes`. Instead, the set of string types in the `readables::strings` module should be used appropriately.

## Plans

- [x] Derive macros
- [ ] Support for arrays
- [ ] Support for externally-supported items (Vec or String with length specified out-of-file)
- [ ] Support for enums
- [ ] `Write` capability
- [ ] Visualisation
