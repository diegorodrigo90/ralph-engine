# Testing Rules

Rules follow EARS syntax (SHALL keyword).
Applies to: `core/**/*.rs`, `plugins/**/*.rs`

## Test Structure

- Tests SHALL follow Arrange, Act, Assert (AAA) pattern.
- Test names SHALL describe the behavior being tested, not the function name.
- Each acceptance criterion SHALL have at least one test.
- Error paths SHALL verify error code or message content, not just `is_err()`.

## Mock Boundary

- Mock-free testing SHALL be the default.
- Real files in temp directories SHALL be used for file I/O tests.
- Real input strings SHALL be used for parser tests.
- Mocking is ONLY acceptable at true I/O boundaries (network, subprocess).

## Test Fixtures (Model B)

- Shared contract tests SHALL use neutral synthetic fixtures.
- Test fixtures SHALL NOT reference official plugin identifiers (`official.*`).
- Use generic IDs: `"test-a"`, `"ind-a"`, `"tool-reader"`, etc.
- Official plugins verify their own manifests in their crate tests.

## Coverage

- Code SHALL target 100% meaningful coverage.
- Genuinely untestable code (subprocess spawn, binary deps) SHALL use
  `#[cfg_attr(coverage_nightly, coverage(off))]` with justification.
- Pure logic SHALL ALWAYS be extracted into testable functions first.

## TDD

- Tests SHALL be written before implementation (RED → GREEN → REFACTOR).
- Bug fixes SHALL include a regression test that fails without the fix.
