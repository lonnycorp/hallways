# Agent Guidelines

## Philosophy

- Keep code minimal; do not add features, flags, or abstractions unless asked.
- Avoid defensive code and "just in case" checks.
- Let errors bubble up unless there is a clear reason to handle them.
- Treat tests as servants of the design: if tests block a clearer intended structure, rewrite the tests to validate the new shape instead of bending code to preserve old test seams.

## Layout

- Tests live in separate modules (`foo/test.rs`), never inline `#[cfg(test)]`.
- Within a file, put dependencies before dependents.
- Define types/errors/helpers before public functions that use them.
- Only extract helper methods when reused.

## Naming

- Unless a module is purely organizational, namespace public APIs by parent module (`foo/bar` -> `FooBar`).
- Use this threshold for argument grouping: if a function has `>= 3` non-`self` arguments, use one `Params`-suffixed struct; if it has `< 3`, use direct arguments.
- Avoid generic names like `State` for shared/public types.
- Prefer object-verb function names (`foo_get` over `get_foo`).
- For noun/adjective-only names, use broader-to-narrower order (`context_item`).
- In PascalCase, keep acronyms fully capitalized (`URLParse`, `GLTFMesh`).
- For concise read-only accessors, `_get` may be implied by context (`foo()` over `foo_get()`).
- For `Result` returns, define `{Type}{Method}Error`.

## Visibility

- Prefer information hiding at externally visible module boundaries.
- Default to private fields with read-only accessors.
- Use `pub` fields module-internal structs or transient data structs.

## Aesthetics

- Prefer explicit `return` where possible.
- If code has side effects or external mutation, use explicit loops/match blocks.
- Avoid side effects in expression-style assignments.
- Prefer named constants over inline magic numbers.
- Avoid no-op assignments, including redundant aliasing.
- No single-line `if` bodies; always use braces on the next line.

## Validation

- Run `cargo fmt`.
- Run `cargo clippy`; avoid unexpected warnings.
