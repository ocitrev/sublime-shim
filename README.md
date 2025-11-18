# Sublime Text Launcher

A lightweight Rust-based command-line wrapper for launching Sublime Text on Windows.

## Overview

This utility provides a convenient way to open files and directories in Sublime Text from the command line. It handles path arguments intelligently, making it particularly useful as a shorthand command or replacement for the full `subl.exe` invocation.

## Features

- **Smart Path Handling**: Automatically combines space-separated arguments into a single path if the combined result exists as a file or directory
- **Argument Passthrough**: Forwards all command-line arguments directly to Sublime Text
- **Exit Code Propagation**: Returns the same exit code as Sublime Text for proper shell integration
- **Error Handling**: Provides clear error messages if Sublime Text cannot be launched

## Usage

```bash
# Open a file
sublime_launcher myfile.txt

# Open a directory
sublime_launcher "C:\Projects\MyProject"

# Open a path with spaces (arguments are automatically combined if the path exists)
sublime_launcher C:\My Documents\file.txt

# Pass multiple files
sublime_launcher file1.txt file2.txt

# Use any Sublime Text command-line options
sublime_launcher --new-window myfile.txt
```

## Installation

1. Ensure you have [Rust](https://rustup.rs/) installed
2. Clone this repository
3. Install the binary to your Cargo bin directory:
   ```bash
   cargo install --path .
   ```
4. The executable will be installed to `~/.cargo/bin/` (which should be in your PATH)

## Configuration

The Sublime Text installation path is hardcoded to the default Windows location:
```
C:\Program Files\Sublime Text\subl.exe
```

If your installation is in a different location, modify the `SUBL` constant in `main.rs` before installing.

## How It Works

1. Collects all command-line arguments
2. Checks if joining all arguments creates a valid filesystem path
3. If a valid path is found, treats it as a single argument (useful for paths with spaces)
4. Otherwise, passes all arguments individually to Sublime Text
5. Launches Sublime Text with the processed arguments
6. Exits with the same status code as Sublime Text

## License

MIT License - see [LICENSE](LICENSE) file for details.
