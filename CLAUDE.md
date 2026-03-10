# CLAUDE.md — AI Assistant Guide for libuniverse

## Project Overview

**libuniverse** (published on crates.io as `runiverse`) is a pure, safe Rust library for astronomical calculations. It implements algorithms from Jean Meeus' book *"Astronomical Algorithms"* and serves as a Rust-native equivalent to the C library [libnova](http://libnova.sourceforge.net/).

- **Language:** Rust (Edition 2021)
- **Crate type:** dylib + staticlib
- **License:** MIT
- **Author:** Mattia Procopio
- **Repository:** https://github.com/MattBlack85/libuniverse/

---

## Repository Structure

```
libuniverse/
├── .cargo/
│   └── config.toml          # CPU feature flags (FMA, AVX2 for x86_64)
├── .github/
│   └── workflows/
│       ├── ci.yml           # Build + format check (Ubuntu, macOS, Windows)
│       └── bench.yml        # Nightly benchmark runner
├── benches/                 # Criterion benchmark binaries
│   ├── dynamical_time.rs
│   ├── julian_day.rs
│   ├── sidereal_time.rs
│   └── transform.rs
├── src/
│   ├── lib.rs               # Crate root: public API, core types, exports
│   ├── date.rs              # Calendar date representation and operations
│   ├── julian_day.rs        # Julian Day Number calculations
│   ├── dynamical_time.rs    # Delta-T (ΔT) dynamical time corrections
│   ├── sidereal_time.rs     # Mean sidereal time calculations
│   └── transform.rs         # Coordinate system transformations
├── Cargo.toml               # Package manifest and dependencies
├── README.md                # Brief project description
└── code_of_conduct.md       # Community guidelines
```

---

## Module Responsibilities

| Module | Purpose |
|---|---|
| `lib.rs` | Public API surface: `HoursMinSec`, `DegMinSec`, `RightAscension`, `Declination`, `EqPosition`, `LongLatPosition`, `fit_degrees()` |
| `date.rs` | `Date` struct with `to_julian_day()`, `week_day()`, `year_day()`, `interval()` |
| `julian_day.rs` | `JulianDay` struct, `get_julian_day()`, `to_calendar_date()`, `to_modified_jd()` |
| `transform.rs` | `ra_to_deg()`, `deg_to_ra()`, `dec_to_deg()`, `deg_to_dms()` — all `#[must_use]` |
| `dynamical_time.rs` | `delta_t()` — polynomial ΔT corrections for years −1999 to +3000 |
| `sidereal_time.rs` | `get_mean_sidereal_time_from_date()` with optional FMA optimisation |

---

## Core Data Types

```rust
// Integer time/angle components
struct HoursMinSec { hours: i16, minutes: u8, seconds: f64 }
type RightAscension = HoursMinSec;

struct DegMinSec { sign: i8, degrees: u16, minutes: u8, seconds: f64 }
type Declination = DegMinSec;

// Composite position types
struct EqPosition     { ra: RightAscension, dec: Declination }
struct LongLatPosition { longitude: f64, latitude: f64 }

// Date and Julian Day
struct Date { year: i16, month: u8, day: f64, hours: u8, minutes: u8, seconds: f64 }
struct JulianDay(f64);
```

---

## Development Commands

### Build

```bash
cargo build               # Debug build
cargo build --release     # Optimised release build
```

### Test

```bash
cargo test --lib          # Run all unit tests (31 tests across 6 modules)
cargo test --lib -- --nocapture  # With stdout output
```

### Lint and Format

```bash
cargo fmt                 # Auto-format code
cargo fmt --all -- --check  # Check formatting (used in CI)
cargo clippy              # Lint checks
```

### Benchmarks

```bash
cargo bench               # Run all Criterion benchmarks (requires nightly for some features)
cargo +nightly bench      # Explicitly use nightly (as CI does)
cargo bench --bench julian_day  # Run a specific benchmark
```

### Documentation

```bash
cargo doc --open          # Generate and open rustdoc locally
```

---

## Dependencies

| Crate | Version | Use |
|---|---|---|
| `libmath` | 0.2 | Rounding utilities for coordinate arithmetic |
| `regex` | 1.6 | Parsing coordinate strings (e.g. `"12h 30m 45s"`) |
| `criterion` | 0.3 | Benchmarking framework (dev dependency) |

