# Tree-Sitter-Clia

Clia for now use TreeSitter as its parser, this is an ongoing implementation

The binding have not been committed, due to the partial WIP status.

It is recommended that you have a read at [the Tree-sitter documentation](http://tree-sitter.github.io/tree-sitter/creating-parsers).

## Hacking

1. You need Tree-sitter 0.20.8, the nix shell should have provided it
2. run `tree-sitter generate`
3. run `tree-sitter test`
4. Everything Should be green. If yes proceed to do changes, add tests and go back to step 2
