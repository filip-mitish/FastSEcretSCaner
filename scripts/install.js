const { spawnSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const scannerPath = path.join(__dirname, '..', 'scanner');
const targetBinPath = path.join(__dirname, '..', 'bin', 'fsesc-native');

console.log('--- FSESC: Compiling native engine from source ---');

const cargoCheck = spawnSync('cargo', ['--version']);
if (cargoCheck.error) {
    console.warn('Warning: Cargo not found. Using pre-built binary if available.');
    process.exit(0);
}

const build = spawnSync('cargo', ['build', '--release'], {
    cwd: scannerPath,
    stdio: 'inherit'
});

if (build.status === 0) {
    const sourceBin = path.join(scannerPath, 'target', 'release', 'fsesc');
    if (fs.existsSync(sourceBin)) {
        fs.copyFileSync(sourceBin, targetBinPath);
        fs.chmodSync(targetBinPath, 0o755);
        console.log('SUCCESS: Native engine compiled and installed.');
    } else {
        console.warn('Warning: Compiled binary not found where expected.');
    }
} else {
    console.warn('Warning: Build failed. Falling back to pre-built binary.');
}
