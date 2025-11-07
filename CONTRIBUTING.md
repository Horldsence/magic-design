# Contributing to Magic Design

Thank you for your interest in contributing to Magic Design! This document provides guidelines and steps for contributing.

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/Horldsence/magic-design/issues)
2. If not, create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Screenshots if applicable
   - Your environment (OS, browser, versions)

### Suggesting Enhancements

1. Check existing issues and pull requests
2. Create an issue describing:
   - The enhancement goal
   - Why it would be useful
   - Possible implementation approach

### Pull Requests

1. Fork the repository
2. Create a new branch: `git checkout -b feature/your-feature-name`
3. Make your changes
4. Test your changes thoroughly
5. Commit with clear messages: `git commit -m "Add feature: description"`
6. Push to your fork: `git push origin feature/your-feature-name`
7. Open a Pull Request

## Development Setup

### Prerequisites

- Node.js 18+
- Rust (latest stable)
- Platform-specific dependencies (see README.md)

### Setup Steps

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/magic-design.git
cd magic-design

# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Code Style

#### TypeScript/JavaScript
- Use TypeScript for type safety
- Follow existing code style
- Use meaningful variable names
- Add comments for complex logic

#### Rust
- Follow Rust conventions
- Use `cargo fmt` to format code
- Use `cargo clippy` for linting
- Write clear error messages

#### CSS
- Use meaningful class names
- Follow BEM naming convention when appropriate
- Prefer CSS variables for theming
- Support both light and dark modes

### Testing

Before submitting a PR:

1. Test the frontend builds: `npm run build`
2. Test Rust code compiles: `cd src-tauri && cargo check`
3. Manually test your changes
4. Verify all existing functionality still works

### Commit Messages

Use clear, descriptive commit messages:

```
Add feature: extract border-radius from styles
Fix: handle URLs with query parameters
Update: improve error messages for network failures
Docs: add examples for color extraction
```

## Areas for Contribution

### High Priority
- [ ] Add support for external CSS files
- [ ] Improve color extraction accuracy
- [ ] Add tests for style extraction
- [ ] Support for CSS variables extraction
- [ ] Better component detection

### Good First Issues
- [ ] Improve error messages
- [ ] Add loading animations
- [ ] Enhance UI/UX
- [ ] Add keyboard shortcuts
- [ ] Improve documentation

### Advanced
- [ ] Add support for CSS-in-JS detection
- [ ] Implement style comparison between URLs
- [ ] Add export to other formats (JSON, YAML)
- [ ] Create browser extension version
- [ ] Add AI-powered style suggestions

## Architecture Overview

### Frontend (TypeScript)
- `src/main.ts`: Main application logic
- `src/styles.css`: Application styling
- `index.html`: HTML structure

### Backend (Rust)
- `src-tauri/src/lib.rs`: Tauri commands and app setup
- `src-tauri/src/style_extractor.rs`: Core extraction logic
- `src-tauri/src/main.rs`: Application entry point

### Key Functions

#### Frontend
- `extractStyles()`: Calls Rust backend via IPC
- `showStatus()`: Updates UI status messages
- `copyToClipboard()`: Copies markdown to clipboard
- `saveToFile()`: Saves markdown to file

#### Backend
- `fetch_website_blocking()`: Fetches HTML from URL
- `extract_styles()`: Main extraction logic
- `extract_colors()`: Color extraction
- `extract_typography()`: Font extraction
- `extract_spacing()`: Spacing extraction
- `extract_components()`: Component detection
- `generate_markdown()`: Creates markdown output

## Questions?

Feel free to:
- Open an issue for questions
- Join discussions
- Ask for help on your PR

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.
