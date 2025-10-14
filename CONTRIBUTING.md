## 📄 CONTRIBUTING.md

# 🤝 Contributing to Dash

Thanks for your interest in contributing to Dash — a lightweight interpreted language written in Rust!

---

## 🧰 Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/Pjdur/dash-lang.git
   cd dash-lang
   ```
3. Build and run:
   ```bash
   cargo run -- examples/hello.dash
   ```

---

## 🛠 What You Can Work On

- 🧠 Language features: booleans, arrays, logical operators
- 🧪 Tests: unit tests for parser and interpreter
- 🖥️ CLI: REPL mode, help/version flags
- 📚 Docs: grammar guide, usage examples
- 🧹 Refactoring: modularization, error handling

---

## 📦 Project Structure

```
src/
├── main.rs       # CLI entry point
├── parser.rs     # AST construction and grammar
├── eval.rs       # Expression and statement execution
├── ast.rs        # Core data types
examples/
├── hello.dash    # Sample programs
dash.pest     # Grammar definition
```

---

## ✅ Code Style

- Use Rust 2021 edition
- Format with `cargo fmt`
- Run `cargo clippy` before submitting

---

## 🧪 Testing

To run tests:

```bash
cargo test
```

To add a test, create a file in `tests/` or use inline unit tests in modules.

---

## 📬 Submitting a Pull Request

1. Create a new branch:
   ```bash
   git checkout -b feature/my-feature
   ```
2. Commit your changes:
   ```bash
   git commit -m "Add feature X"
   ```
3. Push and open a PR on GitHub

---

## 💬 Questions or Ideas?

Open an issue or start a discussion. We welcome feedback, suggestions, and creative ideas!

---

Thanks for helping make Dash better!
