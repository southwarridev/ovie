# Security Policy — Ovie Programming Language

## Supported Versions

Security updates are provided for the following versions:

| Version | Supported |
|---------|-----------|
| 2.3.x   | ✅ Current |
| 2.2.x   | ✅ |
| 2.1.x   | ⚠️ Critical fixes only |
| 2.0.x   | ❌ |
| 1.x.x   | ❌ |
| 0.x.x   | ❌ |

## Reporting a Vulnerability

The Ovie team takes security seriously. If you discover a vulnerability in the Ovie compiler, toolchain, or standard library, please report it responsibly.

### How to Report

1. **GitHub Security Advisories** (preferred): [github.com/southwarridev/ovie/security/advisories/new](https://github.com/southwarridev/ovie/security/advisories/new)
2. **Email**: ovielang@gmail.com — include "SECURITY" in the subject line
3. **Details to include**:
   - Description of the vulnerability
   - Steps to reproduce
   - Affected versions
   - Potential impact
   - Suggested fix (if any)

### Response Timeline

| Stage | Time |
|-------|------|
| Acknowledgment | Within 48 hours |
| Initial assessment | Within 5 business days |
| Fix for critical issues | Within 30 days |
| Public disclosure | After fix is released |

## Security Principles

Ovie is designed with security as a core principle:

1. **Deterministic Builds** — identical inputs always produce identical outputs
2. **Offline-First** — no network access during compilation by default
3. **Supply Chain Security** — all dependencies cryptographically verified
4. **Memory Safety** — ownership system prevents common memory vulnerabilities
5. **No Telemetry** — zero data collection or tracking
6. **Transparent Operations** — all compiler operations are logged and auditable
7. **Module Security** — v2.3 module system validates imports and exports

## Scope

This policy covers:

- Ovie compiler (`oviec`)
- Ovie CLI (`ovie`)
- Aproko reasoning engine
- Standard library (all 11 modules including v2.3 `std::module` and `std::aproko`)
- Package management system
- Build system and dependency resolution

### Out of Scope

- Third-party packages and user-written Ovie code
- Infrastructure and hosting platforms (GitHub, Netlify, Vercel)

## Responsible Disclosure

We ask that security researchers:

- Give us reasonable time to address the issue before public disclosure
- Avoid accessing or modifying data that doesn't belong to you
- Don't perform actions that could harm service availability

## Recognition

Researchers who responsibly disclose vulnerabilities will be acknowledged in our security advisories and release notes (with permission).

## Contact

- **GitHub Advisories**: [github.com/southwarridev/ovie/security](https://github.com/southwarridev/ovie/security)
- **Email**: ovielang@gmail.com
- **GitHub**: [github.com/southwarridev/ovie](https://github.com/southwarridev/ovie)
- **GitLab**: [gitlab.com/ovie1/ovie](https://gitlab.com/ovie1/ovie)
