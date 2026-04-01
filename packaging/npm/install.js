#!/usr/bin/env node

// Downloads the correct ralph-engine binary for the current platform
// from GitHub Releases during `npm install`.
//
// Security: All exec calls use execFileSync (no shell injection).
// Only our own known binary paths and archive files are executed.

"use strict";

const { execFileSync } = require("child_process");
const crypto = require("crypto");
const fs = require("fs");
const https = require("https");
const os = require("os");
const path = require("path");
const { createWriteStream, mkdirSync } = fs;

const REPO = "diegorodrigo90/ralph-engine";
const RELEASE_API_BASE = `https://api.github.com/repos/${REPO}/releases/tags`;
const BIN_DIR = path.join(__dirname, "bin");
const BIN_NAME =
	os.platform() === "win32" ? "ralph-engine.exe" : "ralph-engine";
const BIN_PATH = path.join(BIN_DIR, BIN_NAME);

function getPlatform() {
	switch (os.platform()) {
		case "linux":
		case "darwin":
		case "win32":
			return os.platform();
		default:
			throw new Error(`Unsupported platform: ${os.platform()}`);
	}
}

function getTargetTriple() {
	switch (`${getPlatform()}/${os.arch()}`) {
		case "linux/x64":
			return "x86_64-unknown-linux-gnu";
		case "darwin/x64":
			return "x86_64-apple-darwin";
		case "darwin/arm64":
			return "aarch64-apple-darwin";
		case "win32/x64":
			return "x86_64-pc-windows-msvc";
		default:
			throw new Error(
				`Unsupported platform/architecture: ${getPlatform()}/${os.arch()}`,
			);
	}
}

function getArchiveExtension(targetTriple) {
	if (targetTriple.includes("windows")) {
		return "zip";
	}

	return "tar.xz";
}

function getBinaryAssetName(targetTriple) {
	return `re-cli-${targetTriple}.${getArchiveExtension(targetTriple)}`;
}

function getChecksumAssetName(binaryAssetName) {
	return `${binaryAssetName}.sha256`;
}

function getReleaseApiUrl(version) {
	return `${RELEASE_API_BASE}/v${version}`;
}

async function fetchJson(url) {
	const res = await httpsGet(url);
	let body = "";
	return new Promise((resolve, reject) => {
		res.setEncoding("utf8");
		res.on("data", (chunk) => {
			body += chunk;
		});
		res.on("end", () => {
			try {
				resolve(JSON.parse(body));
			} catch (err) {
				reject(new Error(`Invalid JSON from ${url}: ${err.message}`));
			}
		});
		res.on("error", reject);
	});
}

function findAsset(release, assetName) {
	if (!release.assets || !Array.isArray(release.assets)) {
		throw new Error("Release metadata did not include assets");
	}

	const asset = release.assets.find((candidate) => candidate.name === assetName);
	if (!asset) {
		throw new Error(`Release asset '${assetName}' was not found`);
	}

	return asset;
}

async function fetchReleaseAssetInfo(version, targetTriple) {
	const release = await fetchJson(getReleaseApiUrl(version));
	const binaryAssetName = getBinaryAssetName(targetTriple);
	const checksumAssetName = getChecksumAssetName(binaryAssetName);

	return {
		binaryAsset: findAsset(release, binaryAssetName),
		checksumAsset: findAsset(release, checksumAssetName),
	};
}

async function readText(url) {
	const res = await httpsGet(url);
	let body = "";
	return new Promise((resolve, reject) => {
		res.setEncoding("utf8");
		res.on("data", (chunk) => {
			body += chunk;
		});
		res.on("end", () => resolve(body));
		res.on("error", reject);
	});
}

async function readExpectedChecksum(url) {
	const checksumContents = (await readText(url)).trim();
	const [checksum] = checksumContents.split(/\s+/);

	if (!checksum || !/^[a-f0-9]{64}$/i.test(checksum)) {
		throw new Error(`Invalid SHA256 checksum payload from ${url}`);
	}

	return checksum.toLowerCase();
}

function computeSha256(filePath) {
	const hash = crypto.createHash("sha256");
	hash.update(fs.readFileSync(filePath));
	return hash.digest("hex");
}

function verifyChecksum(filePath, expectedChecksum) {
	const actualChecksum = computeSha256(filePath);

	if (actualChecksum !== expectedChecksum) {
		throw new Error(
			`Checksum mismatch for ${path.basename(filePath)} (expected ${expectedChecksum}, got ${actualChecksum})`,
		);
	}
}

function getBinaryPath(rootDir) {
	return path.join(rootDir, BIN_NAME);
}

