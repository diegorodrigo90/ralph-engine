#!/usr/bin/env node

import { cpSync, existsSync, mkdirSync, rmSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(scriptDir, "..");
const docsDir = path.join(rootDir, "docs");
const contentDir = path.join(docsDir, ".content");
const sourceDir = path.join(docsDir, "locales");

const localeRoots = [
  path.join(sourceDir, "en"),
  path.join(sourceDir, "pt-br"),
];

for (const localeRoot of localeRoots) {
  if (!existsSync(localeRoot)) {
    console.error(`docs locale sources are missing under ${sourceDir}`);
    process.exit(1);
  }
}

rmSync(contentDir, { recursive: true, force: true });
mkdirSync(path.join(contentDir, "pt-br"), { recursive: true });
mkdirSync(path.join(contentDir, ".vitepress"), { recursive: true });
mkdirSync(path.join(contentDir, "public"), { recursive: true });

cpSync(path.join(docsDir, ".vitepress", "config.mts"), path.join(contentDir, ".vitepress", "config.mts"));
cpSync(path.join(docsDir, ".vitepress", "theme"), path.join(contentDir, ".vitepress", "theme"), {
  recursive: true,
});
cpSync(path.join(docsDir, "public"), path.join(contentDir, "public"), { recursive: true });

const localeCopies = [
  {
    source: path.join(sourceDir, "en"),
    target: contentDir,
  },
  {
    source: path.join(sourceDir, "pt-br"),
    target: path.join(contentDir, "pt-br"),
  },
];

const contentSections = ["index.md", "getting-started", "guides", "reference", "development"];

for (const localeCopy of localeCopies) {
  for (const section of contentSections) {
    cpSync(path.join(localeCopy.source, section), path.join(localeCopy.target, section), {
      recursive: true,
    });
  }
}
