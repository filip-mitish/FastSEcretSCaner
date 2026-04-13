# FSESC - Fast SEcret SCanner

Very blyat fast scanner of your stripe or github secrets.

## Core Features

- Performance: Built using memmap2 and rayon for high-speed scanning.
- Accuracy: Heuristic engine with confidence scoring to minimize false positives.
- Integration: Support for GitHub Actions and Git pre-commit hooks.
- Local-only: No external network requests, ensuring data privacy.

## Installation

```bash
npm install -g @tripock/fsesc
```
*Note: Requires cargo (Rust) to be installed on your system for native compilation.*

## Usage

### Direct Scanning
```bash
fsesc scan <path>
```

### Advanced Options
- `--all`: Scan all files, bypassing default ignored patterns (.gitignore).
- `install-hook`: Register local git pre-commit hook.

## Integration

### GitHub Actions
Reference `.github/workflows/fsesc.yml` or use the provided `action.yml`.

### Pre-commit Framework
Add the following to your `.pre-commit-config.yaml`:

```yaml
- repo: local
  hooks:
    - id: fsesc
      name: fsesc
      entry: fsesc scan .
      language: system
```

## Security
This tool scans for sensitive information locally. It does not perform any network requests.
