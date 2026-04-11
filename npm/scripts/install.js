const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const scannerPath = path.join(__dirname, '..', '..', 'scanner');
const targetBinPath = path.join(__dirname, '..', 'bin', 'fsesc-native');

console.log('Building FSESC native binary...');

try {
    execSync('cargo --version', { stdio: 'ignore' });
} catch (e) {
    console.error('Error: "cargo" (Rust) is not installed. Please install Rust (https://rustup.rs/) to compile this tool.');
    process.exit(1);
}

try {
    execSync('cargo build --release', {
        cwd: scannerPath,
        stdio: 'inherit'
    });

    const sourceBin = path.join(scannerPath, 'target', 'release', 'fsesc');
    if (fs.existsSync(sourceBin)) {
        fs.copyFileSync(sourceBin, targetBinPath);
        fs.chmodSync(targetBinPath, 0o755);
        console.log('FSESC Native binary built and installed successfully.');
    } else {
        throw new Error('Binary not found after build.');
    }
} catch (e) {
    console.error('Failed to build native binary:', e.message);
    process.exit(1);
}
