# Clia
A glue language that has your back.

Clia is in a really early design phase. Right now, you will only see design references and research.

## Design Decision

Clia is an all expressions language that is heavily elixir inspired. Statically typed in the future, with type inference.
Clia is an effect handler language, taking much inspiration from Effekt.
Clia uses Capabilities to make running a script easier. The design for these is still open but will probably be based on Effect Handlers.
Clia will support Polymorphism for effects and types.
Clia will target, for now, an interpreted model using the Janet interpreter as its backend. Long term, we plan to target the LLVM in a more AOT-compiled situation.
Clia tooling and compiler are written in Rust.

### Open Questions

- How to handle capabilities
- What design for polymorphism, Traits, Modules, a mix ?
- backends?
- hygienic quotation-based macros or not?
- structural or nominative typing?
- Subtyping?
- Set-Theoretic types?

## Design 

## Design Philosophy

We want to make writing glue scripts easier and less error-prone. Running them should be easy to do and safe, and it should be easy to discover what the use of these scripts is.

### Features

- Ergonomic error message
- Embedded tooling for project management, formatter, LSP, test framework, and package management
- Package management needs to work in drop-in scripts.
- Compilation has to be fast and snappy. We will benchmark it from the start and set a target we refuse to breach early on.
- We must be portable on at least Linux, Windows, MacOS, and FreeBSD.
- Pattern matching
- Safe arithmetic

### Anti features

- No mutability
- Precise binary layout
- No garbage collection
- No implicit type conversion
- No support for concurrency

## Licence

Clia is under the Apache 2.0 version.
