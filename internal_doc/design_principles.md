# Design

This is a description of the design principles that have been used to design
Clia.

The high level goals are:

1. Everything that can be done in a scripting language should be possible in
   Clia, though Clia may rely on external commands to do so
2. Clia should be user-friendly, but not at the expense of expressiveness, Most
   tradeoffs between power and ease of use can be avoided with careful design.
3. Consent of the owner of the system Clia run on is paramount. Programs written
   in Clia are guests on these systems. As such we should respect the wants of
   the end-user and help them understand how they can use programs written in
   Clia but also what these programs will do that may necessitate informed
   consent. Respecting these rules should be easy for the person writing a Clia
   program, to follow the above goals.

To achieve these goals, Clia design relies on a number of more specific design
choices. These are presented below, with a rationale for them and sometimes
examples.

## The law of responsiveness

Writing a clia program should be as slick as possible. As such, every action
that the Clia toolkit need to make during development and release should be fast
and snappy enough, so that the question of performance is rarely raised.

Examples:

- Compilation into a release is incremental and try to reuse information from
  editing
- Running tests should use all cores
- Features like syntax highlighting or autosuggestions have been integrated from
  day 1

## The law of user problems

We do not design features for Clia. We find problems that are widespread in the
field, and then we find ways to solve them. If we cannot find a way to do so
without impossible to surpass implementation problems, it means we cannot solve
the problem for now and we need to keep searching.

Examples:

- We do not support Dependency Injection and all its pain, instead we waited
  until Effects Handlers research had been good enough to offer a good interface
  for this kind of behaviour polymorphism at the call site.

## The law of discoverability

Features of programs written in Clia should be easy to discover for the end
user, and it should be safe to explore the feature to find how it works.

Rationale: Clia programs are rarely interacted with. A program whose feature are
discoverable turns new users into expert in a short time, since the user become
an expert by simply using it, even if wrongly. Discovery is regularly done
through trial and error, so failing safely is a must.

Exemples:

- Clia ship with integrated LSP
- Clia scripts produce shell completion and automatic description and help
  message, both for `-h` and `--help`
- Every syntax error and error in a built-in command should contain an error
  message describing what went wrong, a documentation page and possibly provide
  possible steps to recovery
- The help manual should be easy to read, contain example, be accessible from
  the shell, and be easy to hack on
- Capabilities limit the blast radius of wrong command and allow logging, to
  show the user what has been done

## The law of the ecosystem

Interfacing with other languages and the environement need to be simple but
safe. We do not control the ecosystem we interact with, but we should be good
citizens and good neighbours and collaborators.

Rationale: Clia programs need to interface with other tools regularly, as they
are not designed in a vaccuum, so we must make interoperation as easy as
possible. This also allows for incremental adoption, which make it easier for
Clia benefits to spread with limited buy-in.

Examples:

- Tagged templates allow for safe interpolation of text input for other languages
- Any input from the outside have to be parsed into an internal structure, but
  the stdlib ship with all the tools needed.
- Possibly adding FFI to other languages, like Rust or C for extensions, and
  integrating them in the package management
