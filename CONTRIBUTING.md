# Contributing to PHP Sunshine

Thank you for your interest in contributing to PHP Sunshine! This guide will help you get started.

## Development Setup

1. **Install Rust** (already done! ✅)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone and build**
   ```bash
   cd php-sunshine
   cargo build
   ```

3. **Run in development mode**
   ```bash
   cargo run
   ```

## Project Structure

- `src/main.rs` - Entry point and main application loop
- `src/editor/` - Text editing core (text buffers, cursor management)
- `src/ui/` - Terminal UI components
- `src/parser/` - PHP and template language parsers
- `src/lsp/` - Language Server Protocol implementation
- `src/project/` - Project detection and framework-specific features

## Key Technologies

- **Ratatui** - Terminal UI framework
- **Crossterm** - Cross-platform terminal manipulation
- **Ropey** - Fast text buffer based on ropes
- **Tree-sitter** - Incremental parsing for syntax highlighting
- **Tokio** - Async runtime for non-blocking operations

## Development Workflow

1. Create a new branch for your feature
2. Write code with tests
3. Run tests: `cargo test`
4. Check formatting: `cargo fmt`
5. Check lints: `cargo clippy`
6. Build release: `cargo build --release`

## Feature Priorities

### Phase 1: Basic Editor (Current)
- ✅ Terminal UI setup
- 🔄 File opening and saving
- 🔄 Basic text editing
- 🔄 Cursor movement

### Phase 2: PHP Support
- PHP syntax highlighting
- Code folding
- Line numbers

### Phase 3: Framework Detection
- PrestaShop project detection
- Laravel project detection  
- Symfony project detection

### Phase 4: Advanced Features
- Code completion
- Go to definition
- Refactoring tools

## Code Style

- Follow Rust conventions
- Use `cargo fmt` before committing
- Keep functions focused and small
- Write documentation for public APIs
- Add tests for new features

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Debugging

Enable debug logging:
```bash
RUST_LOG=debug cargo run
```

## Questions?

Feel free to open an issue for any questions or suggestions!

---

Happy coding! ☀️🦀
