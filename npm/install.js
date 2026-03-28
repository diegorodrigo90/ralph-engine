#!/usr/bin/env node

// Downloads the correct ralph-engine binary for the current platform
// from GitHub Releases during `npm install`.
//
// Security: All exec calls use execFileSync (no shell injection).
// Only our own known binary paths and archive files are executed.

"use strict";

const { execFileSync } = require("child_process");
const fs = require("fs");
const https = require("https");
const os = require("os");
const path = require("path");
const { createWriteStream, mkdirSync } = fs;

const REPO = "diegorodrigo90/ralph-engine";
const BIN_DIR = path.join(__dirname, "bin");
const BIN_NAME =
	os.platform() === "win32" ? "ralph-engine.exe" : "ralph-engine";
const BIN_PATH = path.join(BIN_DIR, BIN_NAME);

function getPlatform() {
	const platform = os.platform();
	switch (platform) {
		case "linux":
			return "linux";
		case "darwin":
			return "darwin";
		case "win32":
			return "windows";
		default:
			throw new Error(`Unsupported platform: ${platform}`);
	}
}

function getArch() {
	const arch = os.arch();
	switch (arch) {
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

function extractTarGz(archive, dest) {
	execFileSync("tar", ["-xzf", archive, "-C", dest], { stdio: "pipe" });
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

	const plat = getPlatform();
	const arch = getArch();
	const version = getVersion();
	const ext = plat === "windows" ? "zip" : "tar.gz";
	const url = `https://github.com/${REPO}/releases/download/v${version}/ralph-engine_${version}_${plat}_${arch}.${ext}`;

	console.log(`ralph-engine: downloading v${version} for ${plat}/${arch}...`);

	const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "ralph-engine-"));
	const archive = path.join(tmpDir, `ralph-engine.${ext}`);

	try {
		await downloadFile(url, archive);

		mkdirSync(BIN_DIR, { recursive: true });

		if (ext === "tar.gz") {
			extractTarGz(archive, tmpDir);
		} else {
			extractZip(archive, tmpDir);
		}

		// Find the binary in extracted files.
		const binaryName =
			plat === "windows" ? "ralph-engine.exe" : "ralph-engine";
		const extracted = path.join(tmpDir, binaryName);

		if (!fs.existsSync(extracted)) {
			throw new Error(`Binary not found in archive: ${extracted}`);
		}

		fs.copyFileSync(extracted, BIN_PATH);
		fs.chmodSync(BIN_PATH, 0o755);

		console.log(`ralph-engine: installed v${version} to ${BIN_PATH}`);
	} catch (err) {
		console.error(
			`ralph-engine: failed to install binary — ${err.message}`,
		);
		console.error("ralph-engine: you can install manually:");
		console.error(
			`  curl -fsSL https://raw.githubusercontent.com/${REPO}/main/scripts/install.sh | bash`,
		);
		console.error(
			`  go install github.com/${REPO}/cmd/ralph-engine@v${version}`,
		);
		process.exit(0); // Don't fail npm install.
	} finally {
		// Clean up tmp.
		fs.rmSync(tmpDir, { recursive: true, force: true });
	}
}

main();
