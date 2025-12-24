# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.0] - 2025-12-23

### Added
- **TCX Format Support (Bidirectional)**
  - TCX → PWF import conversion
  - PWF → TCX export conversion
  - Full CLI integration: `pwf convert --from tcx --to pwf` and `pwf convert --from pwf --to tcx`
  - Comprehensive TCX parsing (activities, laps, trackpoints)
  - GPS route extraction with elevation data
  - Heart rate, cadence, and power telemetry import/export
  - TrainingPeaks extensions for power/cadence in TCX export
  - Multi-lap activity support
- **FIT Export Documentation**
  - Comprehensive `FIT_EXPORT_ANALYSIS.md` documenting why FIT export is not feasible
  - Evaluation of all available Rust FIT libraries
  - Recommended alternatives (TCX export + Garmin FitCSVTool)
  - Helpful CLI error messages for unsupported `--from pwf --to fit`
- **Test Coverage**
  - 159 new tests (729 total, up from 570)
  - 93.84% code coverage (exceeds 90% requirement)
  - 23 TCX-specific CLI tests
  - 13 TCX exporter unit tests
  - 10 TCX conversion integration tests

### Changed
- Improved CLI error handling for format conversion edge cases
- Enhanced verbose mode to show conversion statistics and warnings
- Updated README with TCX conversion examples and documentation

### Fixed
- None

## [1.1.0] - 2025-12-22

### Added
- **FIT Format Import Support**
  - FIT → PWF conversion (read-only)
  - CLI integration: `pwf convert --from fit --to pwf`
  - Multi-sport activities (triathlon/duathlon) support
  - GPS routes with full telemetry extraction
  - Swimming data (pool config, stroke types, SWOLF calculation)
  - Power metrics (NP, TSS, IF, VI, FTP)
  - Device information extraction
  - Physiological metrics (Training Effect, recovery time)
  - Advanced telemetry (heart rate, power, cadence, temperature)
- **pwf-converters Crate**
  - New library crate for format conversions
  - Comprehensive error handling with warnings
  - Conversion result tracking and validation
- **Test Coverage Improvements**
  - Boosted coverage from 85.68% to 94.70%
  - 63 FIT parser unit tests
  - 33 CLI convert command tests
  - Test helpers for synthetic FIT data generation

### Changed
- Updated workspace to include pwf-converters crate
- Enhanced CLI with `convert` subcommand
- Improved documentation with converter usage examples

### Fixed
- None

## [1.0.1] - 2025-12-21

### Added
- **PWF v2.1 Advanced Features**
  - Time-series telemetry data support
  - GPS route tracking with position history
  - Multi-sport activity segments
  - Pool swimming length tracking
  - Progressive overload strategies
  - Workout templates and reusable exercises
  - Exercise library and endurance workouts
  - Plan lifecycle tracking (activated_at, completed_at, archived_at)
  - Glossary support for custom terminology
  - RIR (Reps in Reserve) and rep-specific PRs
  - Percentage-based loading with reference exercises
  - Grouping (supersets, circuits) and rest period tracking

### Changed
- Expanded PWF type system to v2.1
- Enhanced validation for new features
- Updated documentation for all new fields

### Fixed
- None

## [1.0.0] - 2025-12-20

### Added
- Initial release of PWF (Portable Workout Format)
- **pwf-core Library**
  - Plan parsing and validation
  - History parsing and validation
  - Comprehensive error reporting with error codes
  - Support for strength, countdown, stopwatch, and interval modalities
- **pwf-cli Binary**
  - `validate` command for plan validation
  - `history` command for history validation
  - `init` command for template generation
  - JSON, compact, and pretty output formats
  - Strict mode for treating warnings as errors
- **Documentation**
  - Complete specification (docs/SPECIFICATION.md)
  - Modality reference (docs/modalities.md)
  - Equipment reference (docs/equipment.md)
  - Block-level documentation
  - Example files and usage guides
- **JSON Schema**
  - PWF v1 plan schema
  - PWF v1 history schema
  - Schema validation integration

### Changed
- N/A (initial release)

### Fixed
- N/A (initial release)

---

## Release Links

- [1.2.0](https://github.com/bcarlson/pwf/releases/tag/v1.2.0) - TCX Format Conversion
- [1.1.0](https://github.com/bcarlson/pwf/releases/tag/v1.1.0) - FIT Format Import & Coverage Improvements
- [1.0.1](https://github.com/bcarlson/pwf/releases/tag/v1.0.1) - PWF v2.1 Advanced Features
- [1.0.0](https://github.com/bcarlson/pwf/releases/tag/v1.0.0) - Initial Release

## Legend

- **Added**: New features
- **Changed**: Changes to existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security improvements
