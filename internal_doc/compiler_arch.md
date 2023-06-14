# High Level Compiler arch ideas

A compiler can be seen as having two different goals.

1. Transform an input, through multiple stages, into a binary that can run
2. Validating the soundness of its inputs and reporting a useful message to the user if this fail.

Life as a compiler implementor is far easier if you only do 1, but it is not acceptable for us.

As such, not only is a stage of a compiler having to transform its input into a different output, this output need to retain enough information to walk back a validation error to its original input source. As such, a lot of metadate may be needed to be kept around.

As a general principle, every stage should be able to ouput its result to disk, so that the compiler can be restarted from previous runs but also so that it can be debugged.

## Pipeline

In current understanding, here is a draft pipeline

1. lexing
2. parsing tokens to AST.
3. Generation of module names table
4. DAG generation based on require, imports and use
5. macro expansion
6. Aliases expansions (may be as part of macros?)
7. Lowering into Core AST (pattern matching expansion in particular)
8. Type Inference/Type expansion into Typed Core AST
9. Type checking and Semantic Analysis
10. Pattern matching expansion based on type?
11. Optimisation first pass?
12. Backend IR generation (LLVM IR? Janet?)
13. Backend optimisation
14. Backend CodeGen

Note that we statically link everything. As such, while the dependency can be precompiled up to a point AOT (TODO: how much?), the latests steps can only be done once the whole program is assembled.

## Open questions

CST when?
Could we optimise the compiler so that it can do all the LSP stuff easily?
