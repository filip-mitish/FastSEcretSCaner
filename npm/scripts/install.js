const { spawnSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const scannerPath = path.join(__dirname, '..', 'scanner');
const targetBinPath = path.join(__dirname, '..', 'bin', 'fsesc-native');

console.log('--- FSESC Deployment ---');

if (fs.existsSync(targetBinPath)) {
    console.log('Using pre-built native binary found in package.');
    try {
        fs.chmodSync(targetBinPath, 0o755);
        console.log('SUCCESS: Binary permission set.');
        process.exit(0);
    } catch (e) {
        console.warn('Warning: Could not set permissions on pre-built binary, attempting rebuild...');
    }
}

console.log('Attempting to build from source as fallback...');

const cargoTry = spawnSync('cargo', ['--version']);
if (cargoTry.error) {
    console.error('ERROR: Cargo not found and no working pre-built binary available.');
    process.exit(1);
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
        console.log('SUCCESS: Native binary built and installed.');
        process.exit(0);
    }
}

console.error('ERROR: Installation failed.');
process.exit(1);
