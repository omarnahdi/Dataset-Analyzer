# Changelog

All notable changes to this project are documented here.

---

## [v1.2.0] – Report Formatting, Execution Timing & Command Renames

### ⚠️ Breaking Changes
- `csv` command renamed to `stats` — update any scripts or workflows using `rustsight csv`
- `analyze` command renamed to `inspect` — update any scripts or workflows using `rustsight analyze`

### Added
- Execution timer for all commands — displays milliseconds for fast runs, seconds for slower ones

### Fixed
- Report generation column widths increased from 7 to 14 characters — large numbers like `14,118,918` and `1,885,922.46` no longer break table alignment
- Report header row now uses the same dynamic widths as data rows for consistent formatting
- Separator line extended to match new total table width

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

## [v1.1.1] – Minor Changes Before Publishing

### Fixed
- Fixed minor changes before publishing

---

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