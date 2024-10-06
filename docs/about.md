# The Agate Infrastructure Project

This project aims to empower anyone on any platform to program in Agate, a generally safe programming language.
Some of Agate's strengths includes things like:

- Null safety (See `systems/nullability-in-ag.md`)
- Type safety (See `systems/agate-type_sys.md`)
- Memory safety (See `systems/memory-management.md` and `built-in-rust.md`)
- Concurrency inspired by Erlang & Rust (See `concurrency/threading.md`, `concurrency/processes.md`, and `rts/concurrency.md`)
- Simple syntax, taking the best from languages from Rust and C# and combining them with Python.
- A performant, portable runtime and bytecode/assembly format
- WASM compilation for the Web
- A simple backend plugin capability, for other backends to be added!

A lot about Agate is taking what _works_ from other languages and taking out what doesn't. Key examples include Erlang's dynamic typing and Rust's ownership & lifetime (nothing against them) systems.

Please see some of the examples in `./examples.md`. If you are a developer that wants to compile into Serin, please see the Serin project or visit `/ag_rs/serin/`
