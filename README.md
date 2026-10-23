
# U/R — Unrestricted Browser

**A Human-Built, Open-Source, Non-Chromium Browser.**

U/R (Unrestricted) is an experimental browser project developed under the **ZTL (Zeo Thinking Labs) Foundation**.

The project explores the challenge of building a browser with a focus on cybersecurity, privacy, user control, and an architecture that does not depend on Chromium.

U/R is being developed in Rust, with the long-term goal of building browser components from the ground up.

> **The mission:** Build tools that outlive their creator.

---

## Table of Contents

- [About the Project](#about-the-project)
- [Project Goals](#project-goals)
- [Core Philosophy](#core-philosophy)
- [Architecture and Modules](#architecture-and-modules)
- [Technology](#technology)
- [Privacy](#privacy)
- [Security](#security)
- [Development Status](#development-status)
- [Human vs AI Experiment](#human-vs-ai-experiment)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [Contributing](#contributing)
- [License](#license)

---

## About the Project

U/R stands for **Unrestricted**.

It is an experiment in browser engineering, systems programming, and cybersecurity-focused software development.

Rather than building another Chromium-based browser, U/R aims to explore an independent browser architecture, including the components required to process and display web content.

The project is being developed incrementally, starting with Rust fundamentals and evolving toward more complex systems.

U/R is not intended to be just another browser interface. The long-term goal is to understand and build the underlying technology that makes a browser work.

## Project Goals

The project aims to:

- Build a non-Chromium browser.
- Develop core components using Rust.
- Explore HTML parsing and CSS processing.
- Develop a rendering and layout system.
- Investigate JavaScript integration.
- Implement networking and navigation components.
- Use SQLite for appropriate local data storage.
- Prioritize privacy and user control.
- Explore cybersecurity tooling and browser security.
- Keep the project open source and publicly developed.

These are development goals, not claims that every feature is already implemented.

## Core Philosophy

### 1. Non-Chromium Architecture

U/R is intended to use an architecture independent of Chromium.

The long-term goal is to build the browser's core components rather than relying on Chromium as the underlying browser engine.

### 2. Open Source

U/R is intended to be developed openly, with source code available through GitHub.

The project encourages transparency, experimentation, and contributions from developers interested in browser engineering.

### 3. Privacy First

U/R aims to avoid unnecessary collection of user information.

The intended design favors local storage and user control over centralized accounts, tracking, and cloud-based browsing data collection.

### 4. Security as an Engineering Discipline

Security is a continuous development responsibility.

U/R aims to investigate browser security, minimize unnecessary privileges, use memory-safe Rust where possible, and evaluate risks throughout development.

### 5. Build to Understand

U/R is also a learning project.

The goal is not merely to produce software that works, but to understand the systems, trade-offs, and engineering decisions behind it.

---

## Architecture and Modules

U/R is organized around modules with distinct responsibilities.

The following describes the intended architecture; individual modules may still be under development.

| Module | Intended Responsibility |
|---|---|
| `Zeo` | Cybersecurity and penetration-testing tools from various sources |
| `Unrestriced` | Core browser architecture, user interface, system design, and technical diagrams |
| `Network` | Networking and communication components |
| `JavaScript` | JavaScript-related components and integration |
| `SQLite` | Database-related components and local persistence |
| `Security` | Security architecture, protections, and security-related policies |

The project aims to keep the main entry point focused on coordinating modules instead of containing the entire implementation.

As the architecture develops, module boundaries and responsibilities may evolve.

## Technology

### Rust

Rust is the primary implementation language.

It provides tools for systems programming, explicit type handling, and memory safety without requiring a garbage collector for ordinary Rust-owned memory.

### SQLite

SQLite is intended for database-related functionality and appropriate local persistence.

### Networking and Web Technologies

Networking, HTML, CSS, and JavaScript are important parts of the long-term browser-engine goal.

Their implementation details will be documented as the relevant components are developed.

## Privacy

Privacy is a core design objective.

U/R aims to:

- Avoid mandatory user accounts.
- Avoid unnecessary telemetry and behavioral tracking.
- Keep browser data local where applicable.
- Avoid mandatory cloud synchronization.
- Make external data transmission transparent.

Visiting external websites still involves network communication, and those websites may receive connection metadata and other information.

See [PRIVACY.md](PRIVACY.md) for the project's privacy policy.

## Security

U/R is intended to be security-conscious software, with security treated as an ongoing engineering process.

Potential areas of focus include:

- Network and URL handling.
- HTML and CSS parsing.
- JavaScript execution and isolation.
- Memory safety.
- File and download handling.
- Local data protection.
- Dependency security.

U/R is experimental software. Its security properties depend on the features implemented and tested in each version.

See [SECURITY.md](SECURITY.md) for the security policy and vulnerability-reporting guidance.

## Development Status

**Status: Experimental / Under Development**

U/R is an active experiment in Rust and browser engineering.

The project is not yet a complete, production-ready browser. Features will be introduced incrementally as the architecture develops.

The long-term roadmap includes:

- [ ] Establish the core Rust project structure.
- [ ] Develop modular browser architecture.
- [ ] Implement URL handling and navigation logic.
- [ ] Develop networking components.
- [ ] Explore HTML parsing.
- [ ] Explore CSS parsing and layout.
- [ ] Develop a rendering system.
- [ ] Integrate JavaScript execution.
- [ ] Implement appropriate local persistence.
- [ ] Develop and test browser security protections.
- [ ] Work toward support for Windows and GNU/Linux environments.

This checklist represents project goals, not a guarantee of implementation or a release schedule.

## Human vs AI Experiment

U/R is part of an experiment exploring the development of an autonomous, non-Chromium browser through a human-versus-AI engineering challenge.

- **U/R:** The human-built browser project.
- **Excalibur:** The AI-side browser project.

The experiment explores how different approaches to software development, architecture, reasoning, and implementation compare when building complex software.

The objective is not just to produce a result, but to learn from the process of engineering it.

## Project Structure

The repository is organized to separate the main application from its modules and supporting documentation.

An illustrative structure is:

```text
U-R/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── PRIVACY.md
├── SECURITY.md
├── src/
│   └── main.rs
└── learning/
    └── Zeo.rs
```

The actual structure may change as new modules and components are introduced.

## Getting Started

### Prerequisites

- Git
- Rust toolchain (`rustc` and `cargo`)
- A supported development environment

### Clone the Repository

```bash
git clone https://github.com/Zeo120/U-R.git
cd U-R
```

### Build the Project

```bash
cargo build
```

### Run the Main Binary

```bash
cargo run
```

### Run the Custom `Zeo` Binary

The repository currently defines a custom binary named `Zeo` in `Cargo.toml`.

```bash
cargo run --bin Zeo
```

Build and execution behavior may change as development progresses.

## Contributing

Contributions, suggestions, bug reports, and discussions are welcome.

Before contributing:

1. Review the existing project structure.
2. Keep changes focused on their intended responsibilities.
3. Test changes where practical.
4. Document security-relevant behavior and limitations.
5. Avoid introducing unnecessary data collection or external dependencies.

For security issues, consult [SECURITY.md](SECURITY.md).

## License

A license has not yet been specified in this README.

Until a license is added to the repository, do not assume that the source code is available for unrestricted reuse, redistribution, or modification.

## Project

**U/R — Unrestricted Browser**

Developed under the **ZTL (Zeo Thinking Labs) Foundation**.

Repository: https://github.com/Zeo120/U-R

*Built to understand. Built to explore. Built to outlive its creator.*
