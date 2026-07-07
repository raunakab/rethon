# rethon - Python Re-Imagined

[![CI](https://github.com/raunakab/rethon/actions/workflows/test-and-lint.yml/badge.svg)](https://github.com/raunakab/rethon/actions/workflows/test-and-lint.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

Rethon aims to be a widely-compatible, re-imagined successor to Python.
It derives inspiration from many languages, primarily Python and Rust.
At a high-level, the core features behind it are:

- a static, Turing-incomplete type system
- (aggressive) type-inference
- algebraic datatypes
- a concise, pythonic syntax
- first-class expressions
- advanced pattern matching
- a precedence on _pure, functional programming_ (with some leeway for mutability)
- hygienic macros
- arc-based memory management (no garbage collection)[^1]

Rethon comes shipped with a compiler + interpreter.
The compilation targets for Rethon include:

- machine code
- Rust
- JavaScript
- [BEAM](https://en.wikipedia.org/wiki/BEAM_(Erlang_virtual_machine))
- [JVM](https://en.wikipedia.org/wiki/Java_virtual_machine)
- [Wasm](https://en.wikipedia.org/wiki/WebAssembly)

## Philosophy

### Features + Syntax

The general, driving philosophy behind Rethon is to provide a blend between functionalism and pragmaticism.

Functional programming languages, in their full glory, come burdened with loads of mathematical jargon.
Academics tend to hijack such languages and convert them into unusable soup.

Nonfunctional programming languages, on the other hand, are clunky and error-prone.
They suffer from a different problem: the core of the language lacks so many basic features, engineers write roundabout code to achieve the same result.
(Take, for example, the lack of tagged-enums in most popular programming languages.
This has, time and time again, created trivial, avoidable bugs.
And even when "pragmatic" languages add these features, their implementations are oftentimes half-baked.)

If we can take the important things that functional programming languages preach, and merge them with the "go-get-'em" attitude that pragmatic programming languages champion, then I believe that we can achieve a happy-medium.

Case in point: think about Golang.
Everyone loves Golang because it is so simple and quick to write!
Seasoned veterans reach for it because of this very fact.
But despite its ease-of-use, it still lacks basic tagged-enums.
Imagine how much better Golang would be if it had them.

## Running

Another problem that Rethon aims to solve is "target availability".
There are many wonderfully designed and optimized virtual machines, such as the JVM, which perform some fancy, runtime introspection and optimization.
However, rarely do modern, "pragmatic" languages support these VMs as a target.
This is quite unfortunate because these VMs are actually quite powerful; in fact, many fintech shops *run their systems on the JVM!*

Secondly, I have been in many situations myself where I've wanted to quickly enter into a REPL in order to evaluate a quick expression or test out some code.
However, rarely do compiled languages come with a standardized REPL; in order to test out my code, I need to write some `main` boilerplate, compile my code, and then run the outputted binary.
This process is further complicated if I'm working with a language with no standardized build-system (C, for example).
Needless to say, this is an extremely poor developer experience.

My vision was to create a language in which the *compiled binary* and the *direct interpretation of the source-code* are exactly equivalent; any mode of execution leads to no difference at all (in direct results, side-effects, mutations, etc.).
(This, of course, does mean that certain dynamics "privileges" that interpreted languages enjoy needed to be scaled back.
But those "privileges" are, in my opinion, footguns anyways, so I was more than happy to axe them.
Take, for example, dynamic imports.)

## Examples

```rt
-- A simple `main` function.
-- We loop through a list of names and print them out.
main = fn ()
  names = ["McDavid", "Crosby", "MacKinnon", "Makar", "Marner"]
  for name in names:
    print(f"Hello, {name}!")
```

## Inspiration

As mentioned above, Rethon draws inspiration from many different languages.
The most prominent are:

- Rust
  - Algebraic datatypes
  - First class expressions
  - Pattern matching
  - Hygienic macros
- Python
  - Syntax
  - Generators
- Swift
  - Arc based memory-management
- Go
  - Brevity
  - Async generators

[^1]: Only for the machine code and Rust compilation targets. For the other compilation targets, that environment's memory management mechanism takes effect.
