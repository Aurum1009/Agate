# Master plan for Agate

Creating infrastructure:

1. Define the argument parsing [completed]
2. Define the scanner infrastructure(tokens, token type enum) [completed]
3. Create the scanner structure (completed)
   1. Enhance the scanner with support for unicode names
4. Create the compiler's `Repr`
5. Define the roots of the compiler (pratt parsing)
6. Create the compiler structure
7. Create the validator
8. Create the type checker
9. Create the null checker
10. Create the optimizer
11. create the `analyze()` function (analyzes + optimizes `Repr`)
12. Create the lowering function from `Repr` into AghAST
13. Create WASM codegen with cranelift
14. Define the instruction set & other AVIAN utilities
15. Create AVIAN assembly codegen (in-house)
16. Create AVIAN bytecode codegen (in-house)
17. Create VM utilities
    1. Make IO thread
    2. Make FS thread
    3. Define `CallFrame`
18. Create the `test_wasm()` function and implement the WASM test runtime
19. Create the `.agbc` disassembler
20. Start making the `.avian` disassembler
21. Create the basic VM structure
22. Hook up the `run_avian()` and `run_agbc` functions
23. Improve the VM's functionality range
24. Make sure all command line paths function correctly

Compilation targets:

- WebAssembly, via Cranelift and Wasmtime.
- AVIAN Agate's IR text/assembly format.
- AgBC, AVIAN's binary format for its IR.

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