---

## Code Conventions

### Rust Idioms to Follow

- **Edition 2021** — use modern Rust idioms.
- Mark pure transformation functions with `#[must_use]` (see `transform.rs`).
- Use factory methods (`from_degrees()`, `from_string()`, `from_full_date()`) over direct struct construction in public APIs.
- Implement `Display`, `PartialEq`, `Eq`, `Debug`, and `PartialOrd` for public types where appropriate.
- Use type aliases (`type RightAscension = HoursMinSec`) to add semantic meaning without new types.
- Conditional CPU optimisation via `#[cfg(target_feature = "fma")]` — keep hardware-specific paths gated.

### Error Handling

- The library currently **panics** on invalid input strings (e.g. malformed coordinate strings). The `DRAGONS AHEAD!` comment in `transform.rs` flags this explicitly.
- New parsing code should prefer returning `Result` over panicking where feasible.
- Internal arithmetic does not use `Result`; domain errors (e.g. out-of-range dates) are caller responsibility.

### Testing

- Every module has a `#[cfg(test)] mod test { }` block at the bottom.
- Test values are taken directly from worked examples in *Meeus, Astronomical Algorithms* — always cite the chapter/example number in the test comment.
- Use exact floating-point comparisons only when the algorithm is deterministic to the last bit; otherwise compare to a suitable tolerance (e.g. `(result - expected).abs() < 1e-6`).
- Do **not** leave tests commented out without a `// TODO:` explanation.

### Benchmarks

- Each benchmark lives in its own file under `benches/`.
- Use `criterion::black_box` to prevent dead-code elimination.
- Benchmarks run with `harness = false` in `Cargo.toml`.

### Performance Considerations

- `.cargo/config.toml` enables `target-feature=+fma,+avx2` for `x86_64` targets — assume these are available in release builds on that architecture.
- Release profile sets `codegen-units = 1` for maximum inlining.
- Algorithms are ported from Meeus' polynomial approximations — preserve the polynomial form for readability and to simplify future corrections.

---

## CI/CD

### `ci.yml` (triggered on PRs and pushes to `main`)

- **Matrix:** `ubuntu-latest`, `macos-latest`, `windows-latest`
- Steps: checkout → cache cargo → `cargo build --verbose` → `cargo fmt --all -- --check`
- A failed format check blocks merge.

### `bench.yml` (triggered on PRs and pushes to `main`)

- **Runner:** `ubuntu-latest` with Rust **nightly**
- Steps: checkout → switch to nightly → `cargo +nightly bench > benches/output.txt`
- Benchmark output is written to `benches/output.txt` (committed if changed).

---

## Astronomical Domain Notes

These notes help AI assistants reason about correctness:

- **Julian Day (JD):** A continuous day count from noon 1 January 4713 BC. JD 2451545.0 = J2000.0 (1 January 2000, 12:00 TT).
- **Modified Julian Day (MJD):** MJD = JD − 2400000.5.
- **Delta-T (ΔT):** Difference between Terrestrial Time (TT) and Universal Time (UT1). Varies historically; `dynamical_time.rs` uses polynomial fits per time interval.
- **Mean Sidereal Time:** Earth's rotation angle relative to distant stars; used to convert between equatorial and horizontal coordinates.
- **Right Ascension (RA):** Measured in hours (0–24 h), minutes, seconds eastward along the celestial equator.
- **Declination (Dec):** Measured in degrees (−90° to +90°), minutes, seconds north/south of the celestial equator.
- **Coordinate transforms in `transform.rs`:** Convert between decimal degrees and sexagesimal (DMS/HMS) representations. All functions are pure and `#[must_use]`.

When modifying or adding algorithms, always cite the relevant Meeus chapter and equation number in a doc comment.

---

## Adding New Features

1. **New astronomical calculation** → add a new module under `src/`, expose public functions via `src/lib.rs`.
2. **New data type** → implement `Display`, `PartialEq`, `Debug` at minimum; add `#[must_use]` to constructors returning `Self`.
3. **New benchmark** → add a file under `benches/` and register it in `Cargo.toml` with `harness = false`.
4. **All new code** must have accompanying unit tests with values from Meeus or another authoritative source.
5. Run `cargo fmt` and `cargo clippy` before committing — CI will block unformatted code.
