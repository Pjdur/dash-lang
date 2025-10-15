# Dash

Dash is a lightweight interpreted programming language written in Rust. It supports variables, arithmetic, control flow, functions, and return values — all powered by a custom grammar using [Pest](https://pest.rs/). Whether you're building a scripting engine, learning interpreters, or just having fun, Dash is a great place to start.

---

## ✨ Features

- ✅ Variables and arithmetic (`let x = 3 + 4`)
- ✅ Control flow: `if`, `while`, `break`, `continue`
- ✅ Functions with parameters and return values
- ✅ Print statements
- ✅ CLI support for running `.dash` files
- ✅ Custom grammar with Pest

---

## 🚀 Getting Started

### 1. Clone the repo

```bash
git clone https://github.com/Pjdur/dash-lang.git
cd dash
```

### 2. Build and run

```bash
cargo run
```

This runs a default hardcoded script. To run a file:

```bash
cargo run -- examples/hello.dash
```

Or build and run:

```bash
cargo build --release
./target/release/Dash examples/hello.dash
```

---

## 📄 Language Syntax

### Variables and Arithmetic

```lang
let x = 3 + 4
print(x)
```

### Control Flow

```lang
let x = 0
while x < 5 {
  print(x)
  let x = x + 1
}
```

### If / Else

```lang
if x > 10 {
  print("big")
} else {
  print("small")
}
```

### Functions and Return

```lang
fn add(a, b) {
  return a + b
}

let result = add(5, 7)
print(result)
```

### Break / Continue

```lang
while x < 10 {
  let x = x + 1
  if x == 5 {
    continue
  }
  if x == 8 {
    break
  }
  print(x)
}
```

---

## 📦 Project Structure

- `src/main.rs` — Entry point and CLI
- `lang.pest` — Grammar definition
- `Context`, `Expr`, `Stmt`, `Op` — Core AST and runtime structures
- `build_expr`, `build_stmt`, `exec_stmt`, `eval_expr` — Parser and interpreter logic

---

## 🧪 Examples

Create a file like `examples/hello.dash`:

```lang
fn greet(name) {
  print("Hello")
  print(name)
}

greet("World")
```

Then run:

```bash
cargo run -- examples/hello.dash
```

---

## 📚 Documentation

All core functions and data structures are documented with Rust-style `///` comments. You can generate docs with:

```bash
cargo doc --open
```

---

## 🤝 Contributing

Pull requests are welcome! If you’d like to add features (booleans, arrays, REPL, etc.), improve error handling, or optimize performance, feel free to fork and submit a PR.

---

## 📜 License

MIT License. See [LICENSE](LICENSE) for details.

---

## 💬 Credits

Built with ❤️ using [Rust](https://www.rust-lang.org/) and [Pest](https://pest.rs/). Inspired by classic interpreter designs and educational language projects.
