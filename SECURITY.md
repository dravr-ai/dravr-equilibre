# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |

## Reporting a Vulnerability

If you discover a security vulnerability in dravr-equilibre, please report it responsibly:

1. **Do not** open a public GitHub issue
2. Email **security@dravr.ai** with a description of the vulnerability
3. Include steps to reproduce, if possible
4. You will receive an acknowledgment within 48 hours

We will work with you to understand the issue and coordinate a fix before any public disclosure.

## Security Model

dravr-equilibre provides health domain models and composition-based provider traits. The security boundary is:

- **No secrets in core** — the library stores no API keys or tokens; authentication is handled by provider implementations
- **Input validation** — all domain models validate inputs at construction boundaries
- **Type-safe deduplication** — device and provider priority systems use strongly-typed enums to prevent misuse
- **No network I/O in core** — the core library performs no network operations; provider traits are implemented externally
