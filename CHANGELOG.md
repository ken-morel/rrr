# Changelog

## v0.1.0

### Added
- **Core:** Initial functional release of the Remote REPL Runner.
- **Networking:** TCP socket implementation for remote server/client communication.
- **Security:** Passcode-based encryption for all queries and responses.
- **Streaming:** Real-time output streaming from the remote process to the client.
- **CLI:** Added `rrr server` command and interactive REPL mode.
- **CLI:** Built-in `ls` command and enhanced `--help` documentation.
- **OS:** Support for kill signals to manage remote process lifecycles.
- **Setup:** Created installation script and comprehensive README.

### Fixed
- Improved UTF-8 handling for shell prompts to prevent character corruption.
- Resolved execution logic errors when running commands inside the REPL.
- Switched to environment-variable based configuration for better container/scripting support.

### Changed
- Migrated from Linux-only sockets to standard TCP sockets.
- Removed `toml` dependency to reduce binary size and compile times.
