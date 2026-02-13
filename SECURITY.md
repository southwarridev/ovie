# Security Policy

## Supported Versions

The Ovie programming language is currently in active development. Security updates will be provided for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 2.1.x   | :white_check_mark: |
| 2.0.x   | :white_check_mark: |
| 1.x.x   | :x:                |
| 0.x.x   | :x:                |

## Reporting a Vulnerability

The Ovie team takes security seriously. If you discover a security vulnerability in the Ovie programming language, compiler, or toolchain, please report it responsibly.

### How to Report

1. **Email**: Send details to security@ovie-lang.org, ovielang@gmail.cm
2. **Subject**: Include "SECURITY" in the subject line
3. **Details**: Provide as much information as possible:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### What to Expect

- **Acknowledgment**: We will acknowledge receipt within 48 hours
- **Assessment**: Initial assessment within 5 business days
- **Updates**: Regular updates on our progress
- **Resolution**: We aim to resolve critical issues within 30 days

### Security Principles

The Ovie language is designed with security as a core principle:

1. **Deterministic Builds**: All builds are reproducible and verifiable
2. **Offline-First**: No network access during compilation by default
3. **Supply Chain Security**: All dependencies are cryptographically verified
4. **Memory Safety**: Ownership system prevents common memory vulnerabilities
5. **No Telemetry**: Zero data collection or tracking
6. **Transparent Operations**: All compiler operations are logged and auditable

### Scope

This security policy covers:

- Ovie compiler (oviec)
- Ovie toolchain (ovie CLI)
- Aproko assistant engine
- Standard library
- Package management system
- Build system and dependency resolution

### Out of Scope

- Third-party packages and dependencies
- User-written Ovie code
- Infrastructure and hosting platforms

### Responsible Disclosure

We request that security researchers:

- Give us reasonable time to address the issue before public disclosure
- Avoid accessing or modifying data that doesn't belong to you
- Don't perform actions that could harm the availability of our services
- Don't use social engineering against our team members

### Recognition

We maintain a security hall of fame for researchers who responsibly disclose vulnerabilities. With your permission, we'll acknowledge your contribution in our security advisories and release notes.

## Security Features

### Compiler Security

- **Safe by Default**: Memory safety without garbage collection
- **Explicit Unsafe**: All unsafe operations must be explicitly marked
- **Effect System**: Track and control side effects
- **Ownership Checking**: Prevent use-after-free and data races

### Build Security

- **Reproducible Builds**: Identical inputs produce identical outputs
- **Dependency Verification**: Cryptographic hashes for all dependencies
- **Offline Builds**: No network access during compilation
- **Supply Chain Isolation**: Local dependency storage and verification

### Runtime Security

- **No Hidden Network**: Explicit permission required for network access
- **Sandboxed Execution**: Controlled execution environment
- **Resource Limits**: Configurable resource usage limits
- **Audit Logging**: Comprehensive operation logging

## Contact

For security-related questions or concerns:

- Email: security@ovie-lang.org
- GPG Key: [Available on our website]
- Response Time: 48 hours for acknowledgment

For general questions about Ovie:

- Website: https://ovie-lang.org
- Documentation: https://docs.ovie-lang.org
- Community: https://community.ovie-lang.org