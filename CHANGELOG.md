# Changelog

All notable changes to this project are documented here.

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