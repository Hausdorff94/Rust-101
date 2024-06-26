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

### Write the code for Guessing Game

Simple code where the user enters a variable on the console and the program prints it.

```rust
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

---

## Crate

A crate is a collection of Rust source code files ([see more](https://crates.io/crates/rand)). Those dependencies must be add in the `Cargo.toml`file:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.5"
```

---

## Shadowing

Shadowing lets us reuse the `guess` variable name rather than forcing us to create two unique variables, such as `guess_str` and `guess`. Let's convert the String input to number type.

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

---

## Loops

The keyword `loop` creates an infinite loop. So, we need to stop the game when the correct number is guessed. Let’s program the game to quit when the user wins by adding a `break` statement.

```rust
// ...

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too Small!"),
    Ordering::Greater => println!("Too Big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}

// ...
```

---

## Handling Invalid Inputs

We will use a `match` expression to handle the error with arms. If `parse` is able to successfully turn the string into a number, it will return an `ok` value that contains the resultant number. Else, it will return an `Error` variant.

```rust

let guess: u32 = match guess.trim().parse(){
    Ok(num) => num,
    Err(_) => continue,
};

```

## Basic Commands

### Init a new default project

Create a new project (binary/application package) with Cargo.toml file if it doesn't exist

```rust
cargo init .
```

### Create a new library

Create a custom library package

```rust
cargo new --lib <LIBRARY_NAME>
```

```rust
cargo init --lib <LIBRARY_PATH>
```

### Compile the program

Compile the program and create an executable binary

```rust
cargo build
```

Run the binary

```rust
cargo run
```

## GPG signature

```bash
gpg --list-secret-keys --keyid-format LONG

gpg --full-generate-key

gpg --armor --export YOUR_GPG_KEY_ID

git config --global user.signingkey YOUR_GPG_KEY_ID

git commit -S -m "your commit message"


brew upgrade gnupg  # This has a make step which takes a while
brew link --overwrite gnupg
brew install pinentry-mac
echo "pinentry-program $(brew --prefix)/bin/pinentry-mac" >> ~/.gnupg/gpg-agent.conf
killall gpg-agent

echo "test" | gpg --clearsign

git config --global gpg.program gpg  # perhaps you had this already? On linux maybe gpg2
git config --global commit.gpgsign true

git log --show-signature -1
```

## Documentation

```bash
cargo doc
```

### Example

#### Comment function

```rust
/// # Examples
/// ```
/// let x = 5;
/// let y = 6;
/// assert_eq!(x + y, 11);
/// ```
```

#### Comment module

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain calculations more convenient.
//! 
//! # Examples
//! ```
//! let x = 5;
//! let y = 6;
//! assert_eq!(x + y, 11);
//! ```
```
