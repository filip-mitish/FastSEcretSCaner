const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');
const os = require('os');

const scannerPath = path.join(__dirname, '..', 'scanner');
const targetBinPath = path.join(__dirname, '..', 'bin', 'fsesc-native');

console.log('--- FSESC Installation Support ---');

function getCargoPath() {
    try {
        return execSync('which cargo').toString().trim();
    } catch (e) {
        const home = os.homedir();
        const commonPaths = [
            path.join(home, '.cargo', 'bin', 'cargo'),
            '/usr/bin/cargo',
            '/usr/local/bin/cargo'
        ];
        for (const p of commonPaths) {
            if (fs.existsSync(p)) return p;
        }
    }
    return 'cargo';
}

const cargo = getCargoPath();
console.log(`Using cargo: ${cargo}`);

try {
    console.log('Attempting to build native binary from source...');
    execSync(`${cargo} build --release`, {
        cwd: scannerPath,
        stdio: 'inherit',
        env: { ...process.env, PATH: `${process.env.PATH}:${path.join(os.homedir(), '.cargo', 'bin')}` }
    });

    const sourceBin = path.join(scannerPath, 'target', 'release', 'fsesc');
    if (fs.existsSync(sourceBin)) {
        fs.copyFileSync(sourceBin, targetBinPath);
        fs.chmodSync(targetBinPath, 0o755);
        console.log('SUCCESS: Native binary built and installed.');
        process.exit(0);
    }
} catch (e) {
    console.warn('WARNING: Build from source failed. Checking for pre-built binary...');
}

if (fs.existsSync(targetBinPath)) {
    console.log('SUCCESS: Using existing pre-built binary.');
    process.exit(0);
}

console.error('ERROR: Could not build or find fsesc-native binary.');
process.exit(1);
