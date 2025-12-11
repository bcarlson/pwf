# Contributing to PWF

Thank you for your interest in improving the Portable Workout Format!

## Ways to Contribute

### 1. Report Issues

Found a bug in the validator or an ambiguity in the spec? [Open an issue](https://github.com/bcarlson/pwf/issues).

Include:
- What you expected
- What actually happened
- Sample YAML that demonstrates the issue

### 2. Propose Specification Changes

Have an idea for v1.1 or v2.0? Open a discussion or issue with:

- **Use case**: Why is this needed?
- **Proposed syntax**: Show example YAML
- **Backwards compatibility**: Will existing plans still work?
- **Validation rules**: What makes it valid/invalid?

### 3. Add Examples

Submit example plans that demonstrate:
- Different training styles (powerlifting, HIIT, endurance, calisthenics)
- Edge cases the validator should handle
- Real-world coaching scenarios

### 4. Improve Documentation

- Fix typos or clarify confusing sections
- Add translations
- Write integration guides for other platforms

### 5. Enhance the Validator

- Add new validation rules
- Improve error messages
- Add output formats
- Performance improvements

## Development Setup

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- Git

### Building

```bash
# Clone the repo
git clone https://github.com/bcarlson/pwf.git
cd pwf

# Build
cargo build

# Run tests
cargo test

# Build release binary
cargo build --release

# Run the CLI
./target/release/pwf --help
```

### Project Structure

```
pwf/
├── crates/
│   ├── pwf-core/      # Library crate (parsing, validation)
│   │   └── src/
│   │       ├── plan/      # Plan parsing & validation
│   │       └── history/   # History parsing & validation
│   └── pwf-cli/       # Binary crate (CLI)
├── schema/            # JSON Schema files
├── examples/          # Example YAML files
└── docs/              # Documentation
```

### Running Tests

```bash
# All tests
cargo test

# Specific crate
cargo test -p pwf-core

# With output
cargo test -- --nocapture
```

### Validating Examples

```bash
# Build CLI
cargo build --release

# Validate all example plans
./target/release/pwf validate examples/*.yaml

# Validate history exports
./target/release/pwf history examples/history-*.yaml

# Ensure invalid examples fail
./target/release/pwf validate examples/invalid/*.yaml && echo "FAIL: should have failed" || echo "OK: correctly rejected"
```

## Pull Request Process

1. **Fork** the repository
2. **Create a branch** (`git checkout -b feature/amazing-feature`)
3. **Make changes** following the style guide below
4. **Add tests** for new functionality
5. **Run tests** (`cargo test`) and ensure they pass
6. **Commit** with a descriptive message
7. **Push** to your fork
8. **Open a Pull Request**

### PR Checklist

- [ ] Tests pass locally
- [ ] New code has tests
- [ ] Documentation updated if needed
- [ ] Examples added for new features
- [ ] CHANGELOG.md updated

## Specification Changes

Changes to `SPECIFICATION.md` or `docs/` require:

1. **Discussion first** - Open an issue to discuss
2. **Backwards compatibility** - Existing valid files should remain valid
3. **JSON Schema update** - Keep schema in sync
4. **Example files** - Demonstrate new features
5. **Validator implementation** - Add validation rules

### Versioning

- **Patch (1.0.x)**: Bug fixes, clarifications, typo fixes
- **Minor (1.x.0)**: New optional features, new fields with defaults
- **Major (x.0.0)**: Breaking changes (new required fields, removed fields)

## Code Style

### Rust

- Follow `rustfmt` defaults (run `cargo fmt`)
- Use `clippy` (run `cargo clippy`)
- No `unwrap()` in library code (use proper error handling)
- Document public APIs with `///` comments

### YAML Examples

- Use 2-space indentation
- Include comments explaining non-obvious fields
- Keep examples realistic and useful

### Documentation

- Use ATX-style headers (`#`, `##`, `###`)
- Include code examples in fenced blocks
- Reference other docs using relative links

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help newcomers get started

## Questions?

- Open a [discussion](https://github.com/bcarlson/pwf/discussions)
- Check existing issues and PRs

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
