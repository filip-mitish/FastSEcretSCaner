# FSESC - Fast SEcret SCanner

High-performance security tool for identifying API keys and credentials. Built with Rust for maximum efficiency and security. 100% local, no network requests.

## Architecture

FSESC consists of two parts:
1.  **Core (Rust)**: High-speed scanning engine located in `/npm/scanner`.
2.  **Wrapper (Node.js)**: CLI interface and git hook manager.

## Installation

### From NPM
```bash
npm install -g @tripock/fsesc
```

### From Source
```bash
git clone https://github.com/filip-mitish/FastSEcretSCaner.git
cd FastSEcretSCaner/npm
npm run build
npm install -g .
```

## Usage

### Scan Directory
```bash
fsesc scan .
```
- `--all`: Ignore `.gitignore` rules and scan everything.

### Git Pre-commit Hook
You can optionally install a local git hook to prevent accidental secret leaks:
```bash
fsesc install-hook
```
*Note: This command only modifies `.git/hooks/pre-commit` in the current repository. It is transparent and easy to remove.*

## Security & Privacy
- **No telemetry**: FSESC does not collect any data.
- **Local only**: All scanning logic stays on your machine. No external APIs are called.
- **Open Source**: Verify the logic yourself in `scanner/src/`.

## Author
tripock
