#!/usr/bin/env node

const { execSync } = require('child_process');
const os = require('os');
const fs = require('fs');
const path = require('path');
const https = require('https');

const REPO = 'jee599/contextzip';

// Detect platform
const platform = os.platform();
const arch = os.arch();
const isWindows = platform === 'win32';

let target;
if (platform === 'darwin' && arch === 'arm64') target = 'contextzip-macos-arm64';
else if (platform === 'darwin' && arch === 'x64') target = 'contextzip-macos-x86_64';
else if (platform === 'linux' && arch === 'x64') target = 'contextzip-linux-x86_64';
else if (platform === 'win32' && arch === 'x64') target = 'contextzip-windows-x86_64.exe';
else {
  console.error(`❌ Unsupported platform: ${platform} ${arch}`);
  process.exit(1);
}

// Install directory
const INSTALL_DIR = isWindows
  ? path.join(os.homedir(), '.local', 'bin')
  : path.join(os.homedir(), '.local', 'bin');

const binName = isWindows ? 'contextzip.exe' : 'contextzip';
const binPath = path.join(INSTALL_DIR, binName);
const url = `https://github.com/${REPO}/releases/latest/download/${target}`;

console.log('');
console.log('⚡ ContextZip Installer');
console.log('─────────────────────────');
console.log(`  Platform: ${platform} ${arch}`);
console.log(`  Binary:   ${target}`);
console.log(`  Install:  ${binPath}`);
console.log('');

// Create install dir
if (!fs.existsSync(INSTALL_DIR)) {
  fs.mkdirSync(INSTALL_DIR, { recursive: true });
}

// Download with redirect support
function download(downloadUrl, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https.get(downloadUrl, (response) => {
      if (response.statusCode === 301 || response.statusCode === 302) {
        file.close();
        try { fs.unlinkSync(dest); } catch (_) {}
        download(response.headers.location, dest).then(resolve).catch(reject);
        return;
      }
      if (response.statusCode !== 200) {
        file.close();
        try { fs.unlinkSync(dest); } catch (_) {}
        reject(new Error(`HTTP ${response.statusCode}`));
        return;
      }
      response.pipe(file);
      file.on('finish', () => { file.close(resolve); });
    }).on('error', (err) => {
      try { fs.unlinkSync(dest); } catch (_) {}
      reject(err);
    });
  });
}

async function main() {
  try {
    console.log('📥 Downloading...');
    await download(url, binPath);

    if (!isWindows) {
      fs.chmodSync(binPath, 0o755);
    }
    console.log(`✅ Installed to ${binPath}`);

    // Check PATH
    const sep = isWindows ? ';' : ':';
    const pathDirs = (process.env.PATH || '').split(sep);
    if (!pathDirs.includes(INSTALL_DIR)) {
      if (isWindows) {
        console.log(`\n⚠️  Add to PATH:`);
        console.log(`   [System.Environment]::SetEnvironmentVariable("PATH", "$env:PATH;${INSTALL_DIR}", "User")`);
      } else {
        console.log(`\n⚠️  Add to PATH: export PATH="${INSTALL_DIR}:$PATH"`);
      }
    }

    // Run init
    console.log('\n🔧 Setting up Claude Code hook...');
    try {
      execSync(`"${binPath}" init -g --hook-only --auto-patch`, { stdio: 'inherit' });
    } catch (e) {
      console.log('   (hook setup skipped — run manually: contextzip init -g --auto-patch)');
    }

    console.log('');
    console.log('✅ Done! Restart Claude Code to activate.');
    console.log('');
    console.log('   Quick check:  contextzip --version');
    console.log('   Stats:        contextzip gain');
    console.log('');
  } catch (err) {
    console.error(`❌ Download failed: ${err.message}`);
    if (isWindows) {
      console.error(`   Download manually: https://github.com/${REPO}/releases/latest`);
    } else {
      console.error(`   Try: curl -fsSL https://raw.githubusercontent.com/${REPO}/main/install.sh | bash`);
    }
    process.exit(1);
  }
}

main();
