# QPass

QPass is a simple command-line utility written in Rust for generating random passwords with an optionally specified length, or for generating a base64-encoded password from a given string.

## Features

- Generate a random password of a specified length.
- Generate a base64-encoded password from a given string.

## Usage

You can use QPass in two ways:

1. Generate a random password of a specified length:

```sh
qpass --length 20
```

This will generate a random password of 20 characters.

2. Generate a base64-encoded password from a given string:

```sh
qpass --from "your string"
```

This will output the base64-encoded version of "your string".

## Installation

To install QPass, you need to have Rust installed on your machine. If you don't have Rust installed, you can install it from the [official website](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone this repository and build the project:

```sh
git clone https://github.com/mdmmn378/qpass.git
cd qpass
cargo install --path .
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
