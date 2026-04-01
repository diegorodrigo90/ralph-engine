#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import process from "node:process";

const ROOT_DIR = path.resolve(import.meta.dirname, "..");
const TEMPLATE_PATH = path.join(
  ROOT_DIR,
  "packaging",
  "homebrew",
  "ralph-engine.rb.tmpl",
);
const REQUIRED_TARGETS = [
  "x86_64-unknown-linux-gnu",
  "x86_64-apple-darwin",
  "aarch64-apple-darwin",
];

function fail(message) {
  console.error(`render-homebrew-formula: ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  const args = {};

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
      case "--dist-dir":
        if (!next) {
          fail("missing value for --dist-dir");
        }
        args.distDir = path.resolve(next);
        index += 1;
        break;
      case "--output":
        if (!next) {
          fail("missing value for --output");
        }
        args.output = path.resolve(next);
        index += 1;
        break;
      default:
        fail(`unknown argument: ${arg}`);
    }
  }

  if (!args.version || !args.distDir || !args.output) {
    fail("--version, --dist-dir, and --output are required");
  }

  return args;
}

function checksumVariableName(target) {
  return `sha256_${target.replaceAll("-", "_")}`;
}

function readChecksum(distDir, target) {
  const checksumPath = path.join(
    distDir,
    `re-cli-${target}.tar.xz.sha256`,
  );

  if (!fs.existsSync(checksumPath)) {
    fail(`missing checksum asset for target '${target}' at ${checksumPath}`);
  }

  const contents = fs.readFileSync(checksumPath, "utf8").trim();
  const [checksum] = contents.split(/\s+/);

  if (!checksum || !/^[a-f0-9]{64}$/i.test(checksum)) {
    fail(`invalid checksum payload in ${checksumPath}`);
  }

  return checksum;
}

function main() {
  const args = parseArgs(process.argv.slice(2));
  let rendered = fs.readFileSync(TEMPLATE_PATH, "utf8");

  rendered = rendered.replaceAll("{{ version }}", args.version);

  for (const target of REQUIRED_TARGETS) {
    rendered = rendered.replaceAll(
      `{{ ${checksumVariableName(target)} }}`,
      readChecksum(args.distDir, target),
    );
  }

  fs.mkdirSync(path.dirname(args.output), { recursive: true });
  fs.writeFileSync(args.output, rendered, "utf8");
  console.log(`Rendered Homebrew formula at ${args.output}`);
}

main();
