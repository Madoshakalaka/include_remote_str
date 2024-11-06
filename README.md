# include_remote_str!()

A Rust procedural macro that allows you to include remote text content at compile time, with built-in caching support.

## Features

- 🔄 Fetch remote text content during compilation
- 📦 Automatic caching of downloaded content
- 🛠️ Workspace-aware cache storage

## Usage

```rust
use include_remote_str::include_remote_str;

// The content will be fetched at compile time and included as a string literal
const REMOTE_TEXT: &str = include_remote_str!("https://example.com/some-text-file.txt");
```

## How it Works

The macro fetches the content from the specified URL during compilation and generates a string literal containing the downloaded text. The content is cached in the workspace root directory under `include-remote-str-cache` to avoid unnecessary downloads in subsequent compilations.

## Safety and Security

Please be aware that this macro fetches content at compile time from remote URLs. Make sure you trust the sources you're including content from, as they will be part of your compiled binary.
