#!/usr/bin/env node
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const binPath = path.join(__dirname, 'fsesc-native');

if (!fs.existsSync(binPath)) {
    console.error('Error: FSESC Native binary not found. Please run "npm install" to build it.');
    process.exit(1);
}

const args = process.argv.slice(2);
const child = spawn(binPath, args, {
    stdio: 'inherit'
});

child.on('exit', (code) => {
    process.exit(code);
});
