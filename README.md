# Rust 101

Assimilate Rust from zero [Rust Book](https://doc.rust-lang.org/book/)

---

## Install

1. Run the following in the local terminal

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Include Cargo's bin directory. Configure the current shell:

```bash
chmod 777 $HOME/.cargo/env
source $HOME/.cargo/env
```

---

## Programming a Guessing Game

### Setup a new project

```bash
cargo new guessing_game
cd guessing_game
```

### Compile the default program

```bash
cargo run
```

