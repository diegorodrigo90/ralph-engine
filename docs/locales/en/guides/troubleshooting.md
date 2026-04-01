# Troubleshooting

## Toolchain drift

Run:

```bash
asdf install
./scripts/bootstrap-dev.sh
```

## Validation failures

Run:

```bash
./scripts/validate.sh --mode local
```

## SonarCloud fails with `Create analysis`

If the SonarCloud job fails during `Create analysis` with a `404` from `api.sonarcloud.io`, treat it as a token or permission issue first.

Checklist:

```text
- confirm the GitHub secret SONAR_TOKEN is present
- confirm the token still belongs to the expected SonarCloud account or organization
- confirm the token can browse the project
- confirm the token can execute analysis for the project
```

The CI workflow now runs a SonarCloud preflight before coverage and scan steps so this class of failure shows up earlier and with a clearer message.
