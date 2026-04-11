# FSESC - Fast SEcret SCanner

Very blyat fast scanner of your stripe or github secrets.

## Core Features

- Performance: Built using memmap2 and rayon for high-speed scanning.
- Accuracy: Heuristic engine with confidence scoring to minimize false positives.
- Validation: Asynchronous API verification for detected credentials.
- Integration: Support for GitHub Actions and Git pre-commit hooks.

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
- `--verify`: Enable online verification for detected secrets (GitHub, Stripe).
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
This tool scans for sensitive information and may perform network requests if `--verify` is enabled. Use with caution in restricted environments.
