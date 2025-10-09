# Contributing to MCP Rust Example

Thank you for your interest in contributing! This document provides guidelines for contributing to this project.

## Development Workflow

### 1. Setup

```bash
# Clone the repository
git clone https://github.com/top-5/mcp-rust-example.git
cd mcp-rust-example

# Install dependencies (if needed)
./scripts/setup-ci.sh

# Build the project
./scripts/build.sh
```

### 2. Making Changes

```bash
# Create a feature branch
git checkout -b feature/your-feature-name

# Make your changes
# ... edit files ...

# Format code
cargo fmt

# Fix linting issues
cargo clippy --fix

# Build and test
./scripts/build.sh && ./scripts/test.sh
```

### 3. Pre-commit Checklist

Before committing, ensure:

- ✅ Code is formatted (`cargo fmt`)
- ✅ No Clippy warnings (`cargo clippy`)
- ✅ All tests pass (`./scripts/test.sh`)
- ✅ New features have tests
- ✅ Documentation is updated
- ✅ CHANGELOG updated (if applicable)

### 4. Submitting Changes

```bash
# Stage your changes
git add .

# Commit with a descriptive message
git commit -m "feat: Add your feature description"

# Push to your fork
git push origin feature/your-feature-name

# Create a Pull Request on GitHub
```

## Commit Message Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

Examples:
```
feat: Add JWT token rotation command
fix: Handle server shutdown gracefully
docs: Update README with CI/CD information
test: Add integration tests for token-manager
```

## Code Style

- Follow Rust 2021/2024 edition idioms
- Use `rustfmt` for formatting (automatic in CI)
- Address all `clippy` warnings
- Write descriptive variable and function names
- Add documentation comments for public APIs

## Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests

```bash
./scripts/test.sh
```

This script tests:
- Unit tests
- Token manager operations
- Server startup and health checks

### Manual Testing

```bash
# Start server
cargo run --bin launcher start

# Use token manager
cargo run --bin token-manager -- add testuser

# Test with client
cargo run --bin mcp-client

# Check logs
tail -f logs/mcp-server.log
```

## Adding New Features

### Adding a New Tool

1. Add tool method to `McpExampleServer` in `src/bin/mcp_server.rs`:

```rust
#[tool(description = "Your tool description")]
pub async fn your_tool(&self, param: String) -> Result<CallToolResult, ErrorData> {
    // Implementation
    Ok(CallToolResult::success(vec![Content::text("Result".to_string())]))
}
```

2. Test the tool:
   - Start server
   - Connect with VS Code
   - Invoke via Copilot

3. Update documentation:
   - Add to README.md features list
   - Add example to README.md tool examples

### Adding a New Binary

1. Create `src/bin/your_binary.rs`
2. Add to `Cargo.toml`:

```toml
[[bin]]
name = "your-binary"
path = "src/bin/your_binary.rs"
```

3. Update README.md documentation

## CI/CD Pipeline

The GitHub Actions workflow runs on:
- Push to `main`, `master`, `develop`
- Pull requests

Jobs:
- **build-and-test**: Format, lint, build, test, upload artifacts
- **wsl-test**: Windows WSL testing
- **security-audit**: Dependency vulnerability scanning
- **check-dependencies**: Check for outdated dependencies

All CI jobs must pass before merging.

## Questions?

- Open an issue for bugs or feature requests
- Check existing issues before creating new ones
- Be respectful and constructive in discussions

## License

By contributing, you agree that your contributions will be licensed under the Apache 2.0 License.
