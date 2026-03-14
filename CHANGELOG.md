# Changelog

All notable changes to this project are documented here.

---

## [v1.1.2] – CLI Polish & Developer Experience

### Added
- `help` command — displays full usage guide with commands, descriptions, and examples
- `version` command — prints current version from `Cargo.toml` at compile time
- Auto-help when running `rustsight` with no arguments

### Improved
- Unknown command error now echoes the invalid command back to the user
- All error messages now point to `rustsight help` instead of hardcoded command lists

---

## [v1.1.1] - Minor Changes before publishing

### Fixed
- Fixed minor changes before publishing.

## [v1.0.0] – Data Validation Release

### Added
- `validate` CLI command for dataset sanity checks
- Detection of:
  - High missing value ratios
  - No-variance numeric columns
  - Potential outliers
  - Mixed-type columns
- Clear warning-based validation output
- Release-ready `.exe` usage documentation

### Improved
- CSV analysis reporting
- Internal statistics aggregation
- CLI usability and messaging

---

## [v0.1.0] – Initial Release

### Added
- CSV dataset analyzer
- Column profiling (numeric vs categorical)
- Missing value detection
- Statistics (min, max, mean)
- File analyzer for text and binary files
- Report generation (`_report.txt`)
- Streaming support for large files