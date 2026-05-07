# CubeK Audio

**CubeK Audio: high-performance audio kernels in CubeCL.**

This workspace contains backend-agnostic audio kernels built with
[`cubecl`](https://github.com/tracel-ai/cubecl) and designed to compose with
the broader [`cubek`](https://github.com/tracel-ai/cubek) ecosystem.

## Algorithms

| Algorithm | Variants |
| --- | --- |
| [STFT](./crates/cubek-stft) | `stft` `istft` `hann-window` |
| [Phase vocoder](./crates/cubek-phasevocoder) | `phase-locked` `time-stretch` |
| [Resample](./crates/cubek-resample) | `rational-ratio` `sinc-fir` `fast-shifts` |
| [Sinc filter](./crates/cubek-sinc-filter) | `low-pass` `high-pass` `filter-bank` `fft-conv` |

## Running tests

Each crate is generic over `cubecl::Runtime`. Pick one backend feature for a
test run:

```sh
cargo test --workspace --no-default-features --features "std,test-metal"   # Apple Silicon
cargo test --workspace --no-default-features --features "std,test-cuda"    # Linux / NVIDIA
cargo test --workspace --no-default-features --features "std,test-vulkan"  # Linux / AMD / fallback
cargo test --workspace --no-default-features --features "std,test-cpu"     # portable CPU sanity run
```

Exactly one `test-*` backend feature should be enabled. The test crates include
backend guards so accidental zero-backend runs fail early.

For faster iteration on one crate, keep the same feature shape and add
`-p <crate>`:

```sh
cargo test -p cubek-stft --no-default-features --features "std,test-cpu"
```
