#!/usr/bin/env node

import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import process from "node:process";
import { execFileSync } from "node:child_process";

const COMMAND_TIMEOUT_MS = 180_000;

const PACKAGE_BIN_ASSERTIONS = {
  "ralph-engine": {
    binName: "ralph-engine",
    args: ["--version"],
    assertOutput: (stdout, packageJson) => stdout.includes(packageJson.version),
  },
  "create-ralph-engine-plugin": {
    binName: "create-ralph-engine-plugin",
    args: ["--help"],
    assertOutput: (stdout) => stdout.includes("create-ralph-engine-plugin"),
  },
};

function fail(message) {
  console.error(`verify-npm-install: ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  const args = { packageDirs: [] };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    const next = argv[index + 1];

    switch (arg) {
      case "--package-dir":
        if (!next) {
          fail("missing value for --package-dir");
        }
        args.packageDirs.push(path.resolve(next));
        index += 1;
        break;
      default:
        fail(`unknown argument: ${arg}`);
    }
  }

  if (args.packageDirs.length === 0) {
    fail("at least one --package-dir is required");
  }

  return args;
}

function run(command, args, options = {}) {
  try {
    return execFileSync(command, args, {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
      timeout: COMMAND_TIMEOUT_MS,
      ...options,
    });
  } catch (error) {
    if (error?.code === "ETIMEDOUT") {
      fail(
        `command timed out after ${COMMAND_TIMEOUT_MS}ms: ${command} ${args.join(" ")}`,
      );
    }

    const stderr = typeof error?.stderr === "string" ? error.stderr.trim() : "";
    const stdout = typeof error?.stdout === "string" ? error.stdout.trim() : "";
    const diagnostics = [stderr, stdout].filter(Boolean).join("\n");
    fail(
      diagnostics
        ? `command failed: ${command} ${args.join(" ")}\n${diagnostics}`
        : `command failed: ${command} ${args.join(" ")}`,
    );
  }
}

function readPackageJson(packageDir) {
  const packageJsonPath = path.join(packageDir, "package.json");
  if (!fs.existsSync(packageJsonPath)) {
    fail(`package.json was not found in ${packageDir}`);
  }

  return JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
}

function createTempDir(prefix) {
  return fs.mkdtempSync(path.join(os.tmpdir(), `ralph-engine-${prefix}-`));
}

function packPackage(packageDir, outputDir) {
  const stdout = run(
    "npm",
    ["pack", "--json", "--pack-destination", outputDir],
    { cwd: packageDir },
  );

  let parsed;
  try {
    parsed = JSON.parse(stdout);
  } catch (error) {
    fail(`invalid npm pack JSON for ${packageDir}: ${error}`);
  }

  if (!Array.isArray(parsed) || parsed.length !== 1 || !parsed[0].filename) {
    fail(`unexpected npm pack payload for ${packageDir}`);
  }

  const tarballPath = path.join(outputDir, parsed[0].filename);
  if (!fs.existsSync(tarballPath)) {
    fail(`npm pack did not produce ${tarballPath}`);
  }

  return tarballPath;
}

function writeConsumerPackageJson(consumerDir) {
  fs.writeFileSync(
    path.join(consumerDir, "package.json"),
    `${JSON.stringify({ name: "ralph-engine-install-smoke", private: true }, null, 2)}\n`,
    "utf8",
  );
}

function binPath(consumerDir, binName) {
  const suffix = process.platform === "win32" ? ".cmd" : "";
  return path.join(consumerDir, "node_modules", ".bin", `${binName}${suffix}`);
}

function installTarball(consumerDir, tarballPath) {
  console.log(`Installing staged tarball ${path.basename(tarballPath)}...`);
  run(
    "npm",
    ["install", "--no-package-lock", "--fund=false", "--audit=false", tarballPath],
    { cwd: consumerDir },
  );
}

function verifyInstalledCommand(packageJson, consumerDir) {
  const assertion = PACKAGE_BIN_ASSERTIONS[packageJson.name];
  if (!assertion) {
    fail(`unsupported staged package: ${packageJson.name}`);
  }

  const executable = binPath(consumerDir, assertion.binName);
  if (!fs.existsSync(executable)) {
    fail(`installed executable was not found: ${executable}`);
  }

  const stdout = run(executable, assertion.args, { cwd: consumerDir }).trim();
  if (!assertion.assertOutput(stdout, packageJson)) {
    fail(
      `${packageJson.name} executable output did not match expectations: ${stdout}`,
    );
  }

  console.log(`Verified install usability for ${packageJson.name}@${packageJson.version}`);
}

function assertPackageInstall(packageDir) {
  const packageJson = readPackageJson(packageDir);
  const packDir = createTempDir("pack");
  const consumerDir = createTempDir("consumer");

  try {
    console.log(`Packing and installing ${packageJson.name}@${packageJson.version}...`);
    const tarballPath = packPackage(packageDir, packDir);
    writeConsumerPackageJson(consumerDir);
    installTarball(consumerDir, tarballPath);
    verifyInstalledCommand(packageJson, consumerDir);
  } finally {
    fs.rmSync(packDir, { recursive: true, force: true });
    fs.rmSync(consumerDir, { recursive: true, force: true });
  }
}

function main() {
  const args = parseArgs(process.argv.slice(2));

  for (const packageDir of args.packageDirs) {
    assertPackageInstall(packageDir);
  }
}

main();
