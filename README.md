# Sublime Shims

Lightweight Rust-based command-line wrappers for launching Sublime Text and Sublime Merge on Windows.

## Overview

This project provides two convenient shims for running Sublime applications from the command line:
- **subl**: Wrapper for Sublime Text with smart path handling
- **smerge**: Wrapper for Sublime Merge

Both utilities handle arguments intelligently and provide proper error handling and exit code propagation.

## Features

### subl (Sublime Text)
- **Smart Path Handling**: Automatically combines space-separated arguments into a single path if the combined result exists as a file or directory
- **Argument Passthrough**: Forwards all command-line arguments directly to Sublime Text
- **Exit Code Propagation**: Returns the same exit code as Sublime Text for proper shell integration
- **Error Handling**: Provides clear error messages if Sublime Text cannot be launched

### smerge (Sublime Merge)
- **Direct Passthrough**: Forwards all arguments directly to Sublime Merge
- **Exit Code Propagation**: Returns the same exit code as Sublime Merge
- **Error Handling**: Provides clear error messages if Sublime Merge cannot be launched

## Usage

### Sublime Text (subl)
```cmd
:: Open a file
subl myfile.txt

:: Open a directory
subl .

:: Open a path with spaces (arguments are automatically combined if the path exists)
subl C:\My Documents\file.txt

:: Pass multiple files
subl file1.txt file2.txt

:: Use any Sublime Text command-line options
subl --new-window myfile.txt
```

### Sublime Merge (smerge)
```cmd
:: Open current directory
smerge .

:: Use any Sublime Merge command-line options
smerge --help
```

## Installation

1. Ensure you have [Rust](https://rustup.rs/) installed
2. Clone this repository
3. Install both binaries to your Cargo bin directory:
   ```cmd
   .\install
   ```
4. The executables will be installed to `~/.cargo/bin/` (which should be in your PATH)

## Configuration

The application paths are hardcoded to the default Windows locations:
- Sublime Text: `C:\Program Files\Sublime Text\subl.exe`
- Sublime Merge: `C:\Program Files\Sublime Merge\smerge.exe`

If your installations are in different locations, modify the constants in `src/subl.rs` and `src/smerge.rs` before installing.

## How It Works

### subl
1. Collects all command-line arguments
2. Checks if joining all arguments creates a valid filesystem path
3. If a valid path is found, treats it as a single argument (useful for paths with spaces)
4. Otherwise, passes all arguments individually to Sublime Text
5. Launches Sublime Text with the processed arguments
6. Exits with the same status code as Sublime Text

### smerge
1. Collects all command-line arguments
2. Passes them directly to Sublime Merge
3. Exits with the same status code as Sublime Merge

## License

MIT License - see [LICENSE](LICENSE) file for details.
