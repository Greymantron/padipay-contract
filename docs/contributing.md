# Contributing to PadiPay Soroban Escrow Contracts

PadiPay is an open-source escrow protocol built on **Stellar Soroban** with the goal of making trustless escrow accessible to everyday commerce through a Web2.5 experience.

Whether you're fixing a bug, improving documentation, writing tests, or implementing a new feature, your contribution helps move the project closer to production readiness.

---

# Before You Start

Please familiarize yourself with the project before opening a pull request.

We recommend reading the following documents in order:

* **README.md** — Project overview and local development setup
* **ARCHITECTURE.md** — Contract architecture and state flow
* **ROADMAP.md** — Current milestone and future direction
* Open GitHub Issues — Current work available for contributors

Understanding the project's architecture first will make implementation much smoother.

---

# Development Philosophy

The project follows a few simple principles:

* Build incrementally.
* Keep pull requests small and focused.
* Prefer reusable code over duplicated logic.
* Write tests alongside implementation.
* Keep documentation in sync with code.
* Optimize for maintainability rather than cleverness.

The current repository is intentionally focused on delivering a solid **v0.1.0 MVP** before introducing more advanced protocol features.

---

# Contribution Workflow

All work should begin from a GitHub Issue.

The expected workflow is:

```text
Choose an Open Issue
        │
        ▼
    Comment 
        │
        ▼
Fork the Repository
        │
        ▼
Create a Feature Branch
        │
        ▼
Implement the Issue
        │
        ▼
Run Formatting & Tests
        │
        ▼
Open a Pull Request
        │
        ▼
    Code Review
        │
        ▼
      Merge
```

## One Issue = One Pull Request

Each pull request should resolve **one GitHub Issue**.

Avoid combining multiple unrelated issues into a single PR.

Keeping PRs small makes reviews faster and reduces merge conflicts.

---

# Finding Something to Work On

Browse the repository's GitHub Issues.

If you're new to the project, look for labels such as:

* `good first issue`
* `help wanted`
* `mvp`

More experienced contributors may enjoy working on:

* Storage
* Authentication
* Token operations
* Testing
* Events
* Documentation
* CI improvements

If you'd like to work on an issue, leave a comment before starting so maintainers know it's being worked on.

---

# Branch Naming

Use descriptive branch names.

Examples:

```text
feat/create-escrow
feat/token-client
fix/state-validation
refactor/storage-helpers
docs/readme-refresh
test/authorization
ci/github-actions
```

---

# Commit Message Convention

Please use clear, conventional commit messages.

Examples:

```text
feat: implement create_escrow entrypoint

fix: validate escrow state transitions

refactor: extract storage helpers

test: add refund lifecycle tests

docs: improve architecture guide

ci: add GitHub Actions workflow
```

Small, meaningful commits make project history easier to follow.

---

# Rust Coding Standards

## Use Soroban SDK Types

Always prefer Soroban-native types over standard library collections.

Use:

* `Address`
* `Bytes`
* `BytesN`
* `Vec`
* `Map`
* `String`
* `Symbol`

Avoid standard collections such as:

* `Vec<T>` from `std`
* `HashMap`
* `String` from the Rust standard library

unless explicitly required outside the contract environment.

---

## Multi-Escrow Architecture

PadiPay implements an Escrow Manager architecture where a single deployed contract manages multiple concurrent escrow agreements. When contributing:

* Always require an `EscrowId` for state-modifying operations.
* Never use singleton storage (`instance`) for individual escrow state.
* Always use `persistent` storage keyed by the `EscrowId` for escrow records.

---

## Keep Functions Focused

Functions should have a single responsibility.

If a function grows significantly, extract reusable helpers instead of increasing complexity.

---

## Avoid Duplicate Logic

Shared functionality should live in helper modules.

Examples include:

* Storage operations
* Validation
* Authorization
* Token interactions

---

## Prefer Explicit Errors

Never panic for expected contract failures.

Use explicit contract errors instead.

Example scenarios include:

* Unauthorized access
* Invalid state transitions
* Missing escrow
* Invalid amounts

---

## Follow Existing Project Structure

When possible, extend existing modules instead of creating new ones.

Maintain consistency with the repository architecture.

---

# Testing Requirements

Every contribution that changes contract behavior must include corresponding tests.

At a minimum, test:

* Successful execution
* Failure conditions
* Authorization
* State transitions

Use the Soroban mock environment provided by the SDK.

Run the complete test suite before opening a PR.

```bash
cargo test
```

---

# Formatting & Linting

Before submitting your contribution, ensure the project passes formatting and linting.

```bash
cargo fmt

cargo clippy

cargo test
```

Pull requests that fail these checks may be requested for revision before review.

---

# Working with TODOs

The repository intentionally contains `TODO:` comments describing planned implementations.

When resolving a TODO:

* Implement the requested functionality.
* Remove the completed TODO.
* Ensure related documentation remains accurate.
* Add or update tests where necessary.

Avoid leaving partially completed TODOs.

---

# Documentation

Documentation is considered part of the codebase.

If your contribution changes:

* architecture
* workflows
* public interfaces
* contributor experience

please update the appropriate documentation.

This may include:

* README.md
* ARCHITECTURE.md
* ROADMAP.md
* CONTRIBUTING.md

---

# Pull Request Checklist

Before opening a pull request, verify the following:

* [ ] The change resolves a single GitHub Issue.
* [ ] Code follows existing project structure.
* [ ] Formatting passes (`cargo fmt`).
* [ ] Linting passes (`cargo clippy`).
* [ ] All tests pass (`cargo test`).
* [ ] New functionality includes tests.
* [ ] Documentation updated where necessary.
* [ ] Unrelated changes have not been included.

---

# Code Review Expectations

Maintainers may request changes before merging.

Common review feedback includes:

* Improving readability
* Increasing test coverage
* Reducing duplication
* Simplifying logic
* Better naming
* Documentation improvements

Reviews are collaborative and intended to improve the quality of the project.

---

# Definition of Done

A contribution is considered complete when:

* The linked GitHub Issue has been fully addressed.
* All acceptance criteria have been satisfied.
* Tests pass successfully.
* Formatting and linting succeed.
* Documentation has been updated if required.
* The pull request has received maintainer approval.

---

# Need Help?

If anything is unclear:

* Ask a question on the relevant GitHub Issue.
* Open a GitHub Discussion
* Reach out through the project's communication channels.

We welcome questions just as much as code contributions.

---

## Thank You 

Every contribution, whether it's code, tests, documentation, or feedback, helps make PadiPay a more reliable and accessible escrow protocol.

We appreciate your time, effort, and interest in building with us. Happy hacking! 

