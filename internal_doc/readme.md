# Cia, the ClI tools Language

We've tossed around some ideas for a scripting langauge that combines many of
the features we want:

- Static binary compilation
- Cross platforms binary
- Easy shell interop
- Easy rust interop
- Pattern matching
- CLI tools like arg-parsing built in and easy to use. ("Like an included
  `clap`")
- Typed (Crochet level?)

For more complete analysis [we have a whole rationale in a google
doc](https://docs.google.com/document/d/1vI4UgBH7CXGvSFOYNchrusikFqN46-sp6Ph7tVZN5K0/edit?usp=sharing)

## Background

Often projects needs to build some ability to group together actions. This could
be automating the step to do a build, a deploy. This could be the need to dump
a databse, run migrations, connect to a remote service. Historically, this kind
of ad-hoc local tool end up as script, in makefile or justfile, or is handled
in the command runner of the language.

Sometimes, they grow up to the point they are a mini CLI tool, with options,
input, output, arguments and different commands.

How can we make it easy to build these ad-hoc tools, while making them easy to
discover and learn how they work. While making it also safe to use. Basically,
how much time is spent a new hire the precise arcane incantation to use your
project specific scripts ? Could we make that part of the tools and easy to
discover for them, so they can do it at their speed?

## Existing solutions

This is a review of what is currently used to solve this problem

### Shell scripts

In a lot of case, this kind of tool start their life as a shell script.
This provide repeatability and does not need additional work to "just run"

Pros:

- repeatable
- Available
- reflect the way the tool was explored
- Easy to call out other cli tools
- Portable-ish

Cons:

- Hard to document
- Hard to manpage
- Hard to build a structure and type coerced CLI arguments and flags
- Not that portable, quirks between shells, etc
- Dependencies on cli tool needing to be installed with no way to define them
- Hard to test
- Quirky, footgunny language

### Scripting language (Ruby, Python, ...)

A natural evolution is to move the script from shell to a scripting language

Pros:

- Access to libraries and package ecosystem
- Ability to test
- Better expressivity
- Tooling to help manage risks
- Portable-ish
- Build tools to define dependencies
- Relatively easy to onboard people on

Cons:

- Ok to document
- Ok/hard to manpage
- Limited support for framework for the CLI arguments and flags
- Installing the dependencies can be really hard and painful.
- Slightly harder to shell out

### Task Runners (MakeFiles, JustFiles, bundle, pip, etc)

Pros:

- Discoverability of commands
- help in building the interface (kinda)
- Can run anything
- portable-ish

Cons:

- Usually the same as whatever is used under the hood to run the task

### Golang

Why not make it a small tool in golang!

Pros:

- Complete language
- Packaging
- Single portable-ish binary
- Some documentation
- Tests
- Meh to onboard people on

Cons:

- Golang
- limited support for cli arguments and flags frameworks.
- Hard to shell out

### Rust

A lot of CLI tools are built in Rust, let's do the same!

Pros:

- Clap. Hand down the best
- Complete language
- Packaging
- Single binary that can be cross compiled easily
- Tests
- Documentation
- Manpage (Clap <3)

Cons:

- Compilation time
- Working with string
- The job is too high level for what Rust bring usually, in particular
- Detailed memory handling, type and all.
- can be hard to onboard people on

### Nim

Pros:

- Simple to compile
- packaging
- Single binary that can be cross compiled easily
- Tests
- Documentation
- Python like syntax, so relatively easy to onboard people on
- Good C FFI

Cons:

- C toolchain, but eh
- No framework for cli to the clap level
- Syntax, but eh

### Janet

Janet is a nice tiny little tool that seems made for this!

Pros:

- Simple to compile
- packaging
- Single binary that can be cross compiled easily
- Tests
- Documentation
- PEG parser
- Good C FFI

Cons:

- Lisp, ie harder to onboard people to.
- No cli framework

### Haskell, Scala-native

See Rust, but with on top needing to learn Haskell/Scala

## Requirements

### Goals

- Syntax: python/ruby/Elixir. Classic enough to onboard people quickly
- Need to have an easily accessible (STDLIB) for CLI framework
- Same for http request, json, shelling out to tools, handling files system interactions, ...
- Pattern matching with guards
- Some parser combinator or PEG in the stdlib
- Binary pattern matching? or parser at least
- List, arrays, hashmaps, binaries, string (unicode aware), Path, etc
- Portable on mac, linux, windows, Freebsd, openbsd at least
- easy toolchain
- FFI to Rust easily, embedded in the language at least, possibly C
- Fast compilation (concurrent all the things)
- Cross compilation
- "with" ala python for file opening/closing (see effects)
- Easy documentation. Markdown or Djot
- Libraries handling well the portability of system interactions
- Type System
- One file program possible and easy
- function pattern matching everywhere (ala erlang/elixir)
- Code formatter
- LSP integration (probably easier if the compiler steps make sense)

### Nice to have

- Package management
  - package repo
  - Semver library and Semver constraints library
  - package resolution (what was that new paper already)
- Test framework that fit the same file
- One single portable binary instead of one binary per target (Cosmopolitan ?)
- pipe operator
- Macro, hygienic, for extensibility
- Ability to turn into a full fledge cli tools, modules, big projects. So that
  you can start small and grow into what is needed without having to use
  Rust/Haskell/Go/Python when it become too big
- Compiler error messages helpful
- Capabilities (see Crochet or Deno)
  - Note that we are targetting an _interactive_ environment, so we can ask for capabilities on the fly
  - Would allow to replace curl | sh by ... a "safe" installer script.
  - Effects as capabilities ?
    - If we can make them ergonomics, effects handlers would really be lovely anyway, so why not...
    - automatically inject flags for cli that handle capabilities passing or a config file with capabilities
- Traits/multi dispatch on traits. We can "relatively" easily dispatch on type, let's try to dispatch on traits too?
  - Not a lot of good solution for statically typed multi dispatch
  - Julia does it well but ... dynamic
  - maybe we just reduce our type safety for it, as long as we have capabilities through effects handler to limit risks ...
  - or we just statically link and check everything, so yolo.
- Separate parsed data from unsafe external data (this is an effect too...)
- Locale independant programming? (see hedy?)
- Functors?
- something that can rewrite code to help upgrade or linters aka macros on first pass AST?
- GC or lexical lifetime....
- Templating engine

### Non goals

- concurrency and parallelism
- Dynamically linking libraries (exception for C ABI)
- fixed size integers, everything is big int
- Objects
- Separating headers from modules body
- GUI
- Dependent types
- Compete with C/Rust/Cpp on speed. Target is bash or python speed level

## Backend

We've considered compiling to Janet, as it has many of the features we want -
static compilation, small binary size, trivial to embed into other programs via
C FFI.

Another solution is to ship the LLVM, compile to Rust, compile to C, etc
Compile to Zig?
Use the Zig toolchain ? Urgh but...

Compile to Ocaml/a ppx? Possibly a good intermediary solution.

Not a lot of good options...

LLVM IR probably...

### Use Case long term

"Replace `curl | sh` scripts/installer with our own?
Basically we would be a "script" or binary runner, but we all the safety we
offer(use case for capabilities)

Replace all the classic `scripts` or `bin` folders
Rewrite something like the AWS cli
