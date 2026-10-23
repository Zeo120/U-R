
# Security Policy — Project U/R

**Last Updated:** October 21, 2026

## 1. Overview

Project U/R (Unrestricted) is an open-source, non-Chromium browser developed under the ZTL (Zeo Thinking Labs) Foundation.

Security is a core consideration in the design and development of U/R. As a browser focused on cybersecurity, privacy, and user control, the project aims to identify, investigate, and address security vulnerabilities transparently.

However, U/R is under active development. Security features, protections, and guarantees depend on the implementation available in each release.

## 2. Supported Versions

Until stable releases and a formal support lifecycle are established, security fixes will primarily target the latest actively maintained version.

| Version | Security Support |
|---|---|
| Development / Latest | Active development; fixes considered |
| Older versions | Not guaranteed |

Users are encouraged to use the latest available version and review release notes for security-related changes.

## 3. Reporting a Vulnerability

If you discover a potential security vulnerability in Project U/R, please report it responsibly.

Include the following information where possible:

- A clear description of the vulnerability.
- Steps to reproduce the issue.
- The affected version, commit, or build.
- Expected behavior and actual behavior.
- Proof-of-concept code or screenshots, if safe and relevant.
- Potential impact and any suggested mitigation.

**Do not include passwords, private keys, personal information, or other sensitive data in public reports.**

For now, reports may be submitted through the official GitHub repository:

https://github.com/Zeo120/U-R

For vulnerabilities that could affect users, avoid publicly disclosing exploitable details until the maintainers have had a reasonable opportunity to investigate and coordinate a fix.

Do not assume that a private reporting channel exists unless one has been explicitly established.

## 4. Response and Remediation

When a vulnerability is reported, the maintainers aim to:

1. Review the report and determine whether it is reproducible.
2. Assess the potential impact and severity.
3. Investigate the underlying cause.
4. Develop and test an appropriate fix or mitigation.
5. Document relevant changes in the repository or release notes.

Response times may vary depending on the severity, complexity, and available maintenance resources. No fixed response or resolution time is guaranteed.

## 5. Scope

Security reports relevant to Project U/R may include vulnerabilities involving:

- URL and navigation handling.
- Network communication and protocol handling.
- HTML parsing and document processing.
- CSS parsing and layout processing.
- JavaScript execution and isolation.
- Memory safety and unsafe Rust code.
- File access and download handling.
- Cookies, local storage, and other website data.
- Browser permissions and origin isolation.
- SQLite databases and persistent browser state.
- Build scripts, dependencies, and software distribution.
- Privacy violations, unintended data transmission, and security boundary failures.

This list is illustrative and may change as the browser develops.

## 6. Responsible Testing

Security researchers are encouraged to test U/R within lawful and authorized environments.

Please do not:

- Access or modify data belonging to other users without authorization.
- Disrupt infrastructure or services that you do not own or have permission to test.
- Use vulnerabilities to compromise third-party systems.
- Publish sensitive user data obtained during testing.
- Misrepresent an unverified issue as a confirmed vulnerability.

Testing should be limited to systems and environments for which you have authorization.

## 7. Third-Party Dependencies

Project U/R may use third-party libraries, tools, and components.

The security of these dependencies can affect the security of the browser. Dependencies should be reviewed and updated as appropriate, and known relevant vulnerabilities should be evaluated for their applicability to U/R.

The presence of a dependency in the project does not imply that it has been independently audited.

## 8. Security Limitations

Project U/R is experimental software under active development.

Until relevant features have been implemented and tested, U/R must not be assumed to provide complete protection against malicious websites, memory corruption, tracking, sandbox escapes, or other browser threats.

No software can guarantee absolute security. Users should exercise caution when handling sensitive information and avoid relying on experimental builds for security-critical activities without an appropriate risk assessment.

## 9. Security Design Principles

Project U/R aims to follow these principles:

- **Transparency:** Security-relevant code and changes should be reviewable.
- **Privacy:** Avoid unnecessary collection or transmission of user data.
- **Least privilege:** Components should receive only the permissions they need.
- **Isolation:** Website content and browser components should be separated where technically feasible.
- **Memory safety:** Prefer safe Rust and minimize the use of `unsafe` code.
- **Defense in depth:** Use multiple complementary protections rather than relying on a single security mechanism.
- **Responsible disclosure:** Investigate and address vulnerabilities with appropriate care.

These are development objectives, not claims that every principle is already fully implemented.

## 10. Security Updates

Security fixes and relevant advisories should be documented through the official GitHub repository, commits, releases, or other project-maintained announcements.

Users should review updates and release notes before upgrading, particularly when a release addresses a security vulnerability.

## 11. Policy Changes

This policy may be revised as Project U/R evolves, its architecture matures, and formal release and vulnerability-handling processes are established.

The latest version of this document will be maintained in the official repository.

## 12. Final Statement

Project U/R is an experiment in building an open-source, non-Chromium browser with a strong emphasis on privacy, security, and user control.

Security is an ongoing engineering responsibility, not a claim of perfection. Vulnerabilities should be investigated, documented, and addressed as the project matures.

**Security is not a checkbox. It is a continuous process.**
