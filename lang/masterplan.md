# Master plan for Agate

Creating infrastructure:

1. Define the scanner
2. Define the aghast standard & enum + structures
3. Define the roots of the compiler
4. Lowering + VM Nonsense (Re-evaluate what to do then)

Compilation targets:

- WebAssembly, via Cranelift and Wasmtime.
- AVIAN Agate's IR text/assembly format.
- AgBC, Agate's binary format for its IR.

Two of these targets, AVIAN and AgBC, are handled in the same loop, while lowering to WASM requires type information, and is therefore handles in AgAIN(the lower IR)

The entire lowering process looks like:

- Source
- High AST, aka aghast.
- Low AST, just called AST or LAST.
  - If targeting WebAssembly, compile into it.
- IR, aka AVIAN => into AgBC(default) or text format

Each module takes:

- utils: none
- agate: program args
- compiler: (source: String) -> ProjectAST
- runtime: (code: AVIANProject)
- codegen_wasm: (ast: ProjectAST)
- codegen_avian: (ast: ProjectAST)
