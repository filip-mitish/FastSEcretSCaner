#!/usr/bin/env node
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const binPath = path.join(__dirname, 'fast-secret-scanner-native');

if (!fs.existsSync(binPath)) {
    console.error('Error: Native binary not found. Please run "npm install" to build it.');
    process.exit(1);
}

// Pass all arguments to the Rust binary
const args = process.argv.slice(2);
const child = spawn(binPath, args, {
    stdio: 'inherit'
});

child.on('exit', (code) => {
    process.exit(code);
});
