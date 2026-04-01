#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import process from "node:process";

const ROOT_DIR = path.resolve(import.meta.dirname, "..");
const DEFAULT_OUT_DIR = path.join(ROOT_DIR, ".release", "npm");
const PACKAGE_SPECS = [
  {
    sourceDir: path.join(ROOT_DIR, "packaging", "npm"),
    outDirName: "ralph-engine",
    mutator: (pkg, version) => ({
      ...pkg,
      version,
      private: false,
    }),
  },
  {
    sourceDir: path.join(ROOT_DIR, "tools", "create-ralph-engine"),
    outDirName: "create-ralph-engine-plugin",
    mutator: (pkg, version) => ({
      ...pkg,
      version,
      private: false,
    }),
  },
];

function fail(message) {
  console.error(`prepare-npm-release: ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  const args = { outDir: DEFAULT_OUT_DIR };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    const next = argv[index + 1];

    switch (arg) {
      case "--version":
        if (!next) {
          fail("missing value for --version");
        }
        args.version = next;
        index += 1;
        break;
      case "--out-dir":
        if (!next) {
          fail("missing value for --out-dir");
        }
        args.outDir = path.resolve(next);
        index += 1;
        break;
      default:
        fail(`unknown argument: ${arg}`);
    }
  }

  if (!args.version) {
    fail("--version is required");
  }

  return args;
}

function copyDirectory(sourceDir, destinationDir) {
  fs.mkdirSync(destinationDir, { recursive: true });

  for (const entry of fs.readdirSync(sourceDir, { withFileTypes: true })) {
    if (entry.name === "node_modules") {
      continue;
    }

    const sourcePath = path.join(sourceDir, entry.name);
    const destinationPath = path.join(destinationDir, entry.name);

    if (entry.isDirectory()) {
      copyDirectory(sourcePath, destinationPath);
      continue;
    }

    fs.copyFileSync(sourcePath, destinationPath);
  }
}

function rewritePackageJson(packageDir, version, mutator) {
  const packageJsonPath = path.join(packageDir, "package.json");
  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
  const updatedPackageJson = mutator(packageJson, version);

  fs.writeFileSync(
    packageJsonPath,
    `${JSON.stringify(updatedPackageJson, null, 2)}\n`,
    "utf8",
  );
}

function main() {
  const args = parseArgs(process.argv.slice(2));

  fs.rmSync(args.outDir, { recursive: true, force: true });
  fs.mkdirSync(args.outDir, { recursive: true });

  for (const packageSpec of PACKAGE_SPECS) {
    const packageOutDir = path.join(args.outDir, packageSpec.outDirName);
    copyDirectory(packageSpec.sourceDir, packageOutDir);
    rewritePackageJson(packageOutDir, args.version, packageSpec.mutator);
    console.log(`Prepared ${packageSpec.outDirName} at ${packageOutDir}`);
  }
}

main();
