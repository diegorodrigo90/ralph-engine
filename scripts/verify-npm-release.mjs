#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { execFileSync } from "node:child_process";

const EXPECTED_PACKAGES = {
  "ralph-engine": {
    requiredEntries: ["bin/", "install.js", "README.md"],
    requiredBin: "ralph-engine",
    requiredScripts: ["postinstall"],
  },
  "create-ralph-engine-plugin": {
    requiredEntries: ["bin/", "lib/", "schema/", "README.md"],
    requiredBin: "create-ralph-engine-plugin",
    requiredScripts: [],
  },
};

function fail(message) {
  console.error(`verify-npm-release: ${message}`);
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

function readPackageJson(packageDir) {
  const packageJsonPath = path.join(packageDir, "package.json");
  if (!fs.existsSync(packageJsonPath)) {
    fail(`package.json was not found in ${packageDir}`);
  }

  return JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
}

function runDryRunPack(packageDir) {
  const stdout = execFileSync("npm", ["pack", "--json", "--dry-run"], {
    cwd: packageDir,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });

  let parsed;
  try {
    parsed = JSON.parse(stdout);
  } catch (error) {
    fail(`invalid npm pack JSON for ${packageDir}: ${error}`);
  }

  if (!Array.isArray(parsed) || parsed.length !== 1) {
    fail(`unexpected npm pack payload for ${packageDir}`);
  }

  return parsed[0];
}

function assertPackageContract(packageDir) {
  const packageJson = readPackageJson(packageDir);
  const expected = EXPECTED_PACKAGES[packageJson.name];
  if (!expected) {
    fail(`unsupported staged package: ${packageJson.name}`);
  }

  if (packageJson.private) {
    fail(`${packageJson.name} is still marked private in staged payload`);
  }

  if (typeof packageJson.version !== "string" || packageJson.version.length === 0) {
    fail(`${packageJson.name} is missing a staged version`);
  }

  if (!packageJson.bin || !packageJson.bin[expected.requiredBin]) {
    fail(`${packageJson.name} is missing bin entry ${expected.requiredBin}`);
  }

  for (const scriptName of expected.requiredScripts) {
    if (!packageJson.scripts || !packageJson.scripts[scriptName]) {
      fail(`${packageJson.name} is missing script ${scriptName}`);
    }
  }

  const packed = runDryRunPack(packageDir);

  if (packed.name !== packageJson.name) {
    fail(`npm pack produced unexpected name for ${packageDir}: ${packed.name}`);
  }

  if (packed.version !== packageJson.version) {
    fail(
      `npm pack produced unexpected version for ${packageDir}: ${packed.version} != ${packageJson.version}`,
    );
  }

  if (!Array.isArray(packed.files) || packed.files.length === 0) {
    fail(`npm pack reported no files for ${packageJson.name}`);
  }

  const packedPaths = new Set(packed.files.map((entry) => entry.path));
  for (const requiredEntry of expected.requiredEntries) {
    const exists = [...packedPaths].some(
      (entryPath) => entryPath === requiredEntry || entryPath.startsWith(requiredEntry),
    );
    if (!exists) {
      fail(`${packageJson.name} tarball is missing required entry ${requiredEntry}`);
    }
  }

  console.log(
    `Verified ${packageJson.name}@${packageJson.version} (${packed.files.length} packed entries)`,
  );
}

function main() {
  const args = parseArgs(process.argv.slice(2));

  for (const packageDir of args.packageDirs) {
    assertPackageContract(packageDir);
  }
}

main();
