<div align="center">

  <img height="170x" src="./assets/icon.png" />

  <h1>Code Scraper for AI</h1>

  <p>
    <strong>Extract code from a source directory for AI analysis</strong>
  </p>

</div>

## Why

- Recursively processes files in a specified source directory
- Respects `.gitignore` file to exclude unwanted files and directories
- Supports custom ignore files for additional exclusion patterns
- Generates a single output file containing the extracted code
- Optimized for code analysis by AI systems
- Provides logging capabilities for better visibility and debugging

## Installation

### Using Cargo

```bash
cargo install --git https://github.com/dfrnoch/code_scraper
```

### Downloading from GitHub

Every new version will be automatically compiled and available for download in the [releases page](https://github.com/dfrnoch/code_scraper/releases).

## Usage

```bash
code_scraper <source_folder> [custom_ignore_file] [--log]
```

- `<source_folder>`: The path to the source directory containing the code files to be extracted.
- `[custom_ignore_file]` (optional): The path to a custom ignore file specifying additional patterns to exclude from extraction. If not provided, only the `.gitignore` file in the source directory will be used.
- `[--log]` (optional): Enable logging for detailed output and debugging information.

## Configuration

Code Scraper for AI provides various configuration options to customize its behavior:

- `source_folder`: Specify the path to the source directory containing the code files to be extracted.
- `custom_ignore_file`: Provide a custom ignore file with additional patterns to exclude from extraction.
- `log`: Enable logging for detailed output and debugging information.

## Output

Code Scraper for AI generates an output file named `output.txt` in the current directory. The file contains the extracted code from the processed files, with each file's content separated by a header indicating the file path.

## Create a Bug Report

If you encounter an error message or run into an issue, please [create an issue](https://github.com/dfrnoch/code_scraper/issues). Your effort is valued and will help improve the tool for other users.

## Submit a Feature Request

If you have an idea for a new feature or enhancement, please [submit a feature request](https://github.com/dfrnoch/code_scraper/issues).

## Contributing

Code Scraper for AI is an open-source project. Whether you are helping to fix bugs, proposing new features, or improving documentation, your contributions are highly appreciated. Please feel free to submit pull requests or open issues to contribute to the project.

## License

Code Scraper for AI is released under the [MIT License](LICENSE).