# terminal-dictionary (td)

[![GitHub](https://img.shields.io/badge/GitHub-Profile-blue?logo=github)](https://github.com/famedaxolotl) [![crates.io](https://img.shields.io/crates/v/terminal-dictionary.svg)](https://crates.io/crates/terminal-dictionary) [![Total Downloads](https://img.shields.io/crates/d/terminal-dictionary.svg)](https://crates.io/crates/terminal-dictionary) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This project uses the [freeDictionaryAPI](https://github.com/meetDeveloper/freeDictionaryAPI). Please support its author if you can.

terminal-dictionary is a simple dictionary and thesaurus tool written in Rust for the command-line. See Installation and Usage for more.

## Installation

Use any of the following methods.

NOTE: The binary name is `td`, which is what you will use to execute it.

### 1. Downloading binary

- You can download the binary from the [releases page.](https://github.com/famedaxolotl/terminal-dictionary/releases)

- Move the binary `td` to `/usr/local/bin` with

```bash
sudo mv /Downloads/td /usr/local/bin
```

### 2. Using Cargo

If you are a Rust programmer and have Cargo installed, simply run the following:

```bash
cargo install terminal-dictionary
```

This will install the binary from crates.io

### 3. Building from GitHub

If you have Cargo installed, run the following commands

```bash
git clone https://github.com/famedaxolotl/terminal-dictionary

cd terminal-dictionary

cargo build --release
```

Now, run with `cargo run` or move the binary to `/usr/bin` with:

`sudo mv target/release/td /usr/local/bin`

Alternatively for (1) and (3), you can add the Downloads folder to `$PATH` by editing the `./bashrc` file.

## Usage

### 1.Dictionary

Use the `def` command to search dictionary:

```bash
td def hobby
```
Output:
```
HOBBY-----------NOUN
An activity that one enjoys doing in one's spare time.
Example: I like to collect stamps from different countries as a hobby.

An extinct breed of horse native to the British Isles, also known as the Irish Hobby
Example: N/A
```

### 2. Thesaurus

Use the `thes` command to search synonyms and antonyms

```sh
td thes gregarious
```

Output:

```
Synonyms and antonyms for GREGARIOUS
Synonyms: outgoing, sociable, social, 
Antonyms: ungregarious, nongregarious,
```