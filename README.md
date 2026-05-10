# Rust Verifier Benchmark

A benchmark to compare Rust verifiers. This repository is currently under construction.

## Examples

The following table shows all examples and whether they were implemented successfully using the given verifiers.
If an entry is not following the legend, an explanation can be found in the linked example file.

**Legend:**

| Entry | Explanation                                                                                    |
|-------|------------------------------------------------------------------------------------------------|   
| ✅     | Implemented successfully. The implementation can be found in the subdirectory of the verifier. |
| N/A   | The example is not applicable to the verifier.                                                 |
| TODO  | Not implemented yet.                                                                           |

| Name                           | Aeneas | Creusot | Flux | Kani | Prusti | Verus | VeriFast |
|--------------------------------|--------|---------|------|------|--------|-------|----------|
| [octuple](examples/octuple.md) | ✅      | ✅       | ✅    | ✅    | TODO   | ✅     | N/A      |

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
