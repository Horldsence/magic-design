# Architecture Documentation

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Magic Design App                         │
│                      (Tauri Desktop)                         │
└─────────────────────────────────────────────────────────────┘
                           │
        ┌──────────────────┴──────────────────┐
        │                                     │
        ▼                                     ▼
┌───────────────┐                    ┌───────────────┐
│   Frontend    │                    │   Backend     │
│  (TypeScript) │◄──── IPC ──────────┤    (Rust)     │
│               │      Bridge        │               │
└───────────────┘                    └───────────────┘
        │                                     │
        │                                     │
        ▼                                     ▼
┌───────────────┐                    ┌───────────────┐
│  UI Layer     │                    │ Core Logic    │
│ - Input Form  │                    │ - HTTP Client │
│ - Display     │                    │ - HTML Parser │
│ - Buttons     │                    │ - Extractor   │
│ - Status      │                    │ - Generator   │
└───────────────┘                    └───────────────┘
```

## Component Breakdown

### Frontend Components

#### 1. UI Layer (`src/main.ts`, `index.html`, `src/styles.css`)
**Responsibilities:**
- Render user interface
- Handle user input
- Display results
- Manage application state
- Show status messages

**Key Functions:**
- `extractStyles()`: Initiates extraction via IPC
- `showStatus()`: Updates UI status
- `copyToClipboard()`: Clipboard operations
- `saveToFile()`: File save operations

#### 2. Vite Build System
**Responsibilities:**
- Bundle frontend code
- Optimize for production
- Hot module replacement (dev mode)
- TypeScript compilation

### Backend Components

#### 1. Tauri Runtime (`src-tauri/src/lib.rs`, `src-tauri/src/main.rs`)
**Responsibilities:**
- Initialize application
- Setup IPC bridge
- Register commands
- Manage plugins

**Key Functions:**
- `run()`: Application entry point
- `extract_website_styles()`: Main Tauri command

#### 2. Style Extractor Module (`src-tauri/src/style_extractor.rs`)
**Responsibilities:**
- Fetch website HTML
- Parse HTML structure
- Extract CSS properties
- Generate markdown output

**Key Functions:**
- `fetch_website_blocking()`: HTTP request handler
- `extract_styles()`: Main extraction orchestrator
- `extract_colors()`: Color extraction
- `extract_typography()`: Font/text extraction
- `extract_spacing()`: Spacing extraction
- `extract_components()`: Component detection
- `generate_markdown()`: Output formatter

## Data Flow

### Extraction Flow

```
User Input (URL)
    │
    ▼
Frontend Validation
    │
    ▼
Tauri IPC Call
    │
    ▼
Rust Backend
    │
    ├─► Fetch HTML (reqwest)
    │       │
    │       ▼
    │   Parse Document (scraper)
    │       │
    │       ▼
    │   Extract Inline Styles
    │       │
    │       ▼
    │   Extract Style Tags
    │       │
    │       ▼
    │   Pattern Matching (regex)
    │       │
    │       ▼
    │   Organize Data
    │       │
    │       ▼
    │   Generate Markdown
    │
    ▼
Return to Frontend
    │
    ▼
Display Results
    │
    ├─► Copy to Clipboard
    └─► Save to File
```

### Data Structures

```rust
StyleGuide {
    url: String,
    colors: ColorScheme {
        primary_colors: Vec<String>,
        background_colors: Vec<String>,
        text_colors: Vec<String>,
        border_colors: Vec<String>
    },
    typography: Typography {
        font_families: Vec<String>,
        font_sizes: Vec<String>,
        font_weights: Vec<String>,
        line_heights: Vec<String>
    },
    spacing: SpacingSystem {
        margins: Vec<String>,
        paddings: Vec<String>,
        gaps: Vec<String>
    },
    components: Vec<ComponentStyle> {
        name: String,
        styles: HashMap<String, String>
    }
}
```

## Technology Stack

### Frontend Technologies
- **TypeScript**: Type-safe JavaScript
- **HTML5**: Semantic markup
- **CSS3**: Modern styling with gradients, flexbox
- **Vite**: Build tool and dev server
- **Tauri API**: Bridge to backend

### Backend Technologies
- **Rust**: Systems programming language
- **Tauri**: Desktop app framework
- **reqwest**: HTTP client
- **scraper**: HTML parsing
- **regex**: Pattern matching
- **serde**: Serialization/deserialization
- **chrono**: Date/time handling

### Build & Distribution
- **npm**: Package management
- **Cargo**: Rust package management
- **Vite**: Frontend bundling
- **Tauri CLI**: App compilation
- **GitHub Actions**: CI/CD pipeline

## Plugin Architecture

```
Tauri Core
    │
    ├─► tauri-plugin-opener
    │   (Open URLs in browser)
    │
    ├─► tauri-plugin-dialog
    │   (Native file dialogs)
    │
    └─► tauri-plugin-fs
        (File system operations)
```

## Security Considerations

### Frontend Security
- Input validation for URLs
- XSS prevention through proper escaping
- Content Security Policy (CSP)

### Backend Security
- Safe HTTP requests with timeout
- HTML parsing in sandboxed environment
- No arbitrary code execution
- Read-only file system access for extraction

### IPC Security
- Typed command interfaces
- Permission-based plugin access
- Capability-based security model

## Performance Optimization

### Frontend
- Minimal dependencies
- Efficient DOM updates
- Debounced user input
- Lazy loading where applicable

### Backend
- Efficient regex compilation
- HashSet for deduplication
- Blocking HTTP for simplicity
- Bounded result sets (limits on extracted items)

### Build Optimization
- Tree shaking (Vite)
- Minification
- Code splitting
- Compression (gzip)

## Error Handling

### Frontend
```typescript
try {
    const result = await invoke("extract_website_styles", { url });
    // Success handling
} catch (error) {
    // Error display
}
```

### Backend
```rust
pub fn extract_website_styles(url: String) -> Result<String, String> {
    // Uses Result type for error propagation
    // Converts errors to user-friendly messages
}
```

## Development Workflow

```
Code Changes
    │
    ▼
Frontend: npm run dev
Backend: cargo check
    │
    ▼
Testing
    │
    ▼
Build: npm run build
    │
    ▼
Tauri Build: npm run tauri build
    │
    ▼
Distribution
```
