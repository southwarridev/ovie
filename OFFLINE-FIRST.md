# 🔒 Ovie Programming Language - Offline-First Design

Ovie is designed from the ground up to work **completely offline**. This document explains our offline-first philosophy and how to use Ovie without any network dependencies.

## Why Offline-First?

1. **Privacy**: No telemetry, no tracking, no data collection
2. **Security**: No supply chain attacks through network dependencies
3. **Reliability**: Works in air-gapped environments, poor connectivity areas
4. **Determinism**: Identical builds regardless of network state
5. **Sovereignty**: Complete control over your development environment

## Offline Development Workflow

### Quick Start (Completely Offline)

```bash
# Clone or download the Ovie source code locally
# Then run the offline development script:

# Linux/macOS
./local-dev.sh

# Windows
./local-dev.ps1

# Or use Make
make offline-dev
```

### Manual Offline Setup

```bash
# 1. Build everything locally
cargo build --release --workspace

# 2. Install locally (no network required)
make install          # Linux/macOS
make install-windows  # Windows

# 3. Create a project (offline)
ovie new my-project
cd my-project

# 4. Build and run (offline)
ovie build
ovie run
```

## What Works Offline

✅ **Complete compiler pipeline**
- Lexing, parsing, semantic analysis
- IR generation and optimization
- Code generation (WASM, LLVM)

✅ **CLI toolchain**
- Project creation (`ovie new`)
- Building (`ovie build`)
- Running (`ovie run`)
- Testing (`ovie test`)
- Formatting (`ovie fmt`)

✅ **Aproko assistant**
- Code analysis and suggestions
- Error reporting and fixes
- Style and performance recommendations

✅ **Package management**
- Local dependency storage in `vendor/`
- Cryptographic verification
- Reproducible builds

✅ **Documentation and examples**
- Complete offline documentation in `docs/`
- Example programs in `examples/`
- Language specification in `spec/`

## Offline-First File Structure

```
ovie/
├── .gitignore          # Prevents online artifacts
├── local-dev.sh        # Offline development script
├── local-dev.ps1       # Windows offline development
├── Makefile            # Offline-first build system
├── vendor/             # Local dependencies (offline)
├── docs/               # Offline documentation
├── examples/           # Offline examples
└── target/             # Local build artifacts
```

## Network Operations (Optional)

Ovie includes optional network operations that require explicit user consent:

⚠️ **Optional Online Operations:**
- `./push-to-repos.sh` - Push to GitHub/GitLab (with confirmation)
- `make release` - Create distribution packages
- CI/CD pipelines (GitHub Actions, GitLab CI)

These operations:
- Require explicit user confirmation
- Are completely optional
- Don't affect core functionality
- Can be disabled by removing the scripts

## Preventing Accidental Network Access

### .gitignore Protection

Our comprehensive `.gitignore` prevents:
- Network configuration files
- Remote repository credentials
- Build artifacts that might contain URLs
- Temporary files with network references

### Build System Safeguards

- All `make` targets work offline by default
- Online operations are clearly marked with warnings
- Local development scripts emphasize offline nature
- No automatic network calls during builds

### Package Management

Ovie's package system is designed for offline-first operation:

```bash
# Dependencies are stored locally
vendor/
├── package-hash-1/
├── package-hash-2/
└── registry.toml

# Lock file ensures reproducible offline builds
ovie.lock
```

## Air-Gapped Development

Ovie works perfectly in air-gapped environments:

1. **Download source code** on a connected machine
2. **Transfer to air-gapped environment** via secure media
3. **Build and develop completely offline**
4. **No network required** for any development tasks

## Verification

To verify Ovie is working offline:

```bash
# Disconnect from network, then:
make clean
make build
make test
make demo

# Everything should work without network!
```

## Philosophy

> "A programming language should work anywhere, anytime, without asking permission from the internet."

Ovie embodies this philosophy by:
- **Never requiring network access** for core functionality
- **Storing everything locally** that's needed for development
- **Making network operations explicit** and optional
- **Prioritizing user privacy** and security
- **Ensuring reproducible builds** regardless of network state

## Support

All support resources work offline:
- Documentation in `docs/`
- Examples in `examples/`
- Language specification in `spec/`
- Error messages with built-in suggestions

For community support (optional online):
- GitHub Issues: https://github.com/southwarridev/ovie/issues
- GitLab Issues: https://gitlab.com/ovie1/ovie/-/issues

---

**Remember: Ovie is designed to work completely offline. Network access is always optional and explicit.**