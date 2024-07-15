# HowLongToBeat CLI

A command-line interface (CLI) tool built with Rust for searching game information from the HowLongToBeat database.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Features

- Search for games by name, platform, year, and more.
- Sort results by various categories such as popularity, rating, and release date.
- Filter results to include or exclude DLCs.
- Display results in a colorized format for better readability.
- Option to output raw JSON for further processing.

## Installation

Ensure you have Rust and Cargo installed on your system. You can install Rust and Cargo using [rustup](https://rustup.rs/).

Clone the repository:

```bash
git clone https://github.com/hypeedev/howlongtobeat-cli.git
cd howlongtobeat-cli
```

Build the project:

```bash
cargo build --release
```

The executable will be located in ./target/release/.

## Usage

To use the HowLongToBeat CLI, run the executable from the command line. Here are some example commands:

```bash
./howlongtobeat-cli "The Witcher 3"
./howlongtobeat-cli Halo --platform "Xbox One" --size 10 --color never
./howlongtobeat-cli "Final Fantasy" --sort rating --json
```

For more information on the available options, run

```bash
./howlongtobeat-cli --help
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request with your improvements.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

<br>

**Disclaimer:** This software is an independent project and is not affiliated with HowLongToBeat in any way.
