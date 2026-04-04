# Audit Ralph Engine Code Quality

Run a comprehensive audit of the Ralph Engine codebase, checking adherence to architecture principles and Golden Rules.

## Checks

### 1. Modularity & Auto-Discovery
- Verify ALL plugin interactions use `collect_*_from_plugins()` pattern
- Verify core crates never import plugin crates directly
- Check that new PluginRuntime trait methods have default implementations
- Verify capabilities are registered in `ALL_PLUGIN_CAPABILITIES`

### 2. i18n Compliance (Rules 56-57)
Run `scripts/check-i18n-compliance.sh` and report results.
- Zero `locale_str!` in command files
- Zero `is_pt_br()` in command files
- Zero `locale == "pt-br"` raw comparisons
- Zero hardcoded plugin IDs in core CLI source

### 3. Test Quality (Rules 51-55)
- Scan for `assert!(result.is_ok())` without value check — flag as weak
- Scan for `assert!(result.is_err())` without error code check — flag as weak
- Scan for `assert!(!output.is_empty())` without content check — flag as weak
- Verify no `stdin().read_line()` in test code
- Check coverage(off) annotations have explanatory comments

### 4. TUI Code (Rule 58)
- Scan re-tui for `println!` or `eprintln!` outside test modules
- Verify tracing is used for all logging

### 5. Plugin Architecture
- Each plugin manifest.yaml has: id, kind, capabilities, lifecycle, runtime_hooks
- Plugin capabilities match their PluginRuntime trait implementations
- No business logic in re-plugin (only traits and types)

### 6. Coding Principles
- Functions > 30 lines — flag for review
- Files > 500 lines — flag for potential splitting
- Unused imports or dead code
- Missing doc comments on public items

## Output

For each category, report:
- PASS: how many checks passed
- FAIL: specific violations with file:line references
- WARN: potential issues that deserve manual review

Run `cargo clippy`, `cargo fmt --check`, and `cargo test` as part of the audit.
