# Rust Verifier Benchmark

A benchmark to compare Rust verifiers. This repository is currently under construction.

## Verifiers

| Verifier             | Version                                                                                                 | Links                                                                                                                          |
|----------------------|---------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------|
| [Aeneas](Aeneas)     | [a2fcf19](https://github.com/AeneasVerif/aeneas/tree/a2fcf1923d16684cec638f5503da11acd0ef002d)          | [GitHub](https://github.com/AeneasVerif/aeneas)                                                                                |
| [Creusot](Creusot)   | [0.11.0](https://github.com/creusot-rs/creusot/releases/tag/v0.11.0)                                    | [Website](https://creusot.rs/) \| [GitHub](https://github.com/creusot-rs/creusot)                                              |
| [Flux](Flux)         | [009f708](https://github.com/flux-rs/flux/tree/009f708f63649f2b0af5cc3f9e6792384cf8eed6)                | [Docs](https://flux-rs.github.io/flux/) \| [GitHub](https://github.com/flux-rs/flux)                                           |
| [Kani](Kani)         | [0.67.0](https://github.com/model-checking/kani/releases/tag/kani-0.67.0)                               | [Docs](https://model-checking.github.io/kani/getting-started.html) \| [GitHub](https://github.com/model-checking/kani)         |
| [VeriFast](VeriFast) | [26.01](https://github.com/verifast/verifast/releases/tag/26.01)                                        | [Docs](https://verifast.github.io/verifast/rust-reference/introduction.html) \| [GitHub](https://github.com/verifast/verifast) |
| [Verus](Verus)       | [0.2026.05.31.5dd6d83](https://github.com/verus-lang/verus/releases/tag/release%2F0.2026.05.31.5dd6d83) | [Docs](https://verus-lang.github.io/verus/guide/) \| [GitHub](https://github.com/verus-lang/verus)                             |

## Examples

The following table shows all examples and whether they were implemented successfully using the given verifiers.
If an entry is not following the legend, an explanation can be found in the linked example file.

**Legend:**

| Entry | Explanation                                                                                    |
|-------|------------------------------------------------------------------------------------------------|   
| ✅     | Implemented successfully. The implementation can be found in the subdirectory of the verifier. |
| ⏺️    | Can be implemented with significant changes to the example.                                    |
| ❌     | Cannot be implemented. See example file.                                                       |
| N/A   | The example is not applicable to the verifier.                                                 |
|       | Not implemented yet.                                                                           |
| !P    | Cannot prove absence of panics.                                                                |
| !T    | Cannot prove termination.                                                                      |

| Name                                                                 | Main Features                       | Aeneas | Creusot | Flux   | Kani | VeriFast | Verus   |
|----------------------------------------------------------------------|-------------------------------------|--------|---------|--------|------|----------|---------|
| [ackermann](examples/ackermann.md)                                   | Difficult Recursion                 |        | ❌       |        |      |          | ❌       |
| [assert](examples/assert.md)                                         | Unreachable Panic                   | ✅      | ✅       | ✅      | ✅    |          | ❌       |
| [binary_search](examples/binary_search.md)                           | Loops, Slices, Termination          |        |         | ❌      |      |          | ✅, !P   |
| [concat_slices_to_vec_clone](examples/concat_slices_to_vec_clone.md) | Loops, Slices, Vec, Clone           |        | ✅, !T   | ⏺️, !T |      |          | ✅       |
| [concat_slices_to_vec_copy](examples/concat_slices_to_vec_copy.md)   | Loops, Slices, Vec                  |        | ✅, !T   | ⏺️, !T |      |          | ✅       |
| [concat_slices_to_vec_std](examples/concat_slices_to_vec_std.md)     | Slices, Vec, Standard Library       |        | ❌       | ⏺️, !T |      |          | (✅)     |
| [div_by_sub_remainder_rec](examples/div_by_sub_remainder_rec.md)     | Tuples, Non-Linear Arithmetic       |        | ✅       | ⏺️, !T |      |          | ✅       |
| [even_odd_mutual_rec](examples/even_odd_mutual_rec.md)               | Mutual Recursion                    | ✅      | ✅, !T   | ✅, !T  |      |          | ✅       |
| [gnome_sort](examples/gnome_sort.md)                                 | Loops, Mutable Slices, Termination  |        | ✅, !T   | ❌      |      |          | ⏺️, !T  |
| [man_or_boy_test](examples/man_or_boy_test.md)                       | Closures, `dyn` Traits, Termination |        | ❌       | ❌      | ✅    |          | ❌       |
| [octuple](examples/octuple.md)                                       | Arithmetic Overflow                 | ✅      | ✅       | ✅      | ✅    | N/A      | ✅       |
| [range](examples/range.md)                                           | Struct Fields, Struct Invariant     | ✅      | ✅       | ✅      | ✅    |          | ✅       |
| [ternary_logic](examples/ternary_logic.md)                           | `std::ops` Traits                   | ✅      | ✅       | ✅      | ✅    |          | (✅), !P |
| [triangle_rec](examples/triangle_rec.md)                             | Recursion, Non-Linear Arithmetic    | ✅      | ✅       | ✅, !T  | (✅)  |          | ✅       |

## Usage

### Docker

You can build a Docker container with all verifiers by running the following command.
(Note that this may take about 30 minutes depending on your connection and hardware and takes about 50GB of free space.)

```bash
.dev/build.sh
```

Then you can start the container with

```bash
.dev/run-interactive.sh
```

Inside the container you can run all verifiers and verify all examples by running

```bash
.dev/verify-all.sh
```
