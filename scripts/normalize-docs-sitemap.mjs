#!/usr/bin/env node

import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";

const targetFile = process.argv[2];

if (!targetFile) {
  console.error("usage: normalize-docs-sitemap.mjs <sitemap-file>");
  process.exit(1);
}

const absolutePath = path.resolve(process.cwd(), targetFile);
const rawXml = readFileSync(absolutePath, "utf8");

const normalizedXml = rawXml.replace(/https:\/\/ralphengine\.com(\/[^"<]*)?/g, (match, rawPath = "") => {
  if (rawPath === "/docs" || rawPath.startsWith("/docs/")) {
    return match;
  }

  if (rawPath === "") {
    return "https://ralphengine.com/docs";
  }

  if (rawPath === "/pt-br" || rawPath.startsWith("/pt-br/")) {
    return `https://ralphengine.com/docs${rawPath}`;
  }

  return `https://ralphengine.com/docs${rawPath}`;
});

writeFileSync(absolutePath, normalizedXml);
