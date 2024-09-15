# Fast Image Scraper from Prnt.sc

## Overview

This Rust-based parser automates the process of downloading images from [prnt.sc](https://prnt.sc/) by generating random URL tokens, sending requests to the site, and scraping the image links. The images are then downloaded and saved to a local directory.

The scraper is optimized for asynchronous operation, supporting multiple concurrent requests and customizable process counts to improve efficiency.

## Features

- **Concurrent Requests**: Controls the number of simultaneous requests to prevent overloading.
- **Random Token Generation**: Automatically generates random tokens for URL requests.
- **Multiple User-Agents**: Randomly selects from a list of user-agents to mimic different browsers/devices.
- **Asynchronous Execution**: Utilizes `Tokio` for efficient, non-blocking requests.
- **Image Saving**: Automatically saves scraped images to the `output/` directory.

## Installation

### Prerequisites

- Rust (ensure you have it installed): [Install Rust](https://www.rust-lang.org/tools/install)

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/lnkssr/prnt.sc_parser.git
   cd print.sc_parser
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Running the Scraper

To start the scraper, run the following command:
```bash
cargo run --release [number_of_concurrent_processes]
```

For example, to run with 5 concurrent processes:
```bash
cargo run --release 5
```

- If you don’t specify a number, the default is 1 process.
  
### Output

- The scraper saves all images to the `output/` directory.
- Filenames are derived from the prnt.sc URL token, and images retain their original file extensions.

### Example

Running the scraper with 3 processes:
```bash
cargo run --release 3
```

Output:
```
[+] Image found for token https://prnt.sc/abc123. Downloading...
[+] No valid image found for token https://prnt.sc/xyz789.
[+] Image found for token https://prnt.sc/klm456. Downloading...
```

### Arguments

- `[number_of_concurrent_processes]` (optional): Number of concurrent processes scraping and downloading images.

### Custom User-Agent

By default, the scraper randomly selects one of several user-agent strings for each request to mimic different browsers or devices. The list is hardcoded, but you can modify it in the `USER_AGENTS` array if needed.

### Example of `USER_AGENTS`:
```rust
const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:90.0) Gecko/20100101 Firefox/90.0",
    "Mozilla/5.0 (Linux; Android 10; SM-G973U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.120 Mobile Safari/537.36 SamsungBrowser/14.0",
];
```

## Error Handling

- **Failed Requests**: If the scraper cannot retrieve an image or encounters an error, it logs an error message in the terminal, but continues to run.
- **Cloudflare Blocking**: Some requests may be blocked by Cloudflare protection. The scraper will log these events but continue attempting other URLs.

## Contributing

Feel free to fork the repository and submit pull requests. Contributions are welcome for:
- Improving error handling.
- Adding features like retries on failure or additional scraping logic.
- Enhancing efficiency in concurrent processing.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

### Notes for Developers

If you're interested in contributing or improving the scraper's logic:
- **Concurrency**: Managed via `tokio::sync::Semaphore`, which limits the number of active requests.
- **Random Token Generation**: Utilizes the `rand` crate to generate tokens of varying lengths (from 3 to 8 characters).

