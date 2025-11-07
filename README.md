# Magic Design - Website Style Extractor

A powerful Tauri desktop application that automatically extracts and analyzes HTML/CSS styles from any website, generating a comprehensive design guide in Markdown format.

## Features

- 🎨 **Automatic Style Extraction**: Parses HTML and CSS to extract design elements
- 🌈 **Color Scheme Analysis**: Identifies primary, background, text, and border colors
- 📝 **Typography Details**: Extracts font families, sizes, weights, and line heights
- 📏 **Spacing System**: Captures margins, paddings, and gaps
- 🧩 **Component Styles**: Analyzes common UI components (buttons, links, forms, etc.)
- 📄 **Markdown Output**: Generates clean, organized style guides in Markdown format
- 💾 **Save & Copy**: Export to file or copy to clipboard

## Tech Stack

- **Frontend**: TypeScript, HTML, CSS, Vite
- **Backend**: Rust
- **Framework**: Tauri
- **Libraries**:
  - `scraper`: HTML parsing
  - `reqwest`: HTTP client for fetching websites
  - `regex`: Pattern matching for style extraction
  - `chrono`: Timestamp generation

## Prerequisites

Before building the application, ensure you have the following installed:

### Common Requirements
- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

### Platform-Specific Requirements

#### Linux
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

#### macOS
```bash
xcode-select --install
```

#### Windows
- Microsoft Visual Studio C++ Build Tools
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/Horldsence/magic-design.git
cd magic-design
```

2. Install dependencies:
```bash
npm install
```

3. Build the application:
```bash
npm run tauri build
```

## Development

Run the application in development mode:

```bash
npm run tauri dev
```

This will start the Vite dev server and launch the Tauri application with hot-reload enabled.

## Usage

1. Launch the Magic Design application
2. Enter a website URL (e.g., `https://example.com`)
3. Click "Extract Styles" button
4. Wait for the analysis to complete
5. View the generated style guide in the output panel
6. Use "Copy Markdown" to copy to clipboard or "Save to File" to export

## Output Format

The generated style guide includes:

### Color Scheme
- Primary colors
- Background colors
- Text colors
- Border colors

### Typography
- Font families used
- Font sizes
- Font weights
- Line heights

### Spacing System
- Margins
- Paddings
- Gaps

### Component Styles
- Buttons
- Links
- Headings
- Forms
- Navigation elements
- And more...

## Example Output

```markdown
# Style Guide for https://example.com

Generated on: 2025-11-07 12:00:00

## Color Scheme

### Primary Colors
- `#667eea`
- `#764ba2`
- `#f6f6f6`

### Typography
- Font Family: `Inter, Arial, sans-serif`
- Font Sizes: `16px`, `1.5rem`, `2rem`
...
```

## Project Structure

```
magic-design/
├── src/                    # Frontend source code
│   ├── main.ts            # TypeScript entry point
│   ├── styles.css         # Application styles
│   └── assets/            # Static assets
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs        # Application entry
│   │   ├── lib.rs         # Tauri commands
│   │   └── style_extractor.rs  # Style extraction logic
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── index.html             # HTML entry point
├── package.json           # Node.js dependencies
└── README.md             # This file
```

## Architecture

### Frontend (TypeScript)
- Handles user input and UI interactions
- Calls Rust backend via Tauri IPC
- Displays extracted styles in a formatted view
- Provides clipboard and file save functionality

### Backend (Rust)
- Fetches website HTML using `reqwest`
- Parses HTML structure with `scraper`
- Extracts CSS properties using regex patterns
- Generates Markdown output
- Returns data to frontend via Tauri commands

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app/)
- UI inspired by modern design tools
- Thanks to the Rust and TypeScript communities
