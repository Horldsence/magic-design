# Features Overview

## Core Capabilities

### 1. Website Style Extraction
The application fetches any publicly accessible website and extracts comprehensive style information including:

- **Color Palette**: All colors used in the site (hex, rgb, rgba, hsl, hsla)
- **Typography System**: Font families, sizes, weights, and line heights
- **Spacing Values**: Margins, paddings, and gaps with all CSS units
- **Component Styles**: Individual styling for common UI elements

### 2. Intelligent Parsing

```
User Input (URL) → Fetch HTML → Parse Document → Extract Styles → Generate Markdown
```

The extraction process:
1. **Fetches** the website using Rust's `reqwest` library
2. **Parses** HTML structure with the `scraper` crate
3. **Analyzes** inline styles and `<style>` tags
4. **Extracts** CSS properties using regex patterns
5. **Organizes** findings into logical categories
6. **Generates** clean, readable Markdown output

### 3. Markdown Output Format

The generated style guide follows a consistent, hierarchical structure:

```markdown
# Style Guide for [URL]
Generated on: [timestamp]

## Color Scheme
### Primary Colors
### Background Colors
### Text Colors
### Border Colors

## Typography
### Font Families
### Font Sizes
### Font Weights
### Line Heights

## Spacing System
### Margins
### Paddings
### Gaps

## Component Styles
### [Component Name]
- Property: Value
```

### 4. User-Friendly Interface

**Input Section:**
- Clean URL input field with validation
- Prominent "Extract Styles" button
- Real-time status updates

**Status Feedback:**
- Loading state with spinner animation
- Success messages with checkmarks
- Error messages with helpful details
- Auto-dismissing success notifications

**Output Display:**
- Syntax-highlighted Markdown preview
- Scrollable output area
- Copy to clipboard button
- Save to file button with file picker

### 5. Export Options

**Copy to Clipboard:**
- One-click copy of entire style guide
- Browser clipboard API integration
- Success confirmation

**Save to File:**
- Native file dialog
- Default filename: `style-guide.md`
- Preserves formatting

## Technical Features

### Frontend (TypeScript)
- Type-safe code with TypeScript
- Modern ES modules
- Vite for fast development and builds
- Responsive CSS design
- Dark mode support
- Gradient-based UI elements

### Backend (Rust)
- Memory-safe code
- Fast HTML parsing
- Robust error handling
- Efficient regex pattern matching
- Structured data extraction
- Clean markdown generation

### Platform Support
- **macOS**: Native builds with .app bundle
- **Windows**: Native .exe with installer
- **Linux**: AppImage, .deb packages

### Performance
- Asynchronous network requests
- Efficient HTML parsing
- Minimal memory footprint
- Fast style extraction
- Quick markdown generation

## Use Cases

### For Designers
- Quickly analyze competitor designs
- Extract color palettes from existing sites
- Document design systems
- Create style references

### For Developers
- Generate CSS documentation
- Audit website styles
- Create migration guides
- Document legacy applications

### For Product Managers
- Compare design consistency
- Create design specifications
- Share visual guidelines
- Document brand standards

## Future Enhancements (Potential)

- [ ] Support for external CSS file fetching
- [ ] CSS variable extraction
- [ ] Comparison mode for multiple URLs
- [ ] Export to JSON/YAML formats
- [ ] Browser extension version
- [ ] Theme detection and categorization
- [ ] AI-powered design insights
- [ ] Image asset extraction
- [ ] Animation/transition detection
- [ ] Accessibility analysis
