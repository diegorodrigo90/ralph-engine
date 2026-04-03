---
title: "Troubleshooting"
description: "Common issues and how to fix them"
---

## Toolchain Drift

If Rust or Node versions are out of sync with the repository pins, re-sync with asdf:

```bash
asdf install
```

Then re-run the bootstrap script:

```bash
./scripts/bootstrap-dev.sh
```

## Validation Failures

Run the full local validation contract to identify what is broken:

```bash
./scripts/validate.sh --mode local
```

## SonarCloud Fails with `Create analysis`

If the SonarCloud job fails during `Create analysis` with a `404` from `api.sonarcloud.io`, treat it as a token or permission issue first.

### Checklist

- Confirm the GitHub secret `SONAR_TOKEN` is present.
- Confirm the token still belongs to the expected SonarCloud account or organization.
- Confirm the token can browse the project.
- Confirm the token can execute analysis for the project.

### How CI Handles SonarCloud

The CI workflow runs a SonarCloud preflight before coverage and scan steps so this class of failure shows up earlier and with a clearer message.

The workflow resolves the project key and organization from `sonar-project.properties` before the scan and passes them explicitly to the scanner. This keeps the scan input visible in the job log and removes ambiguity about which SonarCloud project the workflow is targeting.

### Coverage Artifact Flow

Coverage is generated once in `Quality (ubuntu-latest)`, uploaded as a short-lived artifact, and then reused by the SonarCloud job. If SonarCloud reports a missing coverage file, inspect the earlier Ubuntu quality job before debugging the scan step itself.

### Coverage Quality Gate

If SonarCloud fails because the quality gate coverage is below `100%`, treat that as a real release blocker rather than a documentation mismatch. The repository contract is that reusable release artifacts are approved only after the configured SonarCloud gate passes in full.
