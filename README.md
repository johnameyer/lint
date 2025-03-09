# lint

## Goals

To produce formatters and linters for all languages distributed and usable in those lanuage's package manager.

I.e. Java should be available in Maven for consumption and usable through Ant / Gradle, typescript should be available on the npm repo and usable through `npx`

## Layers

* Lossless syntax parser
  * The goal is to reuse existing parsing configurations (tree-sitter) and avoid using syntax specific to our implementation language (Rust macros)
* Engine
* Rules
* Language / build tool bindings

## Development Plan

* Produce a basic formatter for Java
  * Parse files lossily and re-emit them out formatted
* Produce a formatter for Typescript
* Produce a linter for Java
* Expand linter to Typescript
* Author rules
* Heuristical type data
* Type data
* Dataflow analysis / ?

## Inspiration

* [OpenRewrite](https://docs.openrewrite.org/)
* [Angular Schematics](https://angular.dev/tools/cli/schematics)
* [ESLint: Deprecation of formatting rules](https://eslint.org/blog/2023/10/deprecating-formatting-rules/)

## Notes

We have picked tree-sitter as an initial parser due to its wide support of languages and usage of a generic tree rather than creating Rust nodes. We also think the incremental parsing aspect is useful for our re-writing. It is strange how the parsers themselves are written in javascript however. Parsing out existing whitespace might also be slightly difficult. We might re-evaluate in the future and ideally isolating this layer might also allow for other implementations.


Alternatives considered:
* lalrpop - includes Rust syntax in grammar definition file so hard to maintain
* pest - requires manual definition of enums (https://pest.rs/book/examples/jlang.html#parsing-and-ast-generation)
* rowan - poor documentation and relies on Rust both for definition and output
