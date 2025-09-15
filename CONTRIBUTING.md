# Contributing to SIWT

Thank you for your interest in contributing to SIWT (Sign In With Telegram for Internet Computer)! We welcome contributions from the community and are grateful for your support.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Pull Request Process](#pull-request-process)
- [Issue Guidelines](#issue-guidelines)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Community](#community)

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- [DFX](https://internetcomputer.org/docs/current/developer-docs/setup/install/) (Internet Computer SDK)
- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) (latest stable)
- [Git](https://git-scm.com/)

### Development Setup

1. **Fork the repository**
   ```bash
   # Click the "Fork" button on GitHub, then clone your fork
   git clone https://github.com/lokaverse/SIWT.git
   ```

2. **Add upstream remote**
   ```bash
   git remote add upstream https://github.com/lokaverse/SIWT.git
   ```

3. **Install dependencies**
   ```bash
   npm install
   ```

4. **Start local development**
   ```bash
   dfx start --background
   dfx deploy
   ```

5. **Run tests**
   ```bash
   cargo test
   npm test
   ```

## How to Contribute

### Types of Contributions

We welcome several types of contributions:

- **Bug fixes** - Fix issues in the codebase
- **Feature development** - Add new functionality
- **Documentation** - Improve or add documentation
- **Testing** - Add or improve test coverage
- **Performance** - Optimize existing code
- **Security** - Identify and fix security issues

### Before You Start

1. **Check existing issues** - Look for existing issues or discussions
2. **Create an issue** - For new features or significant changes, create an issue first
3. **Discuss your approach** - Get feedback before starting major work

## Pull Request Process

### 1. Create a Branch

```bash
# Update your fork
git checkout main
git pull upstream main

# Create a feature branch
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number-description
```

### 2. Make Your Changes

- Follow our [coding standards](#coding-standards)
- Add tests for new functionality
- Update documentation as needed
- Ensure all tests pass

### 3. Commit Your Changes

```bash
# Stage your changes
git add .

# Commit with a descriptive message
git commit -m "feat: add telegram user verification"
# or
git commit -m "fix: resolve authentication timeout issue"
```

**Commit Message Format:**
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Adding or updating tests
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Maintenance tasks

### 4. Push and Create PR

```bash
# Push to your fork
git push origin feature/your-feature-name

# Create a pull request on GitHub
```

### 5. PR Requirements

Your pull request must:

- [ ] Pass all CI checks
- [ ] Include tests for new functionality
- [ ] Update documentation if needed
- [ ] Follow coding standards
- [ ] Have a clear description of changes
- [ ] Reference related issues

## Coding Standards

### Rust Code

- Use `cargo fmt` for formatting
- Follow Rust naming conventions
- Add documentation comments for public APIs
- Use `cargo clippy` and fix warnings
- Prefer explicit error handling over panics

```rust
// Good: Explicit error handling
fn validate_user(user_id: &str) -> Result<User, ValidationError> {
    if user_id.is_empty() {
        return Err(ValidationError::EmptyUserId);
    }
    // ...
}

// Bad: Panic on error
fn validate_user(user_id: &str) -> User {
    assert!(!user_id.is_empty(), "User ID cannot be empty");
    // ...
}
```

### JavaScript/TypeScript

- Use consistent indentation (2 spaces)
- Follow ESLint configuration
- Use meaningful variable names
- Add JSDoc comments for functions
- Prefer async/await over callbacks

### General Principles

- **SOLID principles** - Single responsibility, open/closed, etc.
- **DRY** - Don't repeat yourself
- **KISS** - Keep it simple, stupid
- **YAGNI** - You ain't gonna need it

## Testing

### Test Requirements

- **Unit tests** - Test individual functions/modules
- **Integration tests** - Test component interactions
- **End-to-end tests** - Test complete user flows

### Running Tests

```bash
# Rust tests
cargo test

# JavaScript tests
npm test

# All tests
npm run test:all

# Test coverage
cargo tarpaulin --out html
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_validation_success() {
        let user = User::new("valid_id", "username");
        assert!(validate_user(&user).is_ok());
    }

    #[test]
    fn test_user_validation_empty_id() {
        let user = User::new("", "username");
        assert!(validate_user(&user).is_err());
    }
}
```

## Documentation

### Types of Documentation

- **Code comments** - Explain complex logic
- **API documentation** - Document public interfaces
- **User guides** - How to use the library
- **Developer guides** - How to contribute

### Documentation Standards

- Use clear, concise language
- Include code examples
- Keep documentation up-to-date
- Use proper markdown formatting

## Recognition

We appreciate all contributions! Contributors will be:

- Listed in our README
- Mentioned in release notes
- Invited to our contributor Discord
- Eligible for contributor swag