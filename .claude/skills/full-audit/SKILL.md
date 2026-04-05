---
name: full-audit
description: Run a comprehensive 93-check audit of the Ralph Engine codebase covering security, architecture, plugins, tests, i18n, config, and code quality
---

# Full Audit — Ralph Engine

Run ALL checks below in order. Report findings as PASS/FAIL/WARN per category with file:line references.

## Phase 1: Automated (run these commands)

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo deny check
cargo doc --workspace --no-deps
npm run contracts:verify
node --test tools/create-ralph-engine/test/
scripts/check-i18n-compliance.sh
cd site && npm run build && cd ..
```

ALL must pass with zero errors. Any failure = CRITICAL finding.

## Phase 2: Semi-automated (grep + review)

Run each grep. For every match, evaluate if it's a real issue:

1. **Silent .ok() drops**: `grep -rn '\.ok()' --include="*.rs" core/ plugins/` — flag any in business logic (not test setup)
2. **Weak assertions**: `grep -rn 'assert!(.*is_ok())' --include="*.rs"` — each must check the value, not just Ok
3. **Weak error assertions**: `grep -rn 'assert!(.*is_err())' --include="*.rs"` — must check error code
4. **Model B violations**: `grep -rn 'official\.bmad\|official\.claude\|official\.codex' core/crates/` — zero hits outside tests/comments
5. **stdin in non-test code**: `grep -rn 'stdin().read_line\|read_line' --include="*.rs" core/ plugins/` — must have IsTerminal check
6. **println in TUI**: `grep -rn 'println!\|eprintln!' core/crates/re-tui/src/` — zero in non-test code
7. **Large files**: `find core/ plugins/ -name "*.rs" -exec wc -l {} + | sort -rn | head -20` — flag >500 lines
8. **Wildcard match arms**: `grep -rn '_ =>' --include="*.rs" core/ plugins/` — each needs comment explaining why
9. **coverage(off) justified**: `grep -rn 'coverage(off)' --include="*.rs"` — each must have (a) what, (b) why untestable, (c) how validated
10. **Locale key parity**: for each `plugins/official/*/locales/`, diff en.toml keys vs pt-br.toml keys

## Phase 3: Manual review (read code, verify behavior)

For EACH official plugin (`plugins/official/*/src/lib.rs`):

1. Does `PluginDescriptor` match `manifest.yaml`? (id, kind, capabilities, hooks)
2. Do tests verify **behavior** (specific values), not just existence?
3. Are error paths tested (at least one Err test per fallible method)?
4. Does the plugin respect Model B (no cross-plugin imports, no core internals)?

For core crates:

5. `re-plugin/src/lib.rs`: Only traits, types, constants — zero business logic
6. `re-config`: Config round-trip works (parse → render → parse = same)
7. `re-tui`: No business logic, pure rendering framework
8. `re-cli/src/commands/run.rs`: Uses auto-discovery, no hardcoded plugin refs
9. `re-official`: Build.rs auto-generates, no manual plugin wiring

For config templates:

10. `plugins/official/basic/template/config.yaml`: Parseable by core (has schema_version, plugins, mcp, budgets)
11. `plugins/official/bmad/template/config.yaml`: Same + run section
12. Template hooks.yaml: Documented as not-yet-implemented

## Output Format

```
## Audit Results — [date]

### Phase 1: Automated
- fmt: PASS
- clippy: PASS
- test: PASS (N tests)
- deny: PASS
- contracts: PASS
- i18n: PASS
- site: PASS

### Phase 2: Semi-automated
- .ok() drops: N found, M flagged
- Weak assertions: ...
- Model B: PASS/FAIL (list violations)
...

### Phase 3: Manual
- Plugin X: PASS/FAIL (findings)
...

### Summary
CRITICAL: N | HIGH: N | MEDIUM: N | LOW: N
```

Reference: Full 93-check checklist in CP repo at `_bmad-output/planning-artifacts/epics/ralph-engine/RE-FULL-AUDIT-CHECKLIST.md`