function findExtractedBinary(rootDir) {
	const directBinary = getBinaryPath(rootDir);
	if (fs.existsSync(directBinary)) {
		return directBinary;
	}

	for (const entry of fs.readdirSync(rootDir, { withFileTypes: true })) {
		if (!entry.isDirectory()) {
			continue;
		}

		const nestedBinary = getBinaryPath(path.join(rootDir, entry.name));
		if (fs.existsSync(nestedBinary)) {
			return nestedBinary;
		}
	}

	return null;
}

function getInstallHelp(version) {
	return [
		"ralph-engine: you can install manually from the GitHub release assets:",
		`  https://github.com/${REPO}/releases/tag/v${version}`,
	].join("\n");
}

function getArch() {
	switch (os.arch()) {
		case "x64":
			return "amd64";
		case "arm64":
			return "arm64";
		default:
			throw new Error(`Unsupported architecture: ${arch}`);
	}
}

function getVersion() {
	const pkg = require("./package.json");
	return pkg.version;
}

function httpsGet(url, redirects = 0) {
	if (redirects > 5) {
		return Promise.reject(new Error(`Too many redirects (>5) for ${url}`));
	}
	return new Promise((resolve, reject) => {
		https
			.get(
				url,
				{ headers: { "User-Agent": "ralph-engine-npm" } },
				(res) => {
					if (
						res.statusCode >= 300 &&
						res.statusCode < 400 &&
						res.headers.location
					) {
						return httpsGet(
							res.headers.location,
							redirects + 1,
						).then(resolve, reject);
					}
					if (res.statusCode !== 200) {
						return reject(
							new Error(`HTTP ${res.statusCode} for ${url}`),
						);
					}
					resolve(res);
				},
			)
			.on("error", reject);
	});
}

async function downloadFile(url, dest) {
	const res = await httpsGet(url);
	return new Promise((resolve, reject) => {
		const file = createWriteStream(dest);
		res.pipe(file);
		file.on("finish", () => {
			file.close(resolve);
		});
		file.on("error", (err) => {
			fs.unlinkSync(dest);
			reject(err);
		});
	});
}

function extractTarXz(archive, dest) {
	execFileSync("tar", ["-xJf", archive, "-C", dest], { stdio: "pipe" });
}

// Windows ships tar.exe since Windows 10 build 17063 (2018) — supports .zip too.
function extractZip(archive, dest) {
	execFileSync("tar", ["-xf", archive, "-C", dest], { stdio: "pipe" });
}

async function main() {
	// Skip in CI or when RALPH_ENGINE_SKIP_INSTALL is set.
	if (process.env.RALPH_ENGINE_SKIP_INSTALL) {
		console.log(
			"ralph-engine: skipping binary download (RALPH_ENGINE_SKIP_INSTALL)",
		);
		return;
	}

	// If binary already exists and works, skip.
	if (fs.existsSync(BIN_PATH)) {
		try {
			execFileSync(BIN_PATH, ["version"], { stdio: "pipe" });
			console.log("ralph-engine: binary already installed");
			return;
		} catch {
			// Binary exists but doesn't work — re-download.
		}
	}

	const targetTriple = getTargetTriple();
	const version = getVersion();
	const assetInfo = await fetchReleaseAssetInfo(version, targetTriple);
	const binaryAssetUrl = assetInfo.binaryAsset.browser_download_url;
	const checksumAssetUrl = assetInfo.checksumAsset.browser_download_url;
	const archiveExtension = getArchiveExtension(targetTriple);

	console.log(
		`ralph-engine: downloading v${version} for ${getPlatform()}/${getArch()}...`,
	);

	const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "ralph-engine-"));
	const archive = path.join(tmpDir, `ralph-engine.${archiveExtension}`);

	try {
		await downloadFile(binaryAssetUrl, archive);
		verifyChecksum(archive, await readExpectedChecksum(checksumAssetUrl));

		mkdirSync(BIN_DIR, { recursive: true });

		if (archiveExtension === "zip") {
			extractZip(archive, tmpDir);
		} else {
			extractTarXz(archive, tmpDir);
		}

		const extracted = findExtractedBinary(tmpDir);

		if (!extracted) {
			throw new Error(`Binary not found in release asset for ${targetTriple}`);
		}

		fs.copyFileSync(extracted, BIN_PATH);
		fs.chmodSync(BIN_PATH, 0o750);

		console.log(`ralph-engine: installed v${version} to ${BIN_PATH}`);
	} catch (err) {
		console.error(
			`ralph-engine: failed to install binary — ${err.message}`,
		);
		console.error(getInstallHelp(version));
		process.exit(0); // Don't fail npm install.
	} finally {
		// Clean up tmp.
		fs.rmSync(tmpDir, { recursive: true, force: true });
	}
}

main();
