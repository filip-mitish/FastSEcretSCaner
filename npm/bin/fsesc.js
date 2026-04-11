#!/usr/bin/env node
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const binPath = path.join(__dirname, 'fsesc-native');

if (!fs.existsSync(binPath)) {
    console.error('Error: FSESC Native binary not found in package.');
    process.exit(1);
}

try {
    const stats = fs.statSync(binPath);
    if (!(stats.mode & fs.constants.S_IXUSR)) {
        fs.chmodSync(binPath, 0o755);
    }
} catch (e) {
}

const args = process.argv.slice(2);
const child = spawn(binPath, args, {
    stdio: 'inherit'
});

child.on('exit', (code) => {
    process.exit(code || 0);
});

child.on('error', (err) => {
    console.error('Failed to start native binary:', err.message);
    process.exit(1);
});
