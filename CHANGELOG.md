# Changelog

All notable changes to this project will be documented in this file.

## [v0.2.0] - 2026-03-02

### Added
- Deterministic emoji shuffle based on remote URL seed for consistent results per repo
- Random ord probing for visual diversity in multi-emoji branch names
- `seed_from_remote_url` and `next_available_ord` utilities

### Fixed
- Lifetime warning in `map_emoji`
- Clippy warnings: `explicit_auto_deref` and `map_flatten`

## [v0.1.6] - 2024-06-20

### Added
- GitHub release configuration
- Cross-compilation support via Cross.toml

### Changed
- Updated Clap to v4.5.7
- Bumped crate version

## [v0.1.5] - 2024-06-18

### Added
- Show all branch names created by this tool via `-a` flag

### Changed
- Updated README

## [v0.1.4] - 2024-01-15

### Added
- MIT LICENSE

### Changed
- Version bump to 0.1.4

## [v0.1.3] - 2023-12-20

### Added
- Default remote selection when only one remote exists

## [v0.1.2] - 2023-12-15

### Added
- Branch creation support via `-b` flag
- Log level control via `-v` flag
- README documentation

## [v0.1.1] - 2023-12-10

### Added
- Shuffle feature for emoji randomization
- AnyBase module for bijective base-k numeration
- Cargo metadata

## [v0.1.0] - 2023-12-05

### Added
- Initial release
- Emoji-based random git branch name generation
- Remote branch detection
- Basic CLI interface
