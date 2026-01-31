# Manual Release Checklist for Ovie v2.1.0

## Pre-Release Preparation ✅

### 1. Version Updates
- [x] Updated Cargo.toml version to 2.1.0
- [x] Updated oviec/Cargo.toml version to 2.1.0
- [x] Updated aproko/Cargo.toml version to 2.1.0
- [x] Updated README.md with v2.1.0 information
- [x] Updated website/index.html with v2.1.0 branding

### 2. Code Quality
- [x] Fixed MIR compilation errors (MirPlaceKind issues resolved)
- [x] Repository cleanup (moved dev files to shedydev/)
- [x] Updated .gitignore to protect shedydev/
- [x] All core features implemented and tested

### 3. Documentation
- [x] Updated language guide
- [x] Updated API documentation
- [x] Created comprehensive examples
- [x] Updated installation instructions

## Manual Release Process 📦

### Step 1: Create GitHub Release
1. Go to: https://github.com/southwarridev/ovie/releases/new
2. **Tag version**: `v2.1.0`
3. **Release title**: `Ovie v2.1.0 - Low-Level Programming Language with Self-Hosting Capabilities`
4. **Description**: Copy content from `MANUAL_RELEASE_v2.1.0.md`

### Step 2: Build Binaries (If Available)
Since automated builds are failing, you can either:

#### Option A: Use Existing Binaries
- Use the binaries from the `releases/` folder if they're current
- Verify they work with the latest code changes

#### Option B: Manual Build (If Environment Allows)
```bash
# Build for current platform
cargo build --release --workspace

# Copy binaries to release folder
cp target/release/oviec releases/
cp target/release/ovie releases/
cp target/release/aproko releases/
```

### Step 3: Prepare Release Assets
Create these files for upload:

#### Core Binaries
- `ovie-v2.1.0-windows-x64.zip` (if available)
- `ovie-v2.1.0-linux-x64.tar.gz` (if available)
- `ovie-v2.1.0-macos-arm64.tar.gz` (if available)
- `ovie-v2.1.0-macos-x64.tar.gz` (if available)

#### VS Code Extension
- `ovie-lang-1.0.0.vsix` (from extensions/ovie-vscode/)

#### Documentation
- `MANUAL_RELEASE_v2.1.0.md` (this file)
- `README.md`
- `CHANGELOG.md` (if exists)

#### Checksums
- `checksums.txt` (if available)

### Step 4: Upload Assets
1. Drag and drop files into the release creation page
2. Ensure all files are properly uploaded
3. Verify file sizes and names are correct

### Step 5: Release Configuration
- [ ] **Set as latest release**: ✅ Check this box
- [ ] **Pre-release**: ❌ Leave unchecked (this is a stable release)
- [ ] **Generate release notes**: ❌ We have custom notes

### Step 6: Publish Release
1. Review all information
2. Click "Publish release"
3. Verify the release appears correctly

## Post-Release Tasks 📢

### Step 1: Update Documentation Sites
- [ ] Update https://ovie-lang.org (if applicable)
- [ ] Update documentation links
- [ ] Update download links

### Step 2: Community Announcement
- [ ] Post on GitHub Discussions
- [ ] Update social media (if applicable)
- [ ] Notify community channels

### Step 3: Monitor Release
- [ ] Watch for download activity
- [ ] Monitor for bug reports
- [ ] Respond to community feedback

## Fallback Options 🔄

### If Binaries Aren't Available
1. **Source-only release**: Release just the source code
2. **Documentation focus**: Emphasize the source code and documentation
3. **Community builds**: Ask community to help with builds

### Release Notes Template
```markdown
## What's New in v2.1.0
- Self-hosting compiler with bootstrap verification
- Low-level programming capabilities
- Enhanced Aproko analysis engine
- Multi-target code generation (LLVM, WASM, Interpreter)
- Comprehensive VS Code extension

## Installation
Download the source code and follow the installation guide in README.md

## Breaking Changes
None - fully backward compatible with v2.0

## Bug Fixes
- Fixed MIR compilation issues
- Resolved CI/CD pipeline problems
- Improved error handling and reporting
```

## Emergency Contacts 🚨

If issues arise during release:
1. Check GitHub Actions for automated build status
2. Review recent commits for any breaking changes
3. Consider rolling back to previous stable state if needed

## Success Criteria ✅

Release is successful when:
- [ ] GitHub release is published and visible
- [ ] Assets are downloadable (if available)
- [ ] Documentation is accessible
- [ ] No critical issues reported in first 24 hours
- [ ] Community feedback is positive

---

**Note**: Due to CI/CD issues, this is a manual release process. Future releases should use automated pipelines once build issues are resolved.